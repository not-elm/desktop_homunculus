import type { KeyboardHookService } from "./keyboard-hook.ts";
import type { InputAdapter, SDKUserMessage } from "./input-adapter.ts";
import type { ResolvedPttKey } from "./key-mapping.ts";
import type { SttHandler } from "./stt-handler.ts";

/**
 * Bridges PTT keyboard events and STT results into an AsyncGenerator of
 * SDKUserMessage for the Claude Agent SDK's streaming input mode.
 *
 * Uses a state-transition model: on every keydown/keyup, re-evaluates whether
 * the full combo (primary key + all modifiers) is held. Transitions from
 * inactive→active start recording, active→inactive stop recording.
 */
export class PttAdapter implements InputAdapter {
  private resolvedKey: ResolvedPttKey;
  private keyboardHook: KeyboardHookService;
  private sttHandler: SttHandler;
  private characterId: string;
  private unsubscribe: (() => void) | null = null;
  private pendingResolve: ((msg: SDKUserMessage) => void) | null = null;
  private active = false;
  private closed = false;

  constructor(
    keyboardHook: KeyboardHookService,
    sttHandler: SttHandler,
    resolvedKey: ResolvedPttKey,
    characterId: string,
  ) {
    this.keyboardHook = keyboardHook;
    this.sttHandler = sttHandler;
    this.resolvedKey = resolvedKey;
    this.characterId = characterId;
  }

  async *createAsyncGenerator(): AsyncGenerator<SDKUserMessage> {
    this.unsubscribe = this.keyboardHook.subscribeCombo({
      onKeyEvent: (pressedKeys) => this.evaluateState(pressedKeys),
    });

    try {
      while (!this.closed) {
        const msg = await this.waitForNextMessage();
        if (msg.message.content) {
          yield msg;
        }
      }
    } finally {
      this.unsubscribe?.();
      this.unsubscribe = null;
    }
  }

  close(): void {
    this.closed = true;
    this.pendingResolve?.({
      type: "user",
      message: { role: "user", content: "" },
    });
  }

  /**
   * Forces the current PTT session to flush immediately, preventing text buffer
   * loss during shutdown (call before uiohook.stop).
   */
  forceFlush(): void {
    if (this.active && this.pendingResolve) {
      this.active = false;
      this.handlePttStop();
    }
  }

  private evaluateState(pressedKeys: ReadonlySet<number>): void {
    const shouldBeActive = isComboHeld(pressedKeys, this.resolvedKey);

    if (shouldBeActive && !this.active) {
      this.active = true;
      this.sttHandler.startAccumulating(this.characterId);
    } else if (!shouldBeActive && this.active) {
      this.active = false;
      this.handlePttStop();
    }
  }

  private waitForNextMessage(): Promise<SDKUserMessage> {
    return new Promise((resolve) => {
      this.pendingResolve = resolve;
    });
  }

  private handlePttStop(): void {
    this.sttHandler.stopAccumulating().then((text) => {
      this.resolveIfTextPresent(text.trim());
    });
  }

  private resolveIfTextPresent(text: string): void {
    if (text && this.pendingResolve) {
      this.pendingResolve({
        type: "user",
        message: { role: "user", content: text },
      });
      this.pendingResolve = null;
    }
  }
}

/** Checks if the primary key and all required modifiers (Left or Right) are held. */
function isComboHeld(pressedKeys: ReadonlySet<number>, key: ResolvedPttKey): boolean {
  if (!pressedKeys.has(key.primaryKeycode)) return false;
  return key.modifiers.every((keycodes) =>
    keycodes.some((kc) => pressedKeys.has(kc)),
  );
}
