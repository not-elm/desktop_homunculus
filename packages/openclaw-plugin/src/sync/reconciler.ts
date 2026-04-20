import { errorMessage } from '../util/error.js';
import type { SeedDeps } from './seed.js';
import { seedFromHmcs } from './seed.js';

const MAX_BACKOFF_MS = 5 * 60 * 1000;

/**
 * Periodically re-runs `seedFromHmcs`. The base period is the configured
 * `reconcileIntervalSec`; consecutive failures double the wait up to 5 min,
 * and a successful run resets back to the base period. Self-reschedules with
 * `setTimeout` so a slow tick cannot overlap with the next one.
 */
export function startReconciler(deps: SeedDeps): () => void {
  const baseMs = Math.max(1_000, deps.config.reconcileIntervalSec * 1000);
  let timer: ReturnType<typeof setTimeout> | null = null;
  let stopped = false;
  let failureCount = 0;

  function schedule(delayMs: number): void {
    timer = setTimeout(tick, delayMs);
  }

  async function tick(): Promise<void> {
    if (stopped) return;
    try {
      await seedFromHmcs(deps);
      failureCount = 0;
      schedule(baseMs);
    } catch (err) {
      failureCount += 1;
      const backoff = Math.min(MAX_BACKOFF_MS, baseMs * 2 ** (failureCount - 1));
      deps.logger.warn(
        `reconciler tick failed (consecutive=${failureCount}, next in ${Math.round(backoff / 1000)}s) err=${errorMessage(err)}`,
      );
      schedule(backoff);
    }
  }

  schedule(baseMs);

  return () => {
    stopped = true;
    if (timer !== null) {
      clearTimeout(timer);
      timer = null;
    }
  };
}
