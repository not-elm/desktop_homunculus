import { describe, expect, test, vi } from 'vitest';
import { createReplyDispatchHandler } from './reply_dispatch.js';

function makeLogger() {
  return { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

function makeDebouncer() {
  return {
    push: vi.fn(),
    forceFlush: vi.fn().mockResolvedValue(undefined),
  };
}

interface DispatcherStub {
  sendFinalReply: ReturnType<typeof vi.fn>;
  sendBlockReply: ReturnType<typeof vi.fn>;
  sendToolResult: ReturnType<typeof vi.fn>;
}

function makeDispatcher(): DispatcherStub {
  return {
    sendFinalReply: vi.fn().mockReturnValue(true),
    sendBlockReply: vi.fn().mockReturnValue(true),
    sendToolResult: vi.fn().mockReturnValue(true),
  };
}

const SLACK_SESSION_KEY = 'agent:elmer:slack:direct:U123';

function dispatch(
  handler: ReturnType<typeof createReplyDispatchHandler>,
  dispatcher: DispatcherStub,
  sessionKey?: string,
): void {
  handler({ sessionKey } as never, { dispatcher } as never);
}

describe('createReplyDispatchHandler', () => {
  test('wraps sendFinalReply so payload.text is pushed to debouncer with parsed agentId', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const originalFinalReply = dispatcher.sendFinalReply;
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    dispatch(handler, dispatcher, SLACK_SESSION_KEY);

    const result = dispatcher.sendFinalReply({ text: 'hello world' });

    expect(debouncer.push).toHaveBeenCalledWith('slack:U123', 'elmer', 'hello world');
    expect(originalFinalReply).toHaveBeenCalledWith({ text: 'hello world' });
    expect(result).toBe(true);
  });

  test('wraps sendBlockReply and sendToolResult identically', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    dispatch(handler, dispatcher, SLACK_SESSION_KEY);

    dispatcher.sendBlockReply({ text: 'block one' });
    dispatcher.sendToolResult({ text: 'tool out' });

    expect(debouncer.push).toHaveBeenNthCalledWith(1, 'slack:U123', 'elmer', 'block one');
    expect(debouncer.push).toHaveBeenNthCalledWith(2, 'slack:U123', 'elmer', 'tool out');
  });

  test('idempotent — wrapping the same dispatcher twice does not double-wrap', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    dispatch(handler, dispatcher, SLACK_SESSION_KEY);
    dispatch(handler, dispatcher, SLACK_SESSION_KEY);

    dispatcher.sendFinalReply({ text: 'hi' });

    expect(debouncer.push).toHaveBeenCalledTimes(1);
  });

  test('skips push when payload.text is empty, whitespace, or not a string; original is still called', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const originalFinalReply = dispatcher.sendFinalReply;
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    dispatch(handler, dispatcher, SLACK_SESSION_KEY);

    dispatcher.sendFinalReply({ text: '' });
    dispatcher.sendFinalReply({ text: '   \n  ' });
    dispatcher.sendFinalReply({ text: 42 as unknown as string });
    dispatcher.sendFinalReply({});
    dispatcher.sendFinalReply(null);

    expect(debouncer.push).not.toHaveBeenCalled();
    expect(originalFinalReply).toHaveBeenCalledTimes(5);
  });

  test('debouncer push throwing does not break the wrapped dispatcher', () => {
    const debouncer = makeDebouncer();
    debouncer.push.mockImplementation(() => {
      throw new Error('boom');
    });
    const dispatcher = makeDispatcher();
    const originalFinalReply = dispatcher.sendFinalReply;
    const logger = makeLogger();
    const handler = createReplyDispatchHandler({ debouncer, logger });
    dispatch(handler, dispatcher, SLACK_SESSION_KEY);

    const result = dispatcher.sendFinalReply({ text: 'hi' });

    expect(result).toBe(true);
    expect(originalFinalReply).toHaveBeenCalledWith({ text: 'hi' });
    expect(logger.warn).toHaveBeenCalledWith(expect.stringContaining('speak push failed'));
  });

  test('does not wrap dispatcher when sessionKey is missing', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const original = dispatcher.sendFinalReply;
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    dispatch(handler, dispatcher, undefined);

    expect(dispatcher.sendFinalReply).toBe(original);
    dispatcher.sendFinalReply({ text: 'hi' });
    expect(debouncer.push).not.toHaveBeenCalled();
  });

  test('does not wrap dispatcher when sessionKey is malformed', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const original = dispatcher.sendFinalReply;
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    dispatch(handler, dispatcher, 'bogus');

    expect(dispatcher.sendFinalReply).toBe(original);
    dispatcher.sendFinalReply({ text: 'hi' });
    expect(debouncer.push).not.toHaveBeenCalled();
  });

  test('always returns undefined so dispatch is not claimed', () => {
    const debouncer = makeDebouncer();
    const dispatcher = makeDispatcher();
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    const result = handler({ sessionKey: SLACK_SESSION_KEY } as never, { dispatcher } as never);
    expect(result).toBeUndefined();
  });

  test('no-op when ctx has no dispatcher', () => {
    const debouncer = makeDebouncer();
    const handler = createReplyDispatchHandler({ debouncer, logger: makeLogger() });
    const result = handler({ sessionKey: SLACK_SESSION_KEY } as never, {} as never);
    expect(result).toBeUndefined();
    expect(debouncer.push).not.toHaveBeenCalled();
  });
});
