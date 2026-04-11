import type { AgentRuntime } from './runtime/agent-runtime.ts';
import type { PersonaSessions, WorkerTask } from './types.ts';

/**
 * Tracks the set of active LLM sessions for each persona.
 *
 * Each persona has at most one Frontman (user-facing conversation) and
 * zero or more Workers (background implementation tasks). The manager
 * enforces these invariants and exposes the state that RPC handlers
 * need to answer session-status queries.
 *
 * This is a skeleton implementation. Task 3.2 moves the current
 * `runSession` event loop into a `runFrontmanLoop` method. Task 5.1
 * adds Worker delegation methods.
 */
export class SessionManager {
  private readonly sessions = new Map<string, PersonaSessions>();

  /**
   * Register a new Frontman for a persona.
   *
   * Throws if a Frontman is already running for this persona.
   */
  startFrontman(personaId: string, _runtime: AgentRuntime): void {
    const existing = this.sessions.get(personaId);
    if (existing?.frontman) {
      throw new Error(`Frontman already running for persona "${personaId}"`);
    }
    const controller = new AbortController();
    const sessions: PersonaSessions = existing ?? {
      workers: new Map<string, WorkerTask>(),
    };
    sessions.frontman = { controller, sessionId: null };
    this.sessions.set(personaId, sessions);
  }

  hasFrontman(personaId: string): boolean {
    return this.sessions.get(personaId)?.frontman !== undefined;
  }

  /**
   * Stop all sessions (Frontman + Workers) for a persona.
   *
   * Aborts all controllers and removes the persona from the tracking map.
   */
  async stopPersonaSessions(personaId: string): Promise<void> {
    const sessions = this.sessions.get(personaId);
    if (!sessions) return;

    sessions.frontman?.controller.abort();
    for (const worker of sessions.workers.values()) {
      worker.controller.abort();
    }
    this.sessions.delete(personaId);
  }

  /** Internal accessor used by future methods. */
  getPersonaSessions(personaId: string): PersonaSessions | undefined {
    return this.sessions.get(personaId);
  }
}
