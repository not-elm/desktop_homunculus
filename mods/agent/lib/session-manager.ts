import { audio, Persona as SdkPersona, signals, stt } from '@hmcs/sdk';
import { rpc } from '@hmcs/sdk/rpc';
import { AsyncQueue, Deferred } from './async-queue.ts';
import { MessageRouter } from './coordination/message-router.ts';
import { type ResolvedPttKey, resolvePttKeycodes } from './key-mapping.ts';
import { isComboHeld, type KeyboardHookService, waitForComboRelease } from './keyboard-hook.ts';
import { DEFAULT_PERMISSION_SE, resolvePermissionSeAsset } from './permission-se.ts';
import type { AgentEvent, AgentResponse, AgentRuntime } from './runtime/agent-runtime.ts';
import type { SessionHandle, SessionPersistence } from './session-persistence.ts';
import { sanitizeForTts } from './tts.ts';
import {
  type AgentSettings,
  type AgentStatus,
  DEFAULT_WORKER_LIMIT,
  type PersonaSessions,
  type WorkerTask,
} from './types.ts';

/** Options for delegating a task to a new Worker. */
export interface DelegateTaskOptions {
  personaId: string;
  description: string;
  worktreeName: string | null;
  createRuntime: () => AgentRuntime;
  /** Optional override for the concurrent-Worker limit. */
  workerLimit?: number;
}

/**
 * Public API of SessionManager used by service.ts RPC handlers:
 *
 *  - startFrontman(personaId, runtime): void
 *  - hasFrontman(personaId): boolean
 *  - listActivePersonas(): string[]
 *  - stopPersonaSessions(personaId): Promise<void>
 *  - stopAllSessions(): void
 *  - runFrontmanLoop(personaId, runtime, settings, resolvedKey): Promise<void>
 *  - attachSessionHandle(personaId, handle): void
 *  - attachTextQueue(personaId): AsyncQueue<string>
 *  - getTextQueue(personaId): AsyncQueue<string> | undefined
 *  - sendMessage(personaId, text): void (throws if no queue)
 *  - resolvePermission(requestId, response): boolean
 *  - resolveQuestion(requestId, answers): void
 *  - resolveInterrupt(personaId): void
 *  - setTextFocus(personaId, focused): void
 */
interface PendingApprovalResult {
  approved: boolean;
  message?: string;
  decision?: string | Record<string, unknown>;
}

interface UserInput {
  text: string;
  source: 'voice' | 'text';
}

interface RoundResult {
  sessionId: string | null;
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

export class SessionManager {
  private readonly sessions = new Map<string, PersonaSessions>();
  private readonly pendingApprovals = new Map<string, Deferred<PendingApprovalResult>>();
  private readonly pendingQuestions = new Map<string, Deferred<Record<string, string>>>();
  private readonly pendingInterrupts = new Map<string, Deferred<void>>();
  private readonly textQueues = new Map<string, AsyncQueue<string>>();
  private readonly textFocusedPersonas = new Set<string>();
  private readonly activeSessionHandles = new Map<string, SessionHandle>();

  constructor(
    private readonly persistence: SessionPersistence,
    private readonly keyboardHook: KeyboardHookService,
    private readonly messageRouter?: MessageRouter,
  ) {}

  startFrontman(personaId: string, _runtime: AgentRuntime): void {
    const existing = this.sessions.get(personaId);
    if (existing?.frontman) {
      throw new Error(`Frontman already running for persona "${personaId}"`);
    }
    const controller = new AbortController();
    const sessions: PersonaSessions = existing ?? {
      workers: new Map<string, WorkerTask>(),
    };
    const unsubscribePeer = this.subscribeToRouter(personaId);
    sessions.frontman = { controller, sessionId: null, unsubscribePeer };
    this.sessions.set(personaId, sessions);
  }

  private subscribeToRouter(personaId: string): (() => void) | undefined {
    if (!this.messageRouter) return undefined;
    return this.messageRouter.subscribe(personaId, (peerMsg) => {
      signals.send('agent:peer-message', { personaId, message: peerMsg });
    });
  }

  hasFrontman(personaId: string): boolean {
    return this.sessions.get(personaId)?.frontman !== undefined;
  }

  listActivePersonas(): string[] {
    const result: string[] = [];
    for (const [id, sessions] of this.sessions) {
      if (sessions.frontman) result.push(id);
    }
    return result;
  }

  getFrontmanSignal(personaId: string): AbortSignal | undefined {
    return this.sessions.get(personaId)?.frontman?.controller.signal;
  }

  getPersonaSessions(personaId: string): PersonaSessions | undefined {
    return this.sessions.get(personaId);
  }

  /** Spawn a background Worker task for the given persona. */
  async delegateTask(options: DelegateTaskOptions): Promise<{ taskId: string }> {
    const limit = options.workerLimit ?? DEFAULT_WORKER_LIMIT;
    const sessions = this.ensurePersonaSessions(options.personaId);

    const running = [...sessions.workers.values()].filter((w) => w.status === 'running').length;
    if (running >= limit) {
      throw new Error(`Worker limit (${limit}) reached for persona "${options.personaId}"`);
    }

    const taskId = `task-${crypto.randomUUID()}`;
    const controller = new AbortController();
    const task: WorkerTask = {
      taskId,
      personaId: options.personaId,
      controller,
      sessionId: null,
      status: 'running',
      worktreeName: options.worktreeName,
      description: options.description,
      startedAt: new Date().toISOString(),
      endedAt: null,
      errorMessage: null,
    };
    sessions.workers.set(taskId, task);

    const runtime = options.createRuntime();
    this.runWorkerLoop(task, runtime).catch((err) => {
      task.status = 'failed';
      task.endedAt = new Date().toISOString();
      task.errorMessage = err instanceof Error ? err.message : String(err);
    });

    return { taskId };
  }

  /** Cancel a running Worker task. */
  cancelTask(personaId: string, taskId: string): void {
    const task = this.sessions.get(personaId)?.workers.get(taskId);
    if (!task) return;
    if (task.status === 'running') {
      task.controller.abort();
      task.status = 'cancelled';
      task.endedAt = new Date().toISOString();
    }
  }

  /** Get the current status of a Worker task. */
  getTaskStatus(personaId: string, taskId: string): WorkerTask | undefined {
    return this.sessions.get(personaId)?.workers.get(taskId);
  }

  async stopPersonaSessions(personaId: string): Promise<void> {
    const sessions = this.sessions.get(personaId);
    if (!sessions) return;

    const handle = this.activeSessionHandles.get(personaId);
    if (handle) {
      await this.persistence.close(handle).catch(() => {});
      this.activeSessionHandles.delete(personaId);
    }

    sessions.frontman?.unsubscribePeer?.();
    sessions.frontman?.controller.abort();
    for (const worker of sessions.workers.values()) {
      worker.controller.abort();
    }
    const queue = this.textQueues.get(personaId);
    if (queue) {
      queue.rejectAll(new DOMException('Session stopped', 'AbortError'));
      this.textQueues.delete(personaId);
    }
    this.sessions.delete(personaId);
  }

  /** Abort every tracked session without awaiting persistence cleanup. */
  stopAllSessions(): void {
    for (const [, sessions] of this.sessions) {
      sessions.frontman?.controller.abort();
      for (const worker of sessions.workers.values()) {
        worker.controller.abort();
      }
    }
    this.sessions.clear();
  }

  attachSessionHandle(personaId: string, handle: SessionHandle): void {
    this.activeSessionHandles.set(personaId, handle);
  }

  attachTextQueue(personaId: string): AsyncQueue<string> {
    const queue = new AsyncQueue<string>();
    this.textQueues.set(personaId, queue);
    return queue;
  }

  getTextQueue(personaId: string): AsyncQueue<string> | undefined {
    return this.textQueues.get(personaId);
  }

  sendMessage(personaId: string, text: string): void {
    const queue = this.textQueues.get(personaId);
    if (!queue) {
      throw new Error('No active session');
    }
    queue.clear();
    queue.push(text);
  }

  resolvePermission(requestId: string, response: PendingApprovalResult): boolean {
    const deferred = this.pendingApprovals.get(requestId);
    if (!deferred) return false;
    deferred.resolve(response);
    return true;
  }

  resolveQuestion(requestId: string, answers: Record<string, string>): void {
    this.pendingQuestions.get(requestId)?.resolve(answers);
  }

  resolveInterrupt(personaId: string): void {
    this.pendingInterrupts.get(personaId)?.resolve();
  }

  setTextFocus(personaId: string, focused: boolean): void {
    if (focused) {
      this.textFocusedPersonas.add(personaId);
    } else {
      this.textFocusedPersonas.delete(personaId);
    }
  }

  /**
   * Main Frontman event loop. Alternates between waiting for user input and
   * executing one round of agent inference.
   */
  async runFrontmanLoop(
    personaId: string,
    runtime: AgentRuntime,
    settings: AgentSettings,
    resolvedKey: ResolvedPttKey | null,
  ): Promise<void> {
    const signal = this.getFrontmanSignal(personaId);
    if (!signal) {
      throw new Error(`No frontman registered for persona "${personaId}"`);
    }
    let sessionId: string | null = null;

    try {
      while (!signal.aborted) {
        this.emitStatus(personaId, 'listening');
        const input = await this.waitForUserInput(personaId, resolvedKey, signal);
        if (input === null) continue;

        this.emitUserLog(personaId, input.text, input.source);
        this.emitStatus(personaId, 'thinking');
        const result = await this.executeOneRound(
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
      this.textFocusedPersonas.delete(personaId);

      const handle = this.activeSessionHandles.get(personaId);
      if (handle) {
        await this.persistence.close(handle).catch(() => {});
        this.activeSessionHandles.delete(personaId);
      }

      const queue = this.textQueues.get(personaId);
      if (queue) {
        queue.rejectAll(new DOMException('Session ended', 'AbortError'));
        this.textQueues.delete(personaId);
      }
      const sessions = this.sessions.get(personaId);
      sessions?.frontman?.unsubscribePeer?.();
      this.sessions.delete(personaId);
      this.emitStatus(personaId, 'idle', 'session-ended');
    }
  }

  emitStatus(personaId: string, state: AgentStatus, reason?: string): void {
    signals.send('agent:status', { personaId, state, reason });
    console.debug(`[agent] ${personaId}: ${state}${reason ? ` (${reason})` : ''}`);
  }

  emitLog(personaId: string, type: string, message: string): void {
    const entry = { type, message, timestamp: Date.now() };
    signals.send('agent:log', { personaId, ...entry });

    const handle = this.activeSessionHandles.get(personaId);
    if (handle) this.persistence.append(handle, entry);
  }

  private emitUserLog(personaId: string, text: string, source: 'voice' | 'text'): void {
    const entry = { type: 'user' as const, message: text, timestamp: Date.now(), source };
    signals.send('agent:log', { personaId, ...entry });

    const handle = this.activeSessionHandles.get(personaId);
    if (handle) this.persistence.append(handle, entry);
  }

  private emitInterruptLog(personaId: string): void {
    this.emitLog(personaId, 'interrupt', '中断しました');
  }

  private emitRecording(personaId: string, recording: boolean): void {
    signals.send('agent:recording', { personaId, recording });
  }

  private speakText(personaId: string, text: string): void {
    const { sentences, log } = sanitizeForTts(text);
    if (sentences.length === 0) return;
    if (log.length > 0) {
      this.emitLog(personaId, 'tts-sanitize', log.join('; '));
    }
    rpc
      .call({
        modName: '@hmcs/voicevox',
        method: 'speak',
        body: { personaId, text: sentences },
      })
      .catch(() => this.emitLog(personaId, 'warning', 'TTS unavailable'));
  }

  private ensurePersonaSessions(personaId: string): PersonaSessions {
    let sessions = this.sessions.get(personaId);
    if (!sessions) {
      sessions = { workers: new Map<string, WorkerTask>() };
      this.sessions.set(personaId, sessions);
    }
    return sessions;
  }

  private async runWorkerLoop(task: WorkerTask, runtime: AgentRuntime): Promise<void> {
    const gen = runtime.execute(task.description, null, task.controller.signal);
    for await (const event of gen) {
      if (task.controller.signal.aborted) break;
      if (event.type === 'completed') {
        task.sessionId = event.sessionId;
        task.status = 'completed';
        task.endedAt = new Date().toISOString();
        return;
      }
      if (event.type === 'error') {
        task.status = 'failed';
        task.errorMessage = event.message;
        task.endedAt = new Date().toISOString();
        return;
      }
    }
  }

  private async waitForUserInput(
    personaId: string,
    resolvedKey: ResolvedPttKey | null,
    signal: AbortSignal,
  ): Promise<UserInput | null> {
    const queue = this.textQueues.get(personaId);
    if (!queue) return null;

    if (!resolvedKey) {
      const text = await queue.shift(signal);
      return { text, source: 'text' };
    }

    const inputAbort = new AbortController();
    const combined = AbortSignal.any([signal, inputAbort.signal]);

    const voicePromise = this.recognizeOneSentenceVoice(personaId, resolvedKey, combined).then(
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

  private async executeOneRound(
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
    const interruptPromise = this.waitForInterrupt(personaId, resolvedKey, sessionSignal);

    try {
      return await this.driveRuntime(
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

  private async driveRuntime(
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
        lastSessionId = await this.abortExecution(
          personaId,
          runtimeGen,
          interruptAbort,
          lastSessionId,
        );

        if (raceResult.source === 'ptt' && resolvedKey) {
          const voiceText = await this.recognizeWhileHeld(personaId, resolvedKey, sessionSignal);
          if (voiceText) {
            const queue = this.textQueues.get(personaId);
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
      response = await this.handleAgentEvent(personaId, event, settings, sessionSignal);
      if (event.type === 'completed' || event.type === 'error') {
        if (event.type === 'completed') lastSessionId = event.sessionId;
        break;
      }
    }

    await runtimeGen.return(undefined);
    return { sessionId: lastSessionId };
  }

  private async abortExecution(
    personaId: string,
    runtimeGen: AsyncGenerator<AgentEvent, void, AgentResponse | undefined>,
    interruptAbort: AbortController,
    lastSessionId: string | null,
  ): Promise<string | null> {
    interruptAbort.abort();
    await runtimeGen.return(undefined);
    this.emitInterruptLog(personaId);
    return lastSessionId;
  }

  private async handleAgentEvent(
    personaId: string,
    event: AgentEvent,
    settings: AgentSettings,
    signal: AbortSignal,
  ): Promise<AgentResponse | undefined> {
    switch (event.type) {
      case 'assistant_message':
        return this.handleAssistantMessage(personaId, event.text);
      case 'tool_use':
        return this.handleToolUse(personaId, event.summary);
      case 'permission_request':
        return await this.handlePermissionRequest(personaId, event, settings, signal);
      case 'elicitation_request':
        return await this.handleElicitationRequest(personaId, event, signal);
      case 'completed':
        return undefined;
      case 'error':
        return this.handleError(personaId, event.message);
    }
  }

  private handleAssistantMessage(personaId: string, text: string): undefined {
    this.emitStatus(personaId, 'thinking');
    this.emitLog(personaId, 'assistant', text);
    this.speakText(personaId, text);
    return undefined;
  }

  private handleToolUse(personaId: string, summary: string): undefined {
    this.emitStatus(personaId, 'executing');
    this.emitLog(personaId, 'tool', summary);
    return undefined;
  }

  private async handlePermissionRequest(
    personaId: string,
    event: AgentEvent & { type: 'permission_request' },
    settings: AgentSettings,
    signal: AbortSignal,
  ): Promise<AgentResponse> {
    this.emitStatus(personaId, 'waiting');
    return await this.resolvePermissionRequest(personaId, event, settings, signal);
  }

  private async handleElicitationRequest(
    personaId: string,
    event: AgentEvent & { type: 'elicitation_request' },
    signal: AbortSignal,
  ): Promise<AgentResponse> {
    this.emitStatus(personaId, 'waiting');
    return await this.resolveElicitation(personaId, event, signal);
  }

  private handleError(personaId: string, message: string): undefined {
    console.error(`[agent] ${personaId}: error — ${message}`);
    this.emitLog(personaId, 'error', message);
    return undefined;
  }

  private async resolvePermissionRequest(
    personaId: string,
    event: AgentEvent & { type: 'permission_request' },
    settings: AgentSettings,
    signal: AbortSignal,
  ): Promise<AgentResponse> {
    const deferred = new Deferred<PendingApprovalResult>();
    const permAbort = new AbortController();
    const combined = AbortSignal.any([signal, permAbort.signal]);

    this.pendingApprovals.set(event.requestId, deferred);

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

    this.playPermissionSe(personaId);

    signals.send('agent:permission', permissionPayload);

    const timer = setTimeout(
      () => deferred.resolve({ approved: false, message: 'Permission request timed out' }),
      60_000,
    );

    const onAbort = () => deferred.reject(signal.reason);
    signal.addEventListener('abort', onAbort, { once: true });

    const resolvedKey = resolvePttKeycodes(settings.pttKey as NonNullable<typeof settings.pttKey>);
    if (resolvedKey) {
      this.runVoiceApproval(personaId, resolvedKey, settings, combined, deferred);
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
      this.pendingApprovals.delete(event.requestId);
    }
  }

  private async playPermissionSe(personaId: string): Promise<void> {
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

  private runVoiceApproval(
    personaId: string,
    resolvedKey: ResolvedPttKey,
    settings: AgentSettings,
    signal: AbortSignal,
    deferred: Deferred<PendingApprovalResult>,
  ): void {
    (async () => {
      try {
        await this.waitForComboPress(personaId, resolvedKey, signal);
        const text = await this.recognizeWhileHeld(personaId, resolvedKey, signal);
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

  private async resolveElicitation(
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
    this.pendingQuestions.set(event.requestId, deferred);

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
      this.pendingQuestions.delete(event.requestId);
    }
  }

  private async waitForInterrupt(
    personaId: string,
    resolvedKey: ResolvedPttKey | null,
    sessionSignal: AbortSignal,
  ): Promise<'ptt' | 'ui'> {
    const pttPromise = resolvedKey
      ? this.waitForComboPress(personaId, resolvedKey, sessionSignal).then(() => 'ptt' as const)
      : new Promise<never>(() => {});

    return Promise.race([
      pttPromise,
      this.waitForInterruptRpc(personaId, sessionSignal).then(() => 'ui' as const),
    ]);
  }

  private async waitForInterruptRpc(personaId: string, signal: AbortSignal): Promise<void> {
    const deferred = new Deferred<void>();
    this.pendingInterrupts.set(personaId, deferred);

    try {
      await Promise.race([deferred.promise, abortToReject(signal)]);
    } finally {
      this.pendingInterrupts.delete(personaId);
    }
  }

  private async recognizeOneSentenceVoice(
    personaId: string,
    resolvedKey: ResolvedPttKey,
    signal: AbortSignal,
  ): Promise<string | null> {
    await this.waitForComboPress(personaId, resolvedKey, signal);
    return await this.recognizeWhileHeld(personaId, resolvedKey, signal);
  }

  private async recognizeWhileHeld(
    personaId: string,
    resolvedKey: ResolvedPttKey,
    signal: AbortSignal,
  ): Promise<string | null> {
    this.emitRecording(personaId, true);
    let session: stt.ptt.PttSession | null = null;

    try {
      session = await stt.ptt.start({ language: 'ja' });
      await waitForComboRelease(this.keyboardHook, resolvedKey, signal);
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
      this.emitRecording(personaId, false);
    }
  }

  private waitForComboPress(
    personaId: string,
    resolvedKey: ResolvedPttKey,
    signal: AbortSignal,
  ): Promise<void> {
    return new Promise((resolve, reject) => {
      if (signal.aborted) {
        reject(signal.reason);
        return;
      }

      const unsubscribe = this.keyboardHook.subscribeCombo({
        onKeyEvent: (pressedKeys) => {
          if (this.textFocusedPersonas.has(personaId)) return;
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

function suppressRejection(promise: Promise<unknown>): void {
  promise.catch(() => {});
}

function isAbortError(e: unknown): boolean {
  return e instanceof DOMException && e.name === 'AbortError';
}
