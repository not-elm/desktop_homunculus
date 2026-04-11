/**
 * A promise whose resolve/reject can be called externally.
 *
 * Used to create pending promises in canUseTool callbacks that are
 * resolved later when the main loop sends back a response via generator.next().
 */
export class Deferred<T> {
  readonly promise: Promise<T>;
  resolve!: (value: T) => void;
  reject!: (reason?: unknown) => void;

  constructor() {
    this.promise = new Promise<T>((resolve, reject) => {
      this.resolve = resolve;
      this.reject = reject;
    });
  }
}

/**
 * Async queue that bridges push-based producers with pull-based consumers.
 *
 * `canUseTool` pushes items; `mergeStreams` shifts them one at a time.
 * Also tracks Deferred instances so all pending work can be rejected on interrupt.
 */
export class AsyncQueue<T> {
  private items: T[] = [];
  private waiters: Array<{
    resolve: (item: T) => void;
    reject: (reason?: unknown) => void;
  }> = [];
  private deferreds: Set<Deferred<unknown>> = new Set();

  /** Enqueue an item. If someone is awaiting shift(), resolve immediately. */
  push(item: T): void {
    const waiter = this.waiters.shift();
    if (waiter) {
      waiter.resolve(item);
    } else {
      this.items.push(item);
    }
  }

  /** Remove all queued items without notifying waiters. */
  clear(): void {
    this.items = [];
  }

  /** Dequeue the next item. If the queue is empty, wait until one arrives. */
  shift(signal: AbortSignal): Promise<T> {
    const queued = this.items.shift();
    if (queued !== undefined) return Promise.resolve(queued);

    return new Promise<T>((resolve, reject) => {
      this.waiters.push({ resolve, reject });
      signal.addEventListener('abort', () => reject(signal.reason), {
        once: true,
      });
    });
  }

  /** Register a Deferred for bulk cleanup on interrupt. */
  trackDeferred(deferred: Deferred<unknown>): void {
    this.deferreds.add(deferred);
    deferred.promise.finally(() => this.deferreds.delete(deferred)).catch(() => {});
  }

  /** Reject all tracked deferreds and pending waiters. Called during interrupt/cleanup. */
  rejectAll(reason?: unknown): void {
    for (const d of this.deferreds) {
      d.reject(reason);
    }
    this.deferreds.clear();

    for (const w of this.waiters) {
      w.reject(reason);
    }
    this.waiters = [];
  }
}
