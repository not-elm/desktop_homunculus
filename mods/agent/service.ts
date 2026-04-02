import { z } from "zod";
import { Vrm, preferences, signals, stt } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { KeyboardHookService, waitForComboRelease, isComboHeld } from "./lib/keyboard-hook.ts";
import { resolvePttKeycodes, type ResolvedPttKey } from "./lib/key-mapping.ts";
import { ClaudeAgentExecuter } from "./lib/claude-agent-executer.ts";
import { CodexAppServerExecuter } from "./lib/codex-appserver-executer.ts";
import { CodexAppServerProcess } from "./lib/codex-appserver-process.ts";
import type { AIAgentExecuter } from "./lib/ai-agent-executer.ts";
import { AsyncQueue, Deferred } from "./lib/async-queue.ts";
import type { AgentEvent, AgentResponse } from "./lib/ai-agent-executer.ts";
import {
  type AgentSettings,
  type AgentStatus,
  type Persona,
  DEFAULT_SETTINGS,
} from "./lib/types.ts";
import type { WorktreeContext } from "./lib/prompt.ts";
import { WorktreeManager } from "./lib/worktree-manager.ts";
import { isGitRepo, currentBranch, listBranches } from "./lib/git.ts";
import { sanitizeForTts } from "./lib/tts.ts";
import { mkdirSync } from "node:fs";
import { homedir } from "node:os";
import path from "node:path";

const keyboardHook = new KeyboardHookService();

const activeSessions = new Map<string, AbortController>();
const pendingApprovals = new Map<string, Deferred<{ approved: boolean; message?: string; decision?: string | Record<string, unknown> }>>();
const pendingQuestions = new Map<string, Deferred<Record<string, string>>>();
const pendingInterrupts = new Map<string, Deferred<void>>();
const textQueues = new Map<string, AsyncQueue<string>>();

let appServerProcess: CodexAppServerProcess | null = null;

function getAppServerProcess(): CodexAppServerProcess {
  if (!appServerProcess) {
    appServerProcess = new CodexAppServerProcess();
  }
  return appServerProcess;
}

let currentApiKey: string | null = null;

async function loadApiKey(): Promise<string> {
  const apiKey = await preferences.load<string>("agent::api-key");
  if (!apiKey)
    throw new Error(
      "API key not configured. Set 'agent::api-key' in preferences.",
    );
  return apiKey;
}

async function loadCharacterSettings(
  characterId: string,
): Promise<AgentSettings> {
  const saved = await preferences.load<Record<string, unknown>>("agent::" + characterId);
  if (!saved) return { ...DEFAULT_SETTINGS };

  // Migrate workingDirectories → workspaces
  if ("workingDirectories" in saved && !("workspaces" in saved)) {
    const wd = saved.workingDirectories as { paths: string[]; default: number };
    saved.workspaces = {
      paths: wd.paths,
      selection: { workspaceIndex: wd.default, worktreeName: null },
    };
    delete saved.workingDirectories;
    await preferences.save("agent::" + characterId, { ...DEFAULT_SETTINGS, ...saved });
  }

  return { ...DEFAULT_SETTINGS, ...(saved as Partial<AgentSettings>) };
}

function speakText(characterId: string, text: string): void {
  const { sentences, log } = sanitizeForTts(text);
  if (sentences.length === 0) return;
  if (log.length > 0) {
    emitLog(characterId, "tts-sanitize", log.join("; "));
  }
  rpc
    .call({
      modName: "@hmcs/voicevox",
      method: "speak",
      body: { name: characterId, text: sentences },
    })
    .catch(() => emitLog(characterId, "warning", "TTS unavailable"));
}

async function startKeyboardHook(): Promise<void> {
  const started = keyboardHook.start();
  if (!started) {
    console.warn("[agent] Keyboard hook failed to start.");
  }
}

async function startSession(characterId: string): Promise<void> {
  const settings = await loadCharacterSettings(characterId);
  assertCanStartSession(characterId, settings);

  const resolvedKey = resolvePttKeyOptional(settings);
  const persona = await loadPersona(characterId);
  const workDir = resolveWorkingDirectory(characterId, settings);
  mkdirSync(workDir, { recursive: true });

  const selection = settings.workspaces.selection;
  if (selection.worktreeName) {
    await signals.send("agent:worktree", {
      characterId,
      state: "created",
      worktreeName: selection.worktreeName,
      workspacePath: settings.workspaces.paths[selection.workspaceIndex],
    });
  }

  const worktreeCtx = buildWorktreeContext(settings, workDir);
  const executer = createExecuter(settings, persona, currentApiKey, workDir, worktreeCtx);

  const sessionAbort = new AbortController();
  activeSessions.set(characterId, sessionAbort);
  textQueues.set(characterId, new AsyncQueue<string>());

  launchSessionLoop(characterId, executer, sessionAbort, settings, resolvedKey);
}

function assertCanStartSession(
  characterId: string,
  settings: AgentSettings,
): void {
  if (settings.executor === "sdk" && !currentApiKey) {
    throw new Error(
      "API key not configured. Open Agent Settings to set your Anthropic API key.",
    );
  }
  if (activeSessions.has(characterId)) {
    throw new Error(`Session already active for "${characterId}".`);
  }
}

function launchSessionLoop(
  characterId: string,
  executer: AIAgentExecuter,
  sessionAbort: AbortController,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
): void {
  runSession(characterId, executer, sessionAbort, settings, resolvedKey).catch(
    (err) => handleSessionCrash(characterId, err, settings),
  );
}

function handleSessionCrash(
  characterId: string,
  err: unknown,
  settings: AgentSettings,
): void {
  if (!isAbortError(err)) {
    console.error(`[agent] Session error for ${characterId}:`, err);
    emitLog(characterId, "error", extractErrorMessage(err));
  }
  const queue = textQueues.get(characterId);
  if (queue) {
    queue.rejectAll(new DOMException("Session crashed", "AbortError"));
    textQueues.delete(characterId);
  }
  activeSessions.delete(characterId);
  emitStatus(characterId, "idle", isAbortError(err) ? "stopped" : "crashed");
}

function resolvePttKeyOptional(settings: AgentSettings): ResolvedPttKey | null {
  if (!settings.pttKey) return null;
  return resolvePttKeycodes(settings.pttKey) ?? null;
}

async function loadPersona(characterId: string): Promise<Persona> {
  const vrm = await Vrm.findByName(characterId);
  const sdkPersona = await vrm.persona();
  return {
    name: sdkPersona.displayName || characterId,
    age: sdkPersona.age ?? null,
    gender: sdkPersona.gender ?? "unknown",
    firstPersonPronoun: sdkPersona.firstPersonPronoun ?? null,
    profile: sdkPersona.profile ?? "",
    ocean: sdkPersona.ocean ?? {},
  };
}

function resolveWorkingDirectory(
  characterId: string,
  settings: AgentSettings,
): string {
  const { paths, selection } = settings.workspaces;
  const basePath = paths[selection.workspaceIndex];
  if (!basePath) return path.join(homedir(), ".homunculus", "agents", characterId);
  if (selection.worktreeName) {
    return path.join(basePath, ".hmcs/worktrees", selection.worktreeName);
  }
  return basePath;
}

function buildWorktreeContext(
  settings: AgentSettings,
  workDir: string,
): WorktreeContext | undefined {
  const { selection } = settings.workspaces;
  if (!selection.worktreeName) return undefined;
  return {
    worktreeName: selection.worktreeName,
    baseBranch: "main",
    worktreePath: workDir,
  };
}

function createExecuter(
  settings: AgentSettings,
  persona: Persona,
  apiKey: string | null,
  workDir: string,
  worktree?: WorktreeContext,
): AIAgentExecuter {
  switch (settings.executor) {
    case "codex":
      return new CodexAppServerExecuter(persona, settings, workDir, getAppServerProcess(), worktree);
    case "sdk":
      return new ClaudeAgentExecuter(persona, settings, apiKey!, workDir, worktree);
    default:
      throw new Error(`Executor "${settings.executor}" is not yet implemented.`);
  }
}

async function stopSession(characterId: string): Promise<void> {
  const controller = activeSessions.get(characterId);
  if (!controller) return;
  controller.abort();
  const queue = textQueues.get(characterId);
  if (queue) {
    queue.rejectAll(new DOMException("Session stopped", "AbortError"));
    textQueues.delete(characterId);
  }
  activeSessions.delete(characterId);
  emitStatus(characterId, "idle", "stopped");

  if (appServerProcess && appServerProcess.refCount === 0) {
    appServerProcess.shutdown();
    appServerProcess = null;
  }
}

async function interruptSession(characterId: string): Promise<void> {
  const deferred = pendingInterrupts.get(characterId);
  if (deferred) {
    deferred.resolve();
  }
}

const SESSION_PREF_PREFIX = "agent::session::";

async function loadSavedSession(
  characterId: string,
  executor: AgentSettings["executor"],
): Promise<string | null> {
  return (
    (await preferences.load<string>(
      `${SESSION_PREF_PREFIX}${executor}::${characterId}`,
    )) ?? null
  );
}

function saveSession(
  characterId: string,
  executor: AgentSettings["executor"],
  sessionId: string | null,
): void {
  preferences.save(
    `${SESSION_PREF_PREFIX}${executor}::${characterId}`,
    sessionId,
  );
}

interface UserInput {
  text: string;
  source: "voice" | "text";
}

async function runSession(
  characterId: string,
  executer: AIAgentExecuter,
  sessionAbort: AbortController,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
): Promise<void> {
  let sessionId = await loadSavedSession(characterId, settings.executor);
  const signal = sessionAbort.signal;

  try {
    while (!signal.aborted) {
      emitStatus(characterId, "listening");
      const input = await waitForUserInput(characterId, resolvedKey, signal);
      if (input === null) continue;

      emitUserLog(characterId, input.text, input.source);
      const result = await executeOneRound(
        characterId,
        executer,
        input.text,
        sessionId,
        settings,
        resolvedKey,
        signal,
      );
      sessionId = result.sessionId;
    }
  } catch (e) {
    if (!isAbortError(e)) throw e;
  } finally {
    activeSessions.delete(characterId);
    emitStatus(characterId, "idle", "session-ended");
  }
}

interface RoundResult {
  sessionId: string | null;
}

async function executeOneRound(
  characterId: string,
  executer: AIAgentExecuter,
  text: string,
  sessionId: string | null,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
  sessionSignal: AbortSignal,
): Promise<RoundResult> {
  const interruptAbort = new AbortController();
  const executorGen = executer.execute(text, sessionId, interruptAbort.signal);
  const interruptPromise = waitForInterrupt(characterId, resolvedKey, sessionSignal);

  try {
    return await driveExecutor(
      characterId,
      executorGen,
      interruptPromise,
      interruptAbort,
      sessionId,
      settings,
      resolvedKey,
      sessionSignal,
    );
  } catch (e) {
    if (!isAbortError(e)) throw e;
    return { sessionId };
  }
}

async function driveExecutor(
  characterId: string,
  executorGen: AsyncGenerator<AgentEvent, void, AgentResponse | undefined>,
  interruptPromise: Promise<"ptt" | "ui">,
  interruptAbort: AbortController,
  sessionId: string | null,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
  sessionSignal: AbortSignal,
): Promise<RoundResult> {
  let lastSessionId = sessionId;
  let response: AgentResponse | undefined = undefined;

  while (true) {
    const raceResult = await raceInterrupt(executorGen.next(response), interruptPromise);

    if (raceResult.interrupted) {
      lastSessionId = await abortExecution(characterId, executorGen, interruptAbort, lastSessionId);

      if (raceResult.source === "ptt" && resolvedKey) {
        const voiceText = await recognizeWhileHeld(characterId, resolvedKey, sessionSignal);
        if (voiceText) {
          const queue = textQueues.get(characterId);
          if (queue) {
            queue.clear();
            queue.push(voiceText);
          }
        }
      }

      return { sessionId: lastSessionId };
    }
    if (raceResult.done) break;

    const event = raceResult.value as AgentEvent;
    response = await handleAgentEvent(characterId, event, settings, sessionSignal);
    if (event.type === "completed" || event.type === "error") {
      if (event.type === "completed") lastSessionId = event.sessionId;
      break;
    }
  }

  await executorGen.return(undefined);
  return { sessionId: lastSessionId };
}

async function abortExecution(
  characterId: string,
  executorGen: AsyncGenerator<AgentEvent, void, AgentResponse | undefined>,
  interruptAbort: AbortController,
  lastSessionId: string | null,
): Promise<string | null> {
  interruptAbort.abort();
  await executorGen.return(undefined);
  emitInterruptLog(characterId);
  return lastSessionId;
}

interface RaceResultDone {
  interrupted: false;
  done: true;
  value: undefined;
}

interface RaceResultEvent {
  interrupted: false;
  done: false;
  value: AgentEvent;
}

interface RaceResultInterrupted {
  interrupted: true;
  source: "ptt" | "ui";
}

type RaceResult = RaceResultDone | RaceResultEvent | RaceResultInterrupted;

async function raceInterrupt(
  nextResult: Promise<IteratorResult<AgentEvent, void>>,
  interruptPromise: Promise<"ptt" | "ui">,
): Promise<RaceResult> {
  const executorSide = nextResult.then((r): RaceResultDone | RaceResultEvent =>
    r.done
      ? { interrupted: false, done: true, value: undefined }
      : { interrupted: false, done: false, value: r.value as AgentEvent },
  );
  // Suppress unhandled rejection on the losing side of the race.
  executorSide.catch(() => {});

  return Promise.race([
    executorSide,
    interruptPromise.then(
      (source): RaceResultInterrupted => ({ interrupted: true, source }),
    ),
  ]);
}

async function handleAgentEvent(
  characterId: string,
  event: AgentEvent,
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse | undefined> {
  switch (event.type) {
    case "assistant_message":
      return handleAssistantMessage(characterId, event.text);
    case "tool_use":
      return handleToolUse(characterId, event.summary);
    case "permission_request":
      return await handlePermissionRequest(characterId, event, settings, signal);
    case "elicitation_request":
      return await handleElicitationRequest(characterId, event, signal);
    case "completed":
      return handleCompleted(characterId, event.sessionId, settings);
    case "error":
      return handleError(characterId, event.message, settings);
  }
}

function handleAssistantMessage(
  characterId: string,
  text: string,
): undefined {
  emitStatus(characterId, "thinking");
  emitLog(characterId, "assistant", text);
  speakText(characterId, text);
  return undefined;
}

function handleToolUse(characterId: string, summary: string): undefined {
  emitStatus(characterId, "executing");
  emitLog(characterId, "tool", summary);
  return undefined;
}

async function handlePermissionRequest(
  characterId: string,
  event: AgentEvent & { type: "permission_request" },
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse> {
  emitStatus(characterId, "waiting");
  return await resolvePermission(characterId, event, settings, signal);
}

async function handleElicitationRequest(
  characterId: string,
  event: AgentEvent & { type: "elicitation_request" },
  signal: AbortSignal,
): Promise<AgentResponse> {
  emitStatus(characterId, "waiting");
  return await resolveElicitation(characterId, event, signal);
}

function handleCompleted(
  characterId: string,
  sessionId: string,
  settings: AgentSettings,
): undefined {
  emitStatus(characterId, "idle", "turn-completed");
  saveSession(characterId, settings.executor, sessionId);
  return undefined;
}

function handleError(
  characterId: string,
  message: string,
  settings: AgentSettings,
): undefined {
  console.error(`[agent] ${characterId}: error — ${message}`);
  emitLog(characterId, "error", message);
  return undefined;
}

async function resolvePermission(
  characterId: string,
  event: AgentEvent & { type: "permission_request" },
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse> {
  const deferred = new Deferred<{ approved: boolean; message?: string; decision?: string | Record<string, unknown> }>();
  const permAbort = new AbortController();
  const combined = AbortSignal.any([signal, permAbort.signal]);

  pendingApprovals.set(event.requestId, deferred);

  const permissionPayload = {
    characterId,
    requestId: event.requestId,
    action: event.tool,
    target: JSON.stringify(event.input),
    availableDecisions: event.availableDecisions,
  };
  console.log(`[agent] resolvePermission: ${event.tool} (${event.requestId})`, JSON.stringify(permissionPayload, null, 2));

  signals.send("agent:permission", permissionPayload);

  const timer = setTimeout(
    () => deferred.resolve({ approved: false, message: "Permission request timed out" }),
    60_000,
  );

  const onAbort = () => deferred.reject(signal.reason);
  signal.addEventListener("abort", onAbort, { once: true });

  const resolvedKey = resolvePttKeycodes(settings.pttKey!);
  if (resolvedKey) {
    runVoiceApproval(characterId, resolvedKey, settings, combined, deferred);
  }

  try {
    const result = await deferred.promise;
    console.log(`[agent] permission result: approved=${result.approved}, message=${result.message}`);
    return { type: "permission", approved: result.approved, message: result.message, decision: result.decision };
  } finally {
    clearTimeout(timer);
    signal.removeEventListener("abort", onAbort);
    permAbort.abort();
    pendingApprovals.delete(event.requestId);
  }
}

function runVoiceApproval(
  characterId: string,
  resolvedKey: ResolvedPttKey,
  settings: AgentSettings,
  signal: AbortSignal,
  deferred: Deferred<{ approved: boolean; message?: string; decision?: string | Record<string, unknown> }>,
): void {
  (async () => {
    try {
      await waitForComboPress(resolvedKey, signal);
      const text = await recognizeWhileHeld(characterId, resolvedKey, signal);
      if (text === null) {
        deferred.resolve({ approved: false, message: "No speech detected" });
      } else {
        deferred.resolve(evaluateApprovalPhrase(text, settings));
      }
    } catch {
      // Voice approval failed (aborted or error) — UI/timeout will handle it
    }
  })();
}

function evaluateApprovalPhrase(
  text: string,
  settings: AgentSettings,
): { approved: boolean; message?: string } {
  const lower = text.toLowerCase();
  const isApproval = settings.approvalPhrases.some((p) =>
    lower.includes(p.toLowerCase()),
  );
  if (isApproval) return { approved: true };
  const isDenial = settings.denyPhrases.some((p) =>
    lower.includes(p.toLowerCase()),
  );
  if (isDenial) return { approved: false, message: `Denied: "${text}"` };
  return { approved: false, message: `Unrecognized: "${text}"` };
}


async function resolveElicitation(
  characterId: string,
  event: AgentEvent & { type: "elicitation_request" },
  signal: AbortSignal,
): Promise<AgentResponse> {
  signals.send("agent:question", {
    characterId,
    requestId: event.requestId,
    message: event.message,
    schema: event.schema,
  });

  const deferred = new Deferred<Record<string, string>>();
  pendingQuestions.set(event.requestId, deferred);

  try {
    const answers = await Promise.race([
      deferred.promise,
      abortToReject(signal),
      timeoutReject(60_000),
    ]);
    return { type: "elicitation", action: "accept", values: answers };
  } catch {
    return { type: "elicitation", action: "decline" };
  } finally {
    pendingQuestions.delete(event.requestId);
  }
}

function abortToReject(signal: AbortSignal): Promise<never> {
  return new Promise((_, reject) => {
    signal.addEventListener("abort", () => reject(signal.reason), {
      once: true,
    });
  });
}

function timeoutReject(ms: number): Promise<never> {
  return new Promise((_, reject) => {
    setTimeout(() => reject(new Error("Timed out")), ms);
  });
}

async function waitForInterrupt(
  characterId: string,
  resolvedKey: ResolvedPttKey | null,
  sessionSignal: AbortSignal,
): Promise<"ptt" | "ui"> {
  const pttPromise = resolvedKey
    ? waitForComboPress(resolvedKey, sessionSignal).then(() => "ptt" as const)
    : new Promise<never>(() => {});

  return Promise.race([
    pttPromise,
    waitForInterruptRpc(characterId, sessionSignal).then(() => "ui" as const),
  ]);
}

async function waitForInterruptRpc(
  characterId: string,
  signal: AbortSignal,
): Promise<void> {
  const deferred = new Deferred<void>();
  pendingInterrupts.set(characterId, deferred);

  try {
    await Promise.race([deferred.promise, abortToReject(signal)]);
  } finally {
    pendingInterrupts.delete(characterId);
  }
}

async function waitForUserInput(
  characterId: string,
  resolvedKey: ResolvedPttKey | null,
  signal: AbortSignal,
): Promise<UserInput | null> {
  const queue = textQueues.get(characterId);
  if (!queue) return null;

  if (!resolvedKey) {
    const text = await queue.shift(signal);
    return { text, source: "text" };
  }

  const inputAbort = new AbortController();
  const combined = AbortSignal.any([signal, inputAbort.signal]);

  const voicePromise = recognizeOneSentenceVoice(characterId, resolvedKey, combined)
    .then((text): UserInput | null => text ? { text, source: "voice" } : null);
  const textPromise = queue.shift(combined)
    .then((text): UserInput => ({ text, source: "text" }));

  try {
    return await Promise.race([voicePromise, textPromise]);
  } finally {
    inputAbort.abort();
    suppressRejection(voicePromise);
    suppressRejection(textPromise);
  }
}

async function recognizeOneSentenceVoice(
  characterId: string,
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<string | null> {
  await waitForComboPress(resolvedKey, signal);
  return await recognizeWhileHeld(characterId, resolvedKey, signal);
}

function suppressRejection(promise: Promise<unknown>): void {
  promise.catch(() => {});
}

async function recognizeWhileHeld(
  characterId: string,
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<string | null> {
  emitRecording(characterId, true);
  let session: stt.ptt.PttSession | null = null;

  try {
    session = await stt.ptt.start({ language: "ja" });
    await waitForComboRelease(keyboardHook, resolvedKey, signal);
    const result = await session.stop();
    return result.text?.trim() || null;
  } catch (e) {
    if (session) {
      try { await session.stop(); } catch { /* best-effort cleanup */ }
    }
    throw e;
  } finally {
    emitRecording(characterId, false);
  }
}

function waitForComboPress(
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<void> {
  return new Promise((resolve, reject) => {
    if (signal.aborted) {
      reject(signal.reason);
      return;
    }

    const unsubscribe = keyboardHook.subscribeCombo({
      onKeyEvent(pressedKeys) {
        if (isComboHeld(pressedKeys, resolvedKey)) {
          cleanup();
          resolve();
        }
      },
    });

    const onAbort = () => {
      cleanup();
      reject(signal.reason);
    };
    signal.addEventListener("abort", onAbort, { once: true });

    function cleanup() {
      unsubscribe();
      signal.removeEventListener("abort", onAbort);
    }
  });
}

function emitStatus(characterId: string, state: AgentStatus, reason?: string): void {
  signals.send("agent:status", { characterId, state, reason });
  console.debug(`[agent] ${characterId}: ${state}${reason ? ` (${reason})` : ""}`);
}

function emitLog(characterId: string, type: string, message: string): void {
  signals.send("agent:log", {
    characterId,
    type,
    message,
    timestamp: Date.now(),
  });
}

function emitUserLog(characterId: string, text: string, source: "voice" | "text"): void {
  signals.send("agent:log", {
    characterId,
    type: "user",
    message: text,
    source,
    timestamp: Date.now(),
  });
}

function emitInterruptLog(characterId: string): void {
  emitLog(characterId, "interrupt", "中断しました");
}

function emitRecording(characterId: string, recording: boolean): void {
  signals.send("agent:recording", { characterId, recording });
}

function isAbortError(e: unknown): boolean {
  return e instanceof DOMException && e.name === "AbortError";
}

function extractErrorMessage(err: unknown): string {
  if (!(err instanceof Error)) return String(err);
  const cause = err.cause instanceof Error ? `: ${err.cause.message}` : "";
  return `${err.message}${cause}`;
}

function buildRpcMethods() {
  return {
    "approve-permission": rpc.method({
      description: "Approve or deny a pending permission request",
      input: z.object({
        requestId: z.string(),
        approved: z.boolean(),
        decision: z.union([z.string(), z.record(z.unknown())]).optional(),
      }),
      handler: async ({ requestId, approved, decision }) => {
        const deferred = pendingApprovals.get(requestId);
        if (!deferred) {
          return { success: false as const, error: "No pending approval for this request" };
        }
        deferred.resolve({ approved, decision });
        return { success: true as const };
      },
    }),
    "answer-question": rpc.method({
      description: "Answer a pending question request",
      input: z.object({
        requestId: z.string(),
        answers: z.record(z.string()),
      }),
      handler: async ({ requestId, answers }) => {
        const deferred = pendingQuestions.get(requestId);
        if (deferred) {
          deferred.resolve(answers);
        }
        return { success: true as const };
      },
    }),
    "send-message": rpc.method({
      description: "Send a text message to the agent",
      input: z.object({
        characterId: z.string(),
        text: z.string().min(1),
      }),
      handler: async ({ characterId, text }) => {
        if (!activeSessions.has(characterId)) {
          return { success: false as const, error: "No active session" };
        }
        const queue = textQueues.get(characterId);
        if (!queue) {
          return { success: false as const, error: "No active session" };
        }
        await interruptSession(characterId);
        queue.clear();
        queue.push(text);
        return { success: true as const };
      },
    }),
    status: rpc.method({
      description: "Get the current session state for all characters",
      handler: async () => {
        const result: Record<string, string> = {};
        for (const [id] of activeSessions) {
          result[id] = "active";
        }
        return result;
      },
    }),
    "start-session": rpc.method({
      description: "Manually start an agent session for a character",
      input: z.object({ characterId: z.string() }),
      handler: async ({ characterId }) => {
        await startSession(characterId);
        return { success: true as const };
      },
    }),
    "stop-session": rpc.method({
      description: "Stop an active agent session for a character",
      input: z.object({ characterId: z.string() }),
      handler: async ({ characterId }) => {
        await stopSession(characterId);
        return { success: true as const };
      },
    }),
    "interrupt-session": rpc.method({
      description: "Interrupt the current agent execution for a character",
      input: z.object({ characterId: z.string() }),
      handler: async ({ characterId }) => {
        await interruptSession(characterId);
        return { success: true as const };
      },
    }),
    "list-worktrees": rpc.method({
      description: "List worktrees for a workspace",
      input: z.object({ workspacePath: z.string() }),
      handler: async ({ workspacePath }) => {
        const manager = new WorktreeManager(workspacePath);
        const worktrees = await manager.list();
        const results = await Promise.all(
          worktrees.map(async (wt) => {
            const status = await manager.status(wt.name);
            return { ...wt, ...status };
          }),
        );
        return { worktrees: results };
      },
    }),
    "add-worktree": rpc.method({
      description: "Create a new worktree in a workspace",
      input: z.object({
        workspacePath: z.string(),
        name: z.string().min(1),
        branch: z.string().min(1),
      }),
      handler: async ({ workspacePath, name, branch }) => {
        const manager = new WorktreeManager(workspacePath);
        const info = await manager.create(name, branch);
        return { worktree: info };
      },
    }),
    "remove-worktree": rpc.method({
      description: "Remove a worktree (optionally merge first)",
      input: z.object({
        workspacePath: z.string(),
        name: z.string(),
        action: z.enum(["remove", "merge"]),
      }),
      handler: async ({ workspacePath, name, action }) => {
        const manager = new WorktreeManager(workspacePath);
        if (action === "merge") {
          const result = await manager.merge(name);
          if (!result.success) {
            return { success: false, error: result.error };
          }
          return { success: true };
        }
        await manager.remove(name);
        return { success: true };
      },
    }),
    "list-branches": rpc.method({
      description: "List git branches for a workspace",
      input: z.object({ workspacePath: z.string() }),
      handler: async ({ workspacePath }) => {
        const branches = await listBranches(workspacePath);
        const current = await currentBranch(workspacePath);
        return { branches, current };
      },
    }),
    "worktree-status": rpc.method({
      description: "Get status of a specific worktree",
      input: z.object({ workspacePath: z.string(), name: z.string() }),
      handler: async ({ workspacePath, name }) => {
        const manager = new WorktreeManager(workspacePath);
        return await manager.status(name);
      },
    }),
  };
}

async function shutdown(): Promise<void> {
  console.log("[agent] Shutting down...");
  for (const [, controller] of activeSessions) {
    controller.abort();
  }
  activeSessions.clear();
  keyboardHook.stop();

  if (appServerProcess) {
    appServerProcess.shutdown();
    appServerProcess = null;
  }
}

async function main(): Promise<void> {
  try {
    currentApiKey = await loadApiKey();
  } catch {
    console.warn(
      "[agent] API key not configured. Sessions require an API key.",
    );
  }

  await startKeyboardHook();
  await rpc.serve({ methods: buildRpcMethods() });
}

main().catch((err) => console.error("[agent] Fatal startup error:", err));

process.once("SIGTERM", () => {
  shutdown().catch((err) => console.error("[agent] Shutdown error:", err));
});
