import type { KeyboardHookService } from "./keyboard-hook.ts";
import type { SttHandler } from "./stt-handler.ts";

interface SDKUserMessage {
  type: "user";
  message: { role: "user"; content: string };
}

/**
 * Bridges PTT keyboard events and STT results into an AsyncGenerator of
 * SDKUserMessage for the Claude Agent SDK's streaming input mode.
 *
 * Flow: PTT key down → SttHandler starts accumulating → PTT key up →
 * wait for final STT results → yield SDKUserMessage → SDK processes it.
 */
export class PttAdapter {
  private keyboardHook: KeyboardHookService;
  private sttHandler: SttHandler;
  private keycode: number;
  private characterId: string;
  private unsubscribe: (() => void) | null = null;
  private pendingResolve: ((msg: SDKUserMessage) => void) | null = null;
  private closed = false;

  constructor(
    keyboardHook: KeyboardHookService,
    sttHandler: SttHandler,
    keycode: number,
    characterId: string,
  ) {
    this.keyboardHook = keyboardHook;
    this.sttHandler = sttHandler;
    this.keycode = keycode;
    this.characterId = characterId;
  }

  async *createAsyncGenerator(): AsyncGenerator<SDKUserMessage> {
    this.unsubscribe = this.keyboardHook.subscribe(this.keycode, {
      onPttStart: () => this.sttHandler.startAccumulating(this.characterId),
      onPttStop: () => this.handlePttStop(),
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
    const isPttActive = this.sttHandler.getState() === "ptt_active";
    if (isPttActive && this.pendingResolve) {
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
