import { uIOhook } from "uiohook-napi";

export interface PttCallback {
  onPttStart(): void;
  onPttStop(): void;
}

export class KeyboardHookService {
  private pressedKeys = new Set<number>();
  private pressTimestamps = new Map<number, number>();
  private subscribers = new Map<number, Set<PttCallback>>();
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

  private handleKeyDown(keycode: number): void {
    if (this.pressedKeys.has(keycode)) return; // Debounce OS autorepeat
    this.pressedKeys.add(keycode);
    this.pressTimestamps.set(keycode, Date.now());
    this.subscribers.get(keycode)?.forEach((cb) => cb.onPttStart());
  }

  private handleKeyUp(keycode: number): void {
    if (!this.pressedKeys.has(keycode)) return;
    this.pressedKeys.delete(keycode);
    this.pressTimestamps.delete(keycode);
    this.subscribers.get(keycode)?.forEach((cb) => cb.onPttStop());
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
