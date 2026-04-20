import { describe, expect, test, vi } from 'vitest';
import { createSessionEndHandler } from './session_end.js';

function makeDebouncer() {
  return {
    push: vi.fn(),
    forceFlush: vi.fn().mockResolvedValue(undefined),
  };
}

describe('createSessionEndHandler', () => {
  test('force-flushes debouncer using sessionKey-derived key', async () => {
    const debouncer = makeDebouncer();
    const handler = createSessionEndHandler({ debouncer });
    await handler(
      { sessionKey: 'agent:elmer:slack:direct:U123', sessionId: 's1', messageCount: 3 } as never,
      { sessionId: 's1' } as never,
    );
    expect(debouncer.forceFlush).toHaveBeenCalledWith('slack:U123');
  });

  test('uses ctx.sessionKey when event.sessionKey is missing', async () => {
    const debouncer = makeDebouncer();
    const handler = createSessionEndHandler({ debouncer });
    await handler(
      { sessionId: 's1', messageCount: 3 } as never,
      { sessionId: 's1', sessionKey: 'agent:elmer:slack:direct:U123' } as never,
    );
    expect(debouncer.forceFlush).toHaveBeenCalledWith('slack:U123');
  });

  test('no-op when sessionKey is absent everywhere', async () => {
    const debouncer = makeDebouncer();
    const handler = createSessionEndHandler({ debouncer });
    await handler({ sessionId: 's1', messageCount: 3 } as never, { sessionId: 's1' } as never);
    expect(debouncer.forceFlush).not.toHaveBeenCalled();
  });

  test('no-op when sessionKey is unparseable', async () => {
    const debouncer = makeDebouncer();
    const handler = createSessionEndHandler({ debouncer });
    await handler(
      { sessionKey: 'not-a-session', sessionId: 's1', messageCount: 3 } as never,
      { sessionId: 's1' } as never,
    );
    expect(debouncer.forceFlush).not.toHaveBeenCalled();
  });
});
