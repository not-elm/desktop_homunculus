import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { startReconciler } from './reconciler.js';
import * as seed from './seed.js';

beforeEach(() => vi.useFakeTimers());
afterEach(() => vi.useRealTimers());

function makeDeps(intervalMs: number) {
  const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
  return {
    api: { runtime: { logger } } as any,
    cache: { personas: new Map(), agents: new Map() } as any,
    config: { dhBaseUrl: 'http://x', reconcileIntervalSec: intervalMs / 1000, soulMaxChars: 10000 },
    logger,
    cli: { agentsList: vi.fn(async () => []) },
  };
}

describe('startReconciler', () => {
  test('invokes seedFromDh at each interval; stop() cancels', async () => {
    const spy = vi.spyOn(seed, 'seedFromDh').mockResolvedValue(undefined);
    const deps = makeDeps(1000);
    const stop = startReconciler(deps as any);

    await vi.advanceTimersByTimeAsync(1000);
    expect(spy).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(1000);
    expect(spy).toHaveBeenCalledTimes(2);

    stop();
    await vi.advanceTimersByTimeAsync(1000);
    expect(spy).toHaveBeenCalledTimes(2);
  });

  test('logs warn but does not throw when seed rejects', async () => {
    vi.spyOn(seed, 'seedFromDh').mockRejectedValue(new Error('boom'));
    const deps = makeDeps(1000);
    const stop = startReconciler(deps as any);
    await vi.advanceTimersByTimeAsync(1000);
    expect(deps.logger.warn).toHaveBeenCalledWith(
      expect.stringContaining('reconciler tick failed'),
    );
    stop();
  });
});
