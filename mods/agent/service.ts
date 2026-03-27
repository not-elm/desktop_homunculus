import { z } from "zod";
import { Vrm, preferences, Webview, webviewSource, signals, stt } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { KeyboardHookService } from "./lib/keyboard-hook.ts";
import { resolvePttKeycodes, type ResolvedPttKey } from "./lib/key-mapping.ts";
import { ClaudeAgentExecuter } from "./lib/claude-agent-executer.ts";
import { CodexAgentExecuter } from "./lib/codex-agent-executer.ts";
import type { AIAgentExecuter } from "./lib/ai-agent-executer.ts";
import { Deferred } from "./lib/async-queue.ts";
import type { AgentEvent, AgentResponse } from "./lib/ai-agent-executer.ts";
import {
  type AgentSettings,
  type AgentStatus,
  type Persona,
  DEFAULT_SETTINGS,
} from "./lib/types.ts";
import { sanitizeForTts } from "./lib/tts.ts";
import { mkdirSync } from "node:fs";
import { homedir } from "node:os";
import path from "node:path";

const keyboardHook = new KeyboardHookService();

const activeSessions = new Map<string, AbortController>();
const activeWebviews = new Map<string, Webview>();
const pendingApprovals = new Map<string, Deferred<{ approved: boolean; message?: string }>>();
const pendingQuestions = new Map<string, Deferred<Record<string, string>>>();
const pendingInterrupts = new Map<string, Deferred<void>>();

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
  const saved = await preferences.load<AgentSettings>("agent::" + characterId);
  return saved ? { ...DEFAULT_SETTINGS, ...saved } : { ...DEFAULT_SETTINGS };
}

async function openSessionUi(characterId: string): Promise<Webview> {
  const vrm = await Vrm.findByName(characterId);
  return Webview.open({
    source: webviewSource.local("agent:session-ui"),
    size: [0.6, 0.8],
    viewportSize: [400, 500],
    linkedVrm: vrm.entity,
    offset: [-0.8, 0],
  });
}

function closeSessionUi(characterId: string): void {
  const webview = activeWebviews.get(characterId);
  if (webview) {
    webview.close().catch((err) =>
      console.warn("[agent] Failed to close session UI:", err),
    );
    activeWebviews.delete(characterId);
  }
}

async function speakGreeting(
  characterId: string,
  phrases: string[],
): Promise<void> {
  if (phrases.length === 0) return;
  const phrase = phrases[Math.floor(Math.random() * phrases.length)];
  await rpc
    .call({
      modName: "@hmcs/voicevox",
      method: "speak",
      body: { name: characterId, text: phrase },
    })
    .catch(() => console.warn("[agent] TTS unavailable for greeting"));
}

async function speakRandomPhrase(
  characterId: string,
  phrases: string[],
): Promise<void> {
  if (phrases.length === 0) return;
  const phrase = phrases[Math.floor(Math.random() * phrases.length)];
  rpc
    .call({
      modName: "@hmcs/voicevox",
      method: "speak",
      body: { name: characterId, text: phrase },
    })
    .catch(() => emitLog(characterId, "warning", "TTS unavailable"));
}

function speakText(characterId: string, text: string): void {
  const sentences = sanitizeForTts(text);
  if (sentences.length === 0) return;
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

  const resolvedKey = validatePttKey(settings);
  const persona = await loadPersona(characterId);
  const workDir = resolveWorkingDirectory(characterId, settings);
  mkdirSync(workDir, { recursive: true });

  const executer = createExecuter(settings, persona, currentApiKey, workDir);

  const sessionAbort = new AbortController();
  activeSessions.set(characterId, sessionAbort);

  const webview = await openSessionUi(characterId);
  activeWebviews.set(characterId, webview);
  await speakGreeting(characterId, settings.greetingPhrases);
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
  resolvedKey: ResolvedPttKey,
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
    speakRandomPhrase(characterId, settings.errorPhrases);
  }
  activeSessions.delete(characterId);
  closeSessionUi(characterId);
  emitStatus(characterId, "idle");
}

function validatePttKey(settings: AgentSettings): ResolvedPttKey {
  if (!settings.pttKey) {
    throw new Error(
      "PTT key not configured. Open Agent Settings to set a push-to-talk key.",
    );
  }
  const resolved = resolvePttKeycodes(settings.pttKey);
  if (!resolved) {
    throw new Error("PTT key could not be resolved.");
  }
  return resolved;
}

async function loadPersona(characterId: string): Promise<Persona> {
  const vrm = await Vrm.findByName(characterId);
  const sdkPersona = await vrm.persona();
  return { name: characterId, personality: sdkPersona.personality ?? "" };
}

function resolveWorkingDirectory(
  characterId: string,
  settings: AgentSettings,
): string {
  const dirs = settings.workingDirectories;
  return (
    dirs.paths[dirs.default] ??
    path.join(homedir(), ".homunculus", "agents", characterId)
  );
}

function createExecuter(
  settings: AgentSettings,
  persona: Persona,
  apiKey: string | null,
  workDir: string,
): AIAgentExecuter {
  switch (settings.executor) {
    case "codex":
      return new CodexAgentExecuter(persona, settings, workDir);
    case "sdk":
      return new ClaudeAgentExecuter(persona, settings, apiKey!, workDir);
    default:
      throw new Error(`Executor "${settings.executor}" is not yet implemented.`);
  }
}

async function stopSession(characterId: string): Promise<void> {
  const controller = activeSessions.get(characterId);
  if (!controller) return;
  controller.abort();
  activeSessions.delete(characterId);
  closeSessionUi(characterId);
  emitStatus(characterId, "idle");
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

async function runSession(
  characterId: string,
  executer: AIAgentExecuter,
  sessionAbort: AbortController,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey,
): Promise<void> {
  let sessionId = await loadSavedSession(characterId, settings.executor);
  const signal = sessionAbort.signal;

  try {
    while (!signal.aborted) {
      emitStatus(characterId, "listening");
      const text = await recognizeOneSentence(
        characterId,
        resolvedKey,
        signal,
      );
      if (text === null) continue;

      emitUserLog(characterId, text);
      sessionId = await executeOneRound(
        characterId,
        executer,
        text,
        sessionId,
        settings,
        signal,
      );
    }
  } catch (e) {
    if (!isAbortError(e)) throw e;
  } finally {
    activeSessions.delete(characterId);
    emitStatus(characterId, "idle");
  }
}

async function executeOneRound(
  characterId: string,
  executer: AIAgentExecuter,
  text: string,
  sessionId: string | null,
  settings: AgentSettings,
  sessionSignal: AbortSignal,
): Promise<string | null> {
  const interruptAbort = new AbortController();
  const executorGen = executer.execute(text, sessionId, interruptAbort.signal);
  const interruptPromise = waitForInterrupt(characterId, settings, sessionSignal);

  try {
    return await driveExecutor(
      characterId,
      executorGen,
      interruptPromise,
      interruptAbort,
      sessionId,
      settings,
      sessionSignal,
    );
  } catch (e) {
    if (!isAbortError(e)) throw e;
    return sessionId;
  }
}

async function driveExecutor(
  characterId: string,
  executorGen: AsyncGenerator<AgentEvent, void, AgentResponse | undefined>,
  interruptPromise: Promise<"ptt" | "ui">,
  interruptAbort: AbortController,
  sessionId: string | null,
  settings: AgentSettings,
  sessionSignal: AbortSignal,
): Promise<string | null> {
  let lastSessionId = sessionId;
  let response: AgentResponse | undefined = undefined;

  while (true) {
    const raceResult = await raceInterrupt(executorGen.next(response), interruptPromise);

    if (raceResult.interrupted) {
      return await abortExecution(characterId, executorGen, interruptAbort, lastSessionId);
    }
    if (raceResult.done) break;

    const event = raceResult.value as AgentEvent;
    response = await handleAgentEvent(characterId, event, settings, sessionSignal);
    if (event.type === "completed") lastSessionId = event.sessionId;
  }

  return lastSessionId;
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
  return Promise.race([
    nextResult.then((r): RaceResultDone | RaceResultEvent =>
      r.done
        ? { interrupted: false, done: true, value: undefined }
        : { interrupted: false, done: false, value: r.value as AgentEvent },
    ),
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
  emitStatus(characterId, "idle");
  saveSession(characterId, settings.executor, sessionId);
  speakRandomPhrase(characterId, settings.completionPhrases);
  return undefined;
}

function handleError(
  characterId: string,
  message: string,
  settings: AgentSettings,
): undefined {
  emitLog(characterId, "error", message);
  speakRandomPhrase(characterId, settings.errorPhrases);
  return undefined;
}

async function resolvePermission(
  characterId: string,
  event: AgentEvent & { type: "permission_request" },
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse> {
  const deferred = new Deferred<{ approved: boolean; message?: string }>();
  const permAbort = new AbortController();
  const combined = AbortSignal.any([signal, permAbort.signal]);

  pendingApprovals.set(event.requestId, deferred);
  console.log(`[agent] resolvePermission: ${event.tool} (${event.requestId})`);

  signals.send("agent:permission", {
    characterId,
    requestId: event.requestId,
    action: event.tool,
    target: JSON.stringify(event.input),
  });

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
    return { type: "permission", approved: result.approved, message: result.message };
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
  deferred: Deferred<{ approved: boolean; message?: string }>,
): void {
  (async () => {
    try {
      await waitForComboPress(resolvedKey, signal);
      emitRecording(characterId, true);
      try {
        const text = await recognizeWithCancel(resolvedKey, signal);
        if (text === null) {
          deferred.resolve({ approved: false, message: "Cancelled" });
        } else {
          deferred.resolve(evaluateApprovalPhrase(text, settings));
        }
      } finally {
        emitRecording(characterId, false);
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
  settings: AgentSettings,
  sessionSignal: AbortSignal,
): Promise<"ptt" | "ui"> {
  return Promise.race([
    waitForPttDuringExecution(settings, sessionSignal).then(
      () => "ptt" as const,
    ),
    waitForInterruptRpc(characterId, sessionSignal).then(
      () => "ui" as const,
    ),
  ]);
}

async function waitForPttDuringExecution(
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<void> {
  const resolvedKey = resolvePttKeycodes(settings.pttKey!);
  if (!resolvedKey) return new Promise<void>(() => {});
  await waitForComboPress(resolvedKey, signal);
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

async function recognizeOneSentence(
  characterId: string,
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<string | null> {
  await waitForComboPress(resolvedKey, signal);
  emitRecording(characterId, true);

  try {
    const text = await recognizeWithCancel(resolvedKey, signal);
    return text?.trim() || null;
  } finally {
    emitRecording(characterId, false);
  }
}

async function recognizeWithCancel(
  resolvedKey: ResolvedPttKey,
  sessionSignal: AbortSignal,
): Promise<string | null> {
  const cancelAbort = new AbortController();
  const combined = AbortSignal.any([sessionSignal, cancelAbort.signal]);

  const cancelPromise = waitForComboPress(resolvedKey, sessionSignal);
  const recognizePromise = stt.recognize({ language: "ja" }, combined);

  const result = await Promise.race([
    recognizePromise.then((r) => ({ text: r.text })),
    cancelPromise.then(() => {
      cancelAbort.abort();
      return { text: null };
    }),
  ]);

  return result.text;
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

function isComboHeld(
  pressedKeys: ReadonlySet<number>,
  key: ResolvedPttKey,
): boolean {
  if (!pressedKeys.has(key.primaryKeycode)) return false;
  return key.modifiers.every((keycodes) =>
    keycodes.some((kc) => pressedKeys.has(kc)),
  );
}

function emitStatus(characterId: string, state: AgentStatus): void {
  signals.send("agent:status", { characterId, state });
}

function emitLog(characterId: string, type: string, message: string): void {
  signals.send("agent:log", {
    characterId,
    type,
    message,
    timestamp: Date.now(),
  });
}

function emitUserLog(characterId: string, text: string): void {
  emitLog(characterId, "user", text);
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
      input: z.object({ requestId: z.string(), approved: z.boolean() }),
      handler: async ({ requestId, approved }) => {
        const deferred = pendingApprovals.get(requestId);
        if (!deferred) {
          return { success: false as const, error: "No pending approval for this request" };
        }
        deferred.resolve({ approved });
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
  };
}

async function shutdown(): Promise<void> {
  console.log("[agent] Shutting down...");
  for (const [, controller] of activeSessions) {
    controller.abort();
  }
  activeSessions.clear();
  keyboardHook.stop();
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
