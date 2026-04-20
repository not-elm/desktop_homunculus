import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { createSpeakDebouncer } from './speak-debouncer.js';

const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };

beforeEach(() => {
  vi.useFakeTimers();
  logger.debug.mockClear();
  logger.info.mockClear();
  logger.warn.mockClear();
  logger.error.mockClear();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('createSpeakDebouncer', () => {
  test('flushes once after idle delay with the accumulated text', async () => {
    const speak = vi.fn().mockResolvedValue(undefined);
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    debouncer.push('slack:U1', 'elmer', 'こんにちは、');
    debouncer.push('slack:U1', 'elmer', '元気ですか？');
    expect(speak).not.toHaveBeenCalled();
    await vi.advanceTimersByTimeAsync(300);
    expect(speak).toHaveBeenCalledTimes(1);
    expect(speak).toHaveBeenCalledWith({ agentId: 'elmer', text: 'こんにちは、元気ですか？' });
  });

  test('resets the timer on every push', async () => {
    const speak = vi.fn().mockResolvedValue(undefined);
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    debouncer.push('slack:U1', 'elmer', 'a');
    await vi.advanceTimersByTimeAsync(200);
    debouncer.push('slack:U1', 'elmer', 'b');
    await vi.advanceTimersByTimeAsync(200); // 400ms since first push but only 200ms since last
    expect(speak).not.toHaveBeenCalled();
    await vi.advanceTimersByTimeAsync(100); // now 300ms since second push
    expect(speak).toHaveBeenCalledWith({ agentId: 'elmer', text: 'ab' });
  });

  test('keys are isolated', async () => {
    const speak = vi.fn().mockResolvedValue(undefined);
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    debouncer.push('slack:U1', 'elmer', 'one');
    debouncer.push('slack:U2', 'maid', 'two');
    await vi.advanceTimersByTimeAsync(300);
    expect(speak).toHaveBeenCalledWith({ agentId: 'elmer', text: 'one' });
    expect(speak).toHaveBeenCalledWith({ agentId: 'maid', text: 'two' });
  });

  test('forceFlush fires the callback immediately and resolves when speak resolves', async () => {
    let resolveSpeak: () => void = () => {};
    const speak = vi.fn(() => new Promise<void>((r) => (resolveSpeak = r)));
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    debouncer.push('slack:U1', 'elmer', 'hi');
    const flushPromise = debouncer.forceFlush('slack:U1');
    expect(speak).toHaveBeenCalledWith({ agentId: 'elmer', text: 'hi' });
    resolveSpeak();
    await flushPromise;
  });

  test('forceFlush is a no-op when key is absent', async () => {
    const speak = vi.fn().mockResolvedValue(undefined);
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    await debouncer.forceFlush('slack:missing');
    expect(speak).not.toHaveBeenCalled();
  });

  test('push with a new agentId while an entry exists starts a fresh buffer for that agent', async () => {
    const speak = vi.fn().mockResolvedValue(undefined);
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    debouncer.push('slack:U1', 'elmer', 'first-agent ');
    debouncer.push('slack:U1', 'maid', 'second-agent');
    await vi.advanceTimersByTimeAsync(300);
    // New agent wins — the buffer was replaced, not appended
    expect(speak).toHaveBeenCalledTimes(1);
    expect(speak).toHaveBeenCalledWith({ agentId: 'maid', text: 'second-agent' });
  });

  test('logs warn when speak rejects, does not throw', async () => {
    const speak = vi.fn().mockRejectedValue(new Error('voicevox down'));
    const debouncer = createSpeakDebouncer({ delayMs: 300, speak, logger });
    debouncer.push('slack:U1', 'elmer', 'hi');
    await vi.advanceTimersByTimeAsync(300);
    await vi.runAllTimersAsync();
    await Promise.resolve();
    expect(logger.warn).toHaveBeenCalledWith(expect.stringContaining('speak failed'));
  });
});
