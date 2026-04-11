import type { AgentEvent, AgentResponse, AgentRuntime } from './agent-runtime.ts';

/**
 * Test-only {@link AgentRuntime} implementation that yields a scripted
 * sequence of {@link AgentEvent} values.
 *
 * Records the input and any caller-provided {@link AgentResponse} values so
 * tests can assert on how session-manager drove the runtime.
 */
export class MockAgentRuntime implements AgentRuntime {
  lastInput: string | null = null;
  lastSessionId: string | null = null;
  readonly recordedResponses: (AgentResponse | undefined)[] = [];

  constructor(private readonly scriptedEvents: AgentEvent[]) {}

  async *execute(
    text: string,
    sessionId: string | null,
    signal: AbortSignal,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    this.lastInput = text;
    this.lastSessionId = sessionId;

    for (const event of this.scriptedEvents) {
      if (signal.aborted) return;
      const response = yield event;
      this.recordedResponses.push(response);
    }
  }
}
