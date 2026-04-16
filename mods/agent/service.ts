import { mkdirSync } from 'node:fs';
import { homedir } from 'node:os';
import path from 'node:path';
import { audio, preferences, Persona as SdkPersona, signals, stt } from '@hmcs/sdk';
import { rpc } from '@hmcs/sdk/rpc';
import { z } from 'zod';
import type { AgentEvent, AgentResponse, AgentRuntime } from './lib/agent-runtime.ts';
import { AsyncQueue, Deferred } from './lib/async-queue.ts';
import { ClaudeAgentRuntime } from './lib/claude-agent-runtime.ts';
import { CodexAppServerProcess } from './lib/codex-appserver-process.ts';
import { CodexAppServerRuntime } from './lib/codex-appserver-runtime.ts';
import { currentBranch, gitExec, isGitRepo, listBranches } from './lib/git.ts';
import { type ResolvedPttKey, resolvePttKeycodes } from './lib/key-mapping.ts';
import { isComboHeld, KeyboardHookService, waitForComboRelease } from './lib/keyboard-hook.ts';
import { DEFAULT_PERMISSION_SE, resolvePermissionSeAsset } from './lib/permission-se.ts';
import { buildPersonaPrompt, buildSessionContext, type WorktreeContext } from './lib/prompt.ts';
import {
  type PersistLogEntry,
  type SessionHandle,
  SessionPersistence,
} from './lib/session-persistence.ts';
import { sanitizeForTts } from './lib/tts.ts';
import {
  type AgentSettings,
  type AgentStatus,
  DEFAULT_SETTINGS,
  type Persona,
} from './lib/types.ts';
import { WORKTREE_NAME_PATTERN, WorktreeManager } from './lib/worktree-manager.ts';

const keyboardHook = new KeyboardHookService();

const activeSessions = new Map<string, AbortController>();
const pendingApprovals = new Map<
  string,
  Deferred<{ approved: boolean; message?: string; decision?: string | Record<string, unknown> }>
>();
const pendingQuestions = new Map<string, Deferred<Record<string, string>>>();
const pendingInterrupts = new Map<string, Deferred<void>>();
const textQueues = new Map<string, AsyncQueue<string>>();
const textFocusedPersonas = new Set<string>();

const persistence = new SessionPersistence();
const activeSessionHandles = new Map<string, SessionHandle>();

let appServerProcess: CodexAppServerProcess | null = null;

function getAppServerProcess(): CodexAppServerProcess {
  if (!appServerProcess) {
    appServerProcess = new CodexAppServerProcess();
  }
  return appServerProcess;
}

let currentApiKey: string | null = null;

async function loadApiKey(): Promise<string> {
  const apiKey = await preferences.load<string>('agent::api-key');
  if (!apiKey) throw new Error("API key not configured. Set 'agent::api-key' in preferences.");
  return apiKey;
}

async function loadPersonaSettings(personaId: string): Promise<AgentSettings> {
  const saved = await preferences.load<Record<string, unknown>>(`agent::${personaId}`);
  if (!saved) return { ...DEFAULT_SETTINGS };

  // Migrate workingDirectories → workspaces
  if ('workingDirectories' in saved && !('workspaces' in saved)) {
    const wd = saved.workingDirectories as { paths: string[]; default: number };
    saved.workspaces = {
      paths: wd.paths,
      selection: { workspaceIndex: wd.default, worktreeName: null },
    };
    delete saved.workingDirectories;
    await preferences.save(`agent::${personaId}`, { ...DEFAULT_SETTINGS, ...saved });
  }

  return { ...DEFAULT_SETTINGS, ...(saved as Partial<AgentSettings>) };
}

async function resolveTtsModName(personaId: string): Promise<string | null> {
  const persona = await SdkPersona.load(personaId);
  const metadata = await persona.metadata();
  return (metadata?.ttsModName as string | null) ?? null;
}

function speakText(personaId: string, text: string): void {
  const { sentences, log } = sanitizeForTts(text);
  if (sentences.length === 0) return;
  if (log.length > 0) {
    emitLog(personaId, 'tts-sanitize', log.join('; '));
  }
  resolveTtsModName(personaId)
    .then((ttsModName) => {
      if (ttsModName === null) return;
      return rpc.call({
        modName: ttsModName,
        method: 'speak',
        body: { personaId, text: sentences },
      });
    })
    .catch(() => emitLog(personaId, 'warning', 'TTS unavailable'));
}

async function startKeyboardHook(): Promise<void> {
  const started = keyboardHook.start();
  if (!started) {
    console.warn('[agent] Keyboard hook failed to start.');
  }
}

async function scanOrphanedWorktrees(): Promise<void> {
  const snapshots = await SdkPersona.list();
  for (const snapshot of snapshots) {
    const personaId = snapshot.id;
    if (activeSessions.has(personaId)) continue;
    const settings = await loadPersonaSettings(personaId);
    for (const wsPath of settings.workspaces.paths) {
      try {
        if (!(await isGitRepo(wsPath))) continue;
        const manager = new WorktreeManager(wsPath);
        const worktrees = await manager.list();
        for (const wt of worktrees) {
          const hasChanges = await manager.hasUncommittedChanges(wt.name);
          if (hasChanges) {
            await signals.send('agent:worktree', {
              personaId,
              state: 'orphaned',
              worktreeName: wt.name,
              workspacePath: wsPath,
            });
            emitLog(
              personaId,
              'warning',
              `Orphaned worktree detected: ${wt.name} in ${wsPath} (has uncommitted changes)`,
            );
          }
        }
      } catch {
        // Skip workspaces that can't be scanned
      }
    }
  }
}

async function startSession(
  personaId: string,
  contextSessionUuid?: string,
): Promise<PersistLogEntry[]> {
  const settings = await loadPersonaSettings(personaId);
  assertCanStartSession(personaId, settings);

  const resolvedKey = resolvePttKeyOptional(settings);
  const persona = await loadPersona(personaId);
  const workDir = resolveWorkingDirectory(personaId, settings);
  mkdirSync(workDir, { recursive: true });

  const selection = settings.workspaces.selection;
  if (selection.worktreeName) {
    await signals.send('agent:worktree', {
      personaId,
      state: 'created',
      worktreeName: selection.worktreeName,
      workspacePath: settings.workspaces.paths[selection.workspaceIndex],
    });
  }

  const worktreeCtx = await buildWorktreeContext(settings, workDir);

  const basePath = settings.workspaces.paths[selection.workspaceIndex];
  const branchName = basePath ? await resolveCurrentBranch(basePath, selection.worktreeName) : null;

  // Read previous session context for prompt injection
  let sessionContext: string | undefined;
  let replayEntries: PersistLogEntry[] = [];
  if (basePath && branchName) {
    const contextUuid =
      contextSessionUuid ??
      (await persistence.findLatestSessionUuid(basePath, personaId, branchName));
    if (contextUuid) {
      const entries = await persistence.read(basePath, personaId, branchName, contextUuid);
      if (entries.length > 0) {
        sessionContext = buildSessionContext(entries);
        replayEntries = entries;
      }
    }
  }

  const prompt = buildPersonaPrompt(persona, worktreeCtx, sessionContext);
  const runtime = createRuntime(settings, prompt, currentApiKey, workDir);

  if (basePath && branchName) {
    const handle = await persistence.create({
      workspacePath: basePath,
      personaId,
      branchName,
    });
    activeSessionHandles.set(personaId, handle);
    persistence.cleanup(basePath, personaId).catch(() => {});
  }

  const sessionAbort = new AbortController();
  activeSessions.set(personaId, sessionAbort);
  textQueues.set(personaId, new AsyncQueue<string>());

  launchSessionLoop(personaId, runtime, sessionAbort, settings, resolvedKey);
  return replayEntries;
}

function assertCanStartSession(personaId: string, settings: AgentSettings): void {
  if (settings.runtime === 'sdk' && !currentApiKey) {
    throw new Error('API key not configured. Open Agent Settings to set your Anthropic API key.');
  }
  if (activeSessions.has(personaId)) {
    throw new Error(`Session already active for "${personaId}".`);
  }
}

function launchSessionLoop(
  personaId: string,
  runtime: AgentRuntime,
  sessionAbort: AbortController,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
): void {
  runSession(personaId, runtime, sessionAbort, settings, resolvedKey).catch((err) =>
    handleSessionCrash(personaId, err, settings),
  );
}

function handleSessionCrash(personaId: string, err: unknown, _settings: AgentSettings): void {
  if (!isAbortError(err)) {
    console.error(`[agent] Session error for ${personaId}:`, err);
    emitLog(personaId, 'error', extractErrorMessage(err));
  }
  const handle = activeSessionHandles.get(personaId);
  if (handle) {
    persistence.close(handle).catch(() => {});
    activeSessionHandles.delete(personaId);
  }
  const queue = textQueues.get(personaId);
  if (queue) {
    queue.rejectAll(new DOMException('Session crashed', 'AbortError'));
    textQueues.delete(personaId);
  }
  activeSessions.delete(personaId);
  emitStatus(personaId, 'idle', isAbortError(err) ? 'stopped' : 'crashed');
}

function resolvePttKeyOptional(settings: AgentSettings): ResolvedPttKey | null {
  if (!settings.pttKey) return null;
  return resolvePttKeycodes(settings.pttKey) ?? null;
}

async function loadPersona(personaId: string): Promise<Persona> {
  const p = await SdkPersona.load(personaId);
  const snapshot = await p.snapshot();
  return {
    name: snapshot.name || personaId,
    age: snapshot.age ?? null,
    gender: snapshot.gender ?? 'unknown',
    firstPersonPronoun: snapshot.firstPersonPronoun ?? null,
    profile: snapshot.profile ?? '',
    personality: snapshot.personality ?? null,
  };
}

async function playPermissionSe(personaId: string): Promise<void> {
  let assetId: string | null;
  try {
    const p = await SdkPersona.load(personaId);
    const metadata = await p.metadata();
    assetId = resolvePermissionSeAsset(metadata);
  } catch (e) {
    console.error(`[agent] failed to load permission SE metadata, using default:`, e);
    assetId = DEFAULT_PERMISSION_SE;
  }
  if (assetId) {
    audio.se.play(assetId).catch((e) => {
      console.error(`[agent] failed to play permission SE:`, e);
    });
  }
}

function resolveWorkingDirectory(personaId: string, settings: AgentSettings): string {
  const { paths, selection } = settings.workspaces;
  const basePath = paths[selection.workspaceIndex];
  if (!basePath) return path.join(homedir(), '.homunculus', 'agents', personaId);
  if (selection.worktreeName) {
    return path.join(basePath, '.hmcs/worktrees', selection.worktreeName);
  }
  return basePath;
}

async function buildWorktreeContext(
  settings: AgentSettings,
  workDir: string,
): Promise<WorktreeContext | undefined> {
  const { selection } = settings.workspaces;
  if (!selection.worktreeName) return undefined;
  const baseBranch = await readWorktreeBaseBranch(workDir);
  return {
    worktreeName: selection.worktreeName,
    baseBranch,
    worktreePath: workDir,
  };
}

async function readWorktreeBaseBranch(worktreePath: string): Promise<string> {
  try {
    const result = await gitExec(worktreePath, ['config', 'hmcs.baseBranch']);
    return result.trim() || 'main';
  } catch {
    return 'main';
  }
}

/** Resolve the current git branch for session scoping. */
async function resolveCurrentBranch(
  workspacePath: string,
  worktreeName: string | null,
): Promise<string | null> {
  try {
    if (worktreeName) {
      const manager = new WorktreeManager(workspacePath);
      const worktrees = await manager.list();
      const wt = worktrees.find((w) => w.name === worktreeName);
      return wt?.branch ?? null;
    }
    return await currentBranch(workspacePath);
  } catch {
    return null;
  }
}

function createRuntime(
  settings: AgentSettings,
  prompt: string,
  apiKey: string | null,
  workDir: string,
): AgentRuntime {
  switch (settings.runtime) {
    case 'codex':
      return new CodexAppServerRuntime(prompt, settings, workDir, getAppServerProcess());
    case 'sdk':
      return new ClaudeAgentRuntime(prompt, settings, apiKey as string, workDir);
    default:
      throw new Error(`Runtime "${settings.runtime}" is not yet implemented.`);
  }
}

async function stopSession(personaId: string): Promise<void> {
  const controller = activeSessions.get(personaId);
  if (!controller) return;

  const handle = activeSessionHandles.get(personaId);
  if (handle) {
    await persistence.close(handle).catch(() => {});
    activeSessionHandles.delete(personaId);
  }

  controller.abort();
  const queue = textQueues.get(personaId);
  if (queue) {
    queue.rejectAll(new DOMException('Session stopped', 'AbortError'));
    textQueues.delete(personaId);
  }
  activeSessions.delete(personaId);
  emitStatus(personaId, 'idle', 'stopped');

  if (appServerProcess && appServerProcess.refCount === 0) {
    appServerProcess.shutdown();
    appServerProcess = null;
  }
}

async function interruptSession(personaId: string): Promise<void> {
  const deferred = pendingInterrupts.get(personaId);
  if (deferred) {
    deferred.resolve();
  }
}

interface UserInput {
  text: string;
  source: 'voice' | 'text';
}

async function runSession(
  personaId: string,
  runtime: AgentRuntime,
  sessionAbort: AbortController,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
): Promise<void> {
  let sessionId: string | null = null;
  const signal = sessionAbort.signal;

  try {
    while (!signal.aborted) {
      emitStatus(personaId, 'listening');
      const input = await waitForUserInput(personaId, resolvedKey, signal);
      if (input === null) continue;

      emitUserLog(personaId, input.text, input.source);
      emitStatus(personaId, 'thinking');
      const result = await executeOneRound(
        personaId,
        runtime,
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
    activeSessions.delete(personaId);
    textFocusedPersonas.delete(personaId);

    const handle = activeSessionHandles.get(personaId);
    if (handle) {
      await persistence.close(handle).catch(() => {});
      activeSessionHandles.delete(personaId);
    }

    emitStatus(personaId, 'idle', 'session-ended');
  }
}

interface RoundResult {
  sessionId: string | null;
}

async function executeOneRound(
  personaId: string,
  runtime: AgentRuntime,
  text: string,
  sessionId: string | null,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
  sessionSignal: AbortSignal,
): Promise<RoundResult> {
  const interruptAbort = new AbortController();
  const runtimeGen = runtime.execute(text, sessionId, interruptAbort.signal);
  const interruptPromise = waitForInterrupt(personaId, resolvedKey, sessionSignal);

  try {
    return await driveRuntime(
      personaId,
      runtimeGen,
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

async function driveRuntime(
  personaId: string,
  runtimeGen: AsyncGenerator<AgentEvent, void, AgentResponse | undefined>,
  interruptPromise: Promise<'ptt' | 'ui'>,
  interruptAbort: AbortController,
  sessionId: string | null,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
  sessionSignal: AbortSignal,
): Promise<RoundResult> {
  let lastSessionId = sessionId;
  let response: AgentResponse | undefined;

  while (true) {
    const raceResult = await raceInterrupt(runtimeGen.next(response), interruptPromise);

    if (raceResult.interrupted) {
      lastSessionId = await abortExecution(personaId, runtimeGen, interruptAbort, lastSessionId);

      if (raceResult.source === 'ptt' && resolvedKey) {
        const voiceText = await recognizeWhileHeld(personaId, resolvedKey, sessionSignal);
        if (voiceText) {
          const queue = textQueues.get(personaId);
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
    response = await handleAgentEvent(personaId, event, settings, sessionSignal);
    if (event.type === 'completed' || event.type === 'error') {
      if (event.type === 'completed') lastSessionId = event.sessionId;
      break;
    }
  }

  await runtimeGen.return(undefined);
  return { sessionId: lastSessionId };
}

async function abortExecution(
  personaId: string,
  runtimeGen: AsyncGenerator<AgentEvent, void, AgentResponse | undefined>,
  interruptAbort: AbortController,
  lastSessionId: string | null,
): Promise<string | null> {
  interruptAbort.abort();
  await runtimeGen.return(undefined);
  emitInterruptLog(personaId);
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
  source: 'ptt' | 'ui';
}

type RaceResult = RaceResultDone | RaceResultEvent | RaceResultInterrupted;

async function raceInterrupt(
  nextResult: Promise<IteratorResult<AgentEvent, void>>,
  interruptPromise: Promise<'ptt' | 'ui'>,
): Promise<RaceResult> {
  const runtimeSide = nextResult.then((r): RaceResultDone | RaceResultEvent =>
    r.done
      ? { interrupted: false, done: true, value: undefined }
      : { interrupted: false, done: false, value: r.value as AgentEvent },
  );
  // Suppress unhandled rejection on the losing side of the race.
  runtimeSide.catch(() => {});

  return Promise.race([
    runtimeSide,
    interruptPromise.then((source): RaceResultInterrupted => ({ interrupted: true, source })),
  ]);
}

async function handleAgentEvent(
  personaId: string,
  event: AgentEvent,
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse | undefined> {
  switch (event.type) {
    case 'assistant_message':
      return handleAssistantMessage(personaId, event.text);
    case 'tool_use':
      return handleToolUse(personaId, event.summary);
    case 'permission_request':
      return await handlePermissionRequest(personaId, event, settings, signal);
    case 'elicitation_request':
      return await handleElicitationRequest(personaId, event, signal);
    case 'completed':
      return handleCompleted(personaId, event.sessionId);
    case 'error':
      return handleError(personaId, event.message, settings);
  }
}

function handleAssistantMessage(personaId: string, text: string): undefined {
  emitStatus(personaId, 'thinking');
  emitLog(personaId, 'assistant', text);
  speakText(personaId, text);
  return undefined;
}

function handleToolUse(personaId: string, summary: string): undefined {
  emitStatus(personaId, 'executing');
  emitLog(personaId, 'tool', summary);
  return undefined;
}

async function handlePermissionRequest(
  personaId: string,
  event: AgentEvent & { type: 'permission_request' },
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse> {
  emitStatus(personaId, 'waiting');
  return await resolvePermission(personaId, event, settings, signal);
}

async function handleElicitationRequest(
  personaId: string,
  event: AgentEvent & { type: 'elicitation_request' },
  signal: AbortSignal,
): Promise<AgentResponse> {
  emitStatus(personaId, 'waiting');
  return await resolveElicitation(personaId, event, signal);
}

function handleCompleted(_personaId: string, _sessionId: string): undefined {
  return undefined;
}

function handleError(personaId: string, message: string, _settings: AgentSettings): undefined {
  console.error(`[agent] ${personaId}: error — ${message}`);
  emitLog(personaId, 'error', message);
  return undefined;
}

async function resolvePermission(
  personaId: string,
  event: AgentEvent & { type: 'permission_request' },
  settings: AgentSettings,
  signal: AbortSignal,
): Promise<AgentResponse> {
  const deferred = new Deferred<{
    approved: boolean;
    message?: string;
    decision?: string | Record<string, unknown>;
  }>();
  const permAbort = new AbortController();
  const combined = AbortSignal.any([signal, permAbort.signal]);

  pendingApprovals.set(event.requestId, deferred);

  const permissionPayload = {
    personaId,
    requestId: event.requestId,
    action: event.tool,
    target: JSON.stringify(event.input),
    availableDecisions: event.availableDecisions,
  };
  console.log(
    `[agent] resolvePermission: ${event.tool} (${event.requestId})`,
    JSON.stringify(permissionPayload, null, 2),
  );

  playPermissionSe(personaId);

  signals.send('agent:permission', permissionPayload);

  const timer = setTimeout(
    () => deferred.resolve({ approved: false, message: 'Permission request timed out' }),
    60_000,
  );

  const onAbort = () => deferred.reject(signal.reason);
  signal.addEventListener('abort', onAbort, { once: true });

  const resolvedKey = resolvePttKeycodes(settings.pttKey as NonNullable<typeof settings.pttKey>);
  if (resolvedKey) {
    runVoiceApproval(personaId, resolvedKey, settings, combined, deferred);
  }

  try {
    const result = await deferred.promise;
    console.log(
      `[agent] permission result: approved=${result.approved}, message=${result.message}`,
    );
    return {
      type: 'permission',
      approved: result.approved,
      message: result.message,
      decision: result.decision,
    };
  } finally {
    clearTimeout(timer);
    signal.removeEventListener('abort', onAbort);
    permAbort.abort();
    pendingApprovals.delete(event.requestId);
  }
}

function runVoiceApproval(
  personaId: string,
  resolvedKey: ResolvedPttKey,
  settings: AgentSettings,
  signal: AbortSignal,
  deferred: Deferred<{
    approved: boolean;
    message?: string;
    decision?: string | Record<string, unknown>;
  }>,
): void {
  (async () => {
    try {
      await waitForComboPress(personaId, resolvedKey, signal);
      const text = await recognizeWhileHeld(personaId, resolvedKey, signal);
      if (text === null) {
        deferred.resolve({ approved: false, message: 'No speech detected' });
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
  const isApproval = settings.approvalPhrases.some((p) => lower.includes(p.toLowerCase()));
  if (isApproval) return { approved: true };
  const isDenial = settings.denyPhrases.some((p) => lower.includes(p.toLowerCase()));
  if (isDenial) return { approved: false, message: `Denied: "${text}"` };
  return { approved: false, message: `Unrecognized: "${text}"` };
}

async function resolveElicitation(
  personaId: string,
  event: AgentEvent & { type: 'elicitation_request' },
  signal: AbortSignal,
): Promise<AgentResponse> {
  signals.send('agent:question', {
    personaId,
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
    return { type: 'elicitation', action: 'accept', values: answers };
  } catch {
    return { type: 'elicitation', action: 'decline' };
  } finally {
    pendingQuestions.delete(event.requestId);
  }
}

function abortToReject(signal: AbortSignal): Promise<never> {
  return new Promise((_, reject) => {
    signal.addEventListener('abort', () => reject(signal.reason), {
      once: true,
    });
  });
}

function timeoutReject(ms: number): Promise<never> {
  return new Promise((_, reject) => {
    setTimeout(() => reject(new Error('Timed out')), ms);
  });
}

async function waitForInterrupt(
  personaId: string,
  resolvedKey: ResolvedPttKey | null,
  sessionSignal: AbortSignal,
): Promise<'ptt' | 'ui'> {
  const pttPromise = resolvedKey
    ? waitForComboPress(personaId, resolvedKey, sessionSignal).then(() => 'ptt' as const)
    : new Promise<never>(() => {});

  return Promise.race([
    pttPromise,
    waitForInterruptRpc(personaId, sessionSignal).then(() => 'ui' as const),
  ]);
}

async function waitForInterruptRpc(personaId: string, signal: AbortSignal): Promise<void> {
  const deferred = new Deferred<void>();
  pendingInterrupts.set(personaId, deferred);

  try {
    await Promise.race([deferred.promise, abortToReject(signal)]);
  } finally {
    pendingInterrupts.delete(personaId);
  }
}

async function waitForUserInput(
  personaId: string,
  resolvedKey: ResolvedPttKey | null,
  signal: AbortSignal,
): Promise<UserInput | null> {
  const queue = textQueues.get(personaId);
  if (!queue) return null;

  if (!resolvedKey) {
    const text = await queue.shift(signal);
    return { text, source: 'text' };
  }

  const inputAbort = new AbortController();
  const combined = AbortSignal.any([signal, inputAbort.signal]);

  const voicePromise = recognizeOneSentenceVoice(personaId, resolvedKey, combined).then(
    (text): UserInput | null => (text ? { text, source: 'voice' } : null),
  );
  const textPromise = queue.shift(combined).then((text): UserInput => ({ text, source: 'text' }));

  try {
    return await Promise.race([voicePromise, textPromise]);
  } finally {
    inputAbort.abort();
    suppressRejection(voicePromise);
    suppressRejection(textPromise);
  }
}

async function recognizeOneSentenceVoice(
  personaId: string,
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<string | null> {
  await waitForComboPress(personaId, resolvedKey, signal);
  return await recognizeWhileHeld(personaId, resolvedKey, signal);
}

function suppressRejection(promise: Promise<unknown>): void {
  promise.catch(() => {});
}

async function recognizeWhileHeld(
  personaId: string,
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<string | null> {
  emitRecording(personaId, true);
  let session: stt.ptt.PttSession | null = null;

  try {
    session = await stt.ptt.start({ language: 'ja' });
    await waitForComboRelease(keyboardHook, resolvedKey, signal);
    const result = await session.stop();
    return result.text?.trim() || null;
  } catch (e) {
    if (session) {
      try {
        await session.stop();
      } catch {
        /* best-effort cleanup */
      }
    }
    throw e;
  } finally {
    emitRecording(personaId, false);
  }
}

function waitForComboPress(
  personaId: string,
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
        if (textFocusedPersonas.has(personaId)) return;
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
    signal.addEventListener('abort', onAbort, { once: true });

    function cleanup() {
      unsubscribe();
      signal.removeEventListener('abort', onAbort);
    }
  });
}

function emitStatus(personaId: string, state: AgentStatus, reason?: string): void {
  signals.send('agent:status', { personaId, state, reason });
  console.debug(`[agent] ${personaId}: ${state}${reason ? ` (${reason})` : ''}`);
}

function emitLog(personaId: string, type: string, message: string): void {
  const entry = { type, message, timestamp: Date.now() };
  signals.send('agent:log', { personaId, ...entry });

  const handle = activeSessionHandles.get(personaId);
  if (handle) persistence.append(handle, entry);
}

function emitUserLog(personaId: string, text: string, source: 'voice' | 'text'): void {
  const entry = { type: 'user' as const, message: text, timestamp: Date.now(), source };
  signals.send('agent:log', { personaId, ...entry });

  const handle = activeSessionHandles.get(personaId);
  if (handle) persistence.append(handle, entry);
}

function emitInterruptLog(personaId: string): void {
  emitLog(personaId, 'interrupt', '中断しました');
}

function emitRecording(personaId: string, recording: boolean): void {
  signals.send('agent:recording', { personaId, recording });
}

function isAbortError(e: unknown): boolean {
  return e instanceof DOMException && e.name === 'AbortError';
}

function extractErrorMessage(err: unknown): string {
  if (!(err instanceof Error)) return String(err);
  const cause = err.cause instanceof Error ? `: ${err.cause.message}` : '';
  return `${err.message}${cause}`;
}

function buildRpcMethods() {
  return {
    'approve-permission': rpc.method({
      description: 'Approve or deny a pending permission request',
      input: z.object({
        requestId: z.string(),
        approved: z.boolean(),
        decision: z.union([z.string(), z.record(z.unknown())]).optional(),
      }),
      handler: async ({ requestId, approved, decision }) => {
        const deferred = pendingApprovals.get(requestId);
        if (!deferred) {
          throw new Error('No pending approval for this request');
        }
        deferred.resolve({ approved, decision });
        return {};
      },
    }),
    'answer-question': rpc.method({
      description: 'Answer a pending question request',
      input: z.object({
        requestId: z.string(),
        answers: z.record(z.string()),
      }),
      handler: async ({ requestId, answers }) => {
        const deferred = pendingQuestions.get(requestId);
        if (deferred) {
          deferred.resolve(answers);
        }
        return {};
      },
    }),
    'send-message': rpc.method({
      description: 'Send a text message to the agent',
      input: z.object({
        personaId: z.string(),
        text: z.string().min(1),
        contextSessionUuid: z.string().optional(),
      }),
      handler: async ({ personaId, text, contextSessionUuid }) => {
        let replayEntries: PersistLogEntry[] = [];
        if (!activeSessions.has(personaId)) {
          replayEntries = await startSession(personaId, contextSessionUuid);
        }
        const queue = textQueues.get(personaId);
        if (!queue) {
          throw new Error('No active session');
        }
        await interruptSession(personaId);
        queue.clear();
        queue.push(text);
        return { replayEntries };
      },
    }),
    'set-text-focus': rpc.method({
      description: 'Report whether an editable element currently has focus in the WebView',
      input: z.object({
        personaId: z.string(),
        focused: z.boolean(),
      }),
      handler: async ({ personaId, focused }) => {
        if (focused) {
          textFocusedPersonas.add(personaId);
        } else {
          textFocusedPersonas.delete(personaId);
        }
        return {};
      },
    }),
    status: rpc.method({
      description: 'Get the current session state for all personas',
      handler: async () => {
        const result: Record<string, string> = {};
        for (const [id] of activeSessions) {
          result[id] = 'active';
        }
        return result;
      },
    }),
    'get-session-status': rpc.method({
      description: 'Get the session status for a specific persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        const status = activeSessions.has(personaId) ? 'active' : 'idle';
        return { status } as const;
      },
    }),
    'start-session': rpc.method({
      description: 'Manually start an agent session for a persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        const replayEntries = await startSession(personaId);
        return { replayEntries };
      },
    }),
    'stop-session': rpc.method({
      description: 'Stop an active agent session for a persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        await stopSession(personaId);
        return {};
      },
    }),
    'interrupt-session': rpc.method({
      description: 'Interrupt the current agent execution for a persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        await interruptSession(personaId);
        return {};
      },
    }),
    'list-worktrees': rpc.method({
      description: 'List worktrees for a workspace',
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
    'add-worktree': rpc.method({
      description: 'Create a new worktree in a workspace',
      input: z.object({
        workspacePath: z.string(),
        name: z
          .string()
          .min(1)
          .regex(
            WORKTREE_NAME_PATTERN,
            'Name must contain only alphanumeric characters, hyphens, underscores, and dots',
          ),
        branch: z.string().min(1),
      }),
      handler: async ({ workspacePath, name, branch }) => {
        const manager = new WorktreeManager(workspacePath);
        const info = await manager.create(name, branch);
        return { worktree: info };
      },
    }),
    'remove-worktree': rpc.method({
      description: 'Remove a worktree (optionally merge first)',
      input: z.object({
        workspacePath: z.string(),
        name: z.string(),
        action: z.enum(['remove', 'merge']),
      }),
      handler: async ({ workspacePath, name, action }) => {
        const manager = new WorktreeManager(workspacePath);
        if (action === 'merge') {
          const result = await manager.merge(name);
          if (!result.success) {
            throw new Error(result.error);
          }
          return {};
        }
        await manager.remove(name);
        return {};
      },
    }),
    'list-branches': rpc.method({
      description: 'List git branches for a workspace',
      input: z.object({ workspacePath: z.string() }),
      handler: async ({ workspacePath }) => {
        const branches = await listBranches(workspacePath);
        const current = await currentBranch(workspacePath);
        return { branches, current };
      },
    }),
    'worktree-status': rpc.method({
      description: 'Get status of a specific worktree',
      input: z.object({ workspacePath: z.string(), name: z.string() }),
      handler: async ({ workspacePath, name }) => {
        const manager = new WorktreeManager(workspacePath);
        return await manager.status(name);
      },
    }),
    'get-current-branch': rpc.method({
      description: 'Resolve the current git branch for a workspace',
      input: z.object({
        workspacePath: z.string(),
        worktreeName: z.string().nullable(),
      }),
      handler: async ({ workspacePath, worktreeName }) => {
        const branchName = await resolveCurrentBranch(workspacePath, worktreeName);
        if (!branchName) {
          throw new Error('Not a git repository or branch could not be resolved');
        }
        return { branchName };
      },
    }),
    'list-sessions': rpc.method({
      description: 'List past sessions for a persona on a branch',
      input: z.object({
        workspacePath: z.string(),
        personaId: z.string(),
        branchName: z.string(),
      }),
      handler: async ({ workspacePath, personaId, branchName }) => {
        const sessions = await persistence.list(workspacePath, personaId, branchName);
        return { sessions };
      },
    }),
    'get-session-logs': rpc.method({
      description: 'Read the full log entries for a past session',
      input: z.object({
        workspacePath: z.string(),
        personaId: z.string(),
        branchName: z.string(),
        uuid: z.string(),
      }),
      handler: async ({ workspacePath, personaId, branchName, uuid }) => {
        const entries = await persistence.read(workspacePath, personaId, branchName, uuid);
        return { entries };
      },
    }),
  };
}

async function shutdown(): Promise<void> {
  console.log('[agent] Shutting down...');
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
    console.warn('[agent] API key not configured. Sessions require an API key.');
  }

  await startKeyboardHook();
  await scanOrphanedWorktrees();
  await rpc.serve({ methods: buildRpcMethods() });
}

main().catch((err) => console.error('[agent] Fatal startup error:', err));

process.once('SIGTERM', () => {
  shutdown().catch((err) => console.error('[agent] Shutdown error:', err));
});
