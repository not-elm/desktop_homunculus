import type { InputAdapter, SDKUserMessage } from "./input-adapter.ts";
import type { SttHandler } from "./stt-handler.ts";

/**
 * Always-on mode adapter: forwards every STT result directly to the
 * Claude Agent SDK as a user message, without requiring a PTT key press.
 *
 * Lifecycle:
 * 1. `createAsyncGenerator()` puts `SttHandler` into `session_active` state
 * 2. Each STT result triggers `onTextReady`, which resolves the pending promise
 * 3. Shutdown words are checked by `SttHandler` before forwarding
 * 4. `close()` exits the generator and restores `SttHandler` to `idle`
 *
 * NOTE: `onTextReady` is a single callback — only one AlwaysOnAdapter can be
 * active at a time. Multi-character always-on mode is not supported in this
 * initial implementation. A future version should use an event emitter pattern.
 */
export class AlwaysOnAdapter implements InputAdapter {
  private sttHandler: SttHandler;
  private characterId: string;
  private pendingResolve: ((msg: SDKUserMessage) => void) | null = null;
  private closed = false;

  constructor(sttHandler: SttHandler, characterId: string) {
    this.sttHandler = sttHandler;
    this.characterId = characterId;
  }

  async *createAsyncGenerator(): AsyncGenerator<SDKUserMessage> {
    this.sttHandler.onTextReady = (characterId, text) => {
      if (characterId === this.characterId && this.pendingResolve) {
        this.pendingResolve({
          type: "user",
          message: { role: "user", content: text },
        });
        this.pendingResolve = null;
      }
    };
    this.sttHandler.enterSessionActive(this.characterId);

    try {
      while (!this.closed) {
        const msg = await this.waitForNextMessage();
        if (msg.message.content) yield msg;
      }
    } finally {
      this.sttHandler.exitSessionActive();
      this.sttHandler.onTextReady = null;
    }
  }

  close(): void {
    this.closed = true;
    this.pendingResolve?.({
      type: "user",
      message: { role: "user", content: "" },
    });
  }

  private waitForNextMessage(): Promise<SDKUserMessage> {
    return new Promise((resolve) => {
      this.pendingResolve = resolve;
    });
  }
}
