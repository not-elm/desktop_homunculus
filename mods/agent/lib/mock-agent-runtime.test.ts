import { describe, expect, it } from 'vitest';
import type { AgentEvent, AgentResponse } from './agent-runtime.ts';
import { MockAgentRuntime } from './mock-agent-runtime.ts';

describe('MockAgentRuntime', () => {
  it('yields scripted events in order', async () => {
    const events: AgentEvent[] = [
      { type: 'assistant_message', text: 'hello' },
      { type: 'tool_use', tool: 'bash', summary: '$ ls' },
      { type: 'completed', sessionId: 'session-abc' },
    ];
    const runtime = new MockAgentRuntime(events);

    const collected: AgentEvent[] = [];
    const gen = runtime.execute('hi', null, new AbortController().signal);
    for await (const event of gen) {
      collected.push(event);
    }

    expect(collected).toEqual(events);
  });

  it('records the last execute() arguments for assertions', async () => {
    const runtime = new MockAgentRuntime([{ type: 'completed', sessionId: 's1' }]);

    const gen = runtime.execute('my prompt', 'existing-session', new AbortController().signal);
    for await (const _event of gen) {
      // drain
    }

    expect(runtime.lastInput).toBe('my prompt');
    expect(runtime.lastSessionId).toBe('existing-session');
  });

  it('awaits a response on permission_request before proceeding', async () => {
    const events: AgentEvent[] = [
      {
        type: 'permission_request',
        requestId: 'req-1',
        tool: 'bash',
        input: { command: 'rm -rf /' },
      },
      { type: 'completed', sessionId: 's1' },
    ];
    const runtime = new MockAgentRuntime(events);
    const gen = runtime.execute('hi', null, new AbortController().signal);

    const first = await gen.next();
    expect(first.value).toEqual(events[0]);

    const response: AgentResponse = { type: 'permission', approved: false };
    const second = await gen.next(response);
    expect(second.value).toEqual(events[1]);
    expect(runtime.recordedResponses).toEqual([response]);
  });
});
