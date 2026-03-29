import { uIOhook } from "uiohook-napi";
import { type ResolvedPttKey } from "./key-mapping.ts";

export interface PttCallback {
  onPttStart(): void;
  onPttStop(): void;
}

/** Callback that receives the full set of currently pressed keys on every key event. */
export interface ComboCallback {
  onKeyEvent(pressedKeys: ReadonlySet<number>): void;
}

export class KeyboardHookService {
  private pressedKeys = new Set<number>();
  private pressTimestamps = new Map<number, number>();
  private subscribers = new Map<number, Set<PttCallback>>();
  private comboSubscribers = new Set<ComboCallback>();
  private staleKeyTimeoutMs = 30_000;
  private staleCheckInterval: ReturnType<typeof setInterval> | null = null;
  private started = false;

  start(): boolean {
    if (this.started) return true;
    try {
      uIOhook.on("keydown", (e) => this.handleKeyDown(e.keycode));
      uIOhook.on("keyup", (e) => this.handleKeyUp(e.keycode));
      uIOhook.start();
      this.started = true;
      this.staleCheckInterval = setInterval(() => this.checkStaleKeys(), 5_000);
      return true;
    } catch {
      return false;
    }
  }

  stop(): void {
    if (!this.started) return;
    if (this.staleCheckInterval) {
      clearInterval(this.staleCheckInterval);
      this.staleCheckInterval = null;
    }
    this.flushPressedKeys();
    uIOhook.stop();
    this.started = false;
  }

  subscribe(keycode: number, callback: PttCallback): () => void {
    if (!this.subscribers.has(keycode)) {
      this.subscribers.set(keycode, new Set());
    }
    this.subscribers.get(keycode)!.add(callback);
    return () => {
      this.subscribers.get(keycode)?.delete(callback);
    };
  }

  subscribeCombo(callback: ComboCallback): () => void {
    this.comboSubscribers.add(callback);
    return () => {
      this.comboSubscribers.delete(callback);
    };
  }

  private handleKeyDown(keycode: number): void {
    if (this.pressedKeys.has(keycode)) return; // Debounce OS autorepeat
    this.pressedKeys.add(keycode);
    this.pressTimestamps.set(keycode, Date.now());
    this.subscribers.get(keycode)?.forEach((cb) => cb.onPttStart());
    this.notifyComboSubscribers();
  }

  private handleKeyUp(keycode: number): void {
    if (!this.pressedKeys.has(keycode)) return;
    this.pressedKeys.delete(keycode);
    this.pressTimestamps.delete(keycode);
    this.subscribers.get(keycode)?.forEach((cb) => cb.onPttStop());
    this.notifyComboSubscribers();
  }

  private notifyComboSubscribers(): void {
    for (const cb of this.comboSubscribers) {
      cb.onKeyEvent(this.pressedKeys);
    }
  }

  // Force-flush all pressed keys before stopping to avoid stuck PTT state
  private flushPressedKeys(): void {
    for (const keycode of [...this.pressedKeys]) {
      this.handleKeyUp(keycode);
    }
  }

  // Safety valve: force keyup for keys pressed > 30 seconds (handles lost keyup events)
  private checkStaleKeys(): void {
    const now = Date.now();
    for (const [keycode, timestamp] of this.pressTimestamps) {
      if (now - timestamp > this.staleKeyTimeoutMs) {
        this.handleKeyUp(keycode);
      }
    }
  }
}

/**
 * Wait until a held combo is released (any required key lifts).
 *
 * Subscribes to combo events and resolves once `isComboHeld()` returns
 * `false` — meaning the primary key or a required modifier was released.
 */
export function waitForComboRelease(
  hook: KeyboardHookService,
  resolvedKey: ResolvedPttKey,
  signal: AbortSignal,
): Promise<void> {
  return new Promise((resolve, reject) => {
    if (signal.aborted) {
      reject(signal.reason);
      return;
    }

    const unsubscribe = hook.subscribeCombo({
      onKeyEvent(pressedKeys) {
        if (!isComboHeld(pressedKeys, resolvedKey)) {
          cleanup();
          resolve();
        }
      },
    });

    const onAbort = () => {
      cleanup();
      reject(signal.reason);
    };
    signal.addEventListener("abort", onAbort, { once: true });

    function cleanup() {
      unsubscribe();
      signal.removeEventListener("abort", onAbort);
    }
  });
}

export function isComboHeld(
  pressedKeys: ReadonlySet<number>,
  key: ResolvedPttKey,
): boolean {
  if (!pressedKeys.has(key.primaryKeycode)) return false;
  return key.modifiers.every((keycodes) =>
    keycodes.some((kc) => pressedKeys.has(kc)),
  );
}
