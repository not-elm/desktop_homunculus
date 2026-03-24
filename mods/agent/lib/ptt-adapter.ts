import { stt } from "@hmcs/sdk";
import { signals } from "@hmcs/sdk";
import type { InputAdapter, SDKUserMessage } from "./input-adapter.ts";
import type { KeyboardHookService } from "./keyboard-hook.ts";
import type { ResolvedPttKey } from "./key-mapping.ts";

/** Checks if the primary key and all required modifiers (Left or Right) are held. */
function isComboHeld(
  pressedKeys: ReadonlySet<number>,
  key: ResolvedPttKey,
): boolean {
  if (!pressedKeys.has(key.primaryKeycode)) return false;
  return key.modifiers.every((keycodes) =>
    keycodes.some((kc) => pressedKeys.has(kc)),
  );
}

/**
 * Evaluates what transition should occur given the current key state.
 *
 * Pure function exported for testability.
 * - Combo pressed while idle: `"start"` recognition.
 * - Combo pressed while recording: `"cancel"` recognition.
 * - Otherwise: `"none"`.
 */
export function evaluateTransition(
  pressedKeys: ReadonlySet<number>,
  resolvedKey: ResolvedPttKey,
  isCurrentlyRecording: boolean,
): "start" | "cancel" | "none" {
  if (!isComboHeld(pressedKeys, resolvedKey)) return "none";
  return isCurrentlyRecording ? "cancel" : "start";
}

/** Creates the default recording-change callback that broadcasts via signals. */
function defaultRecordingSignal(
  characterId: string,
): (recording: boolean) => void {
  return (recording) => {
    signals.send("agent:recording", { characterId, recording });
  };
}

/** Wraps recognized text into an `SDKUserMessage`. */
function toUserMessage(text: string): SDKUserMessage {
  return { type: "user", message: { role: "user", content: text } };
}

/**
 * Returns a promise that resolves when `evaluateTransition` yields a
 * non-`"none"` result. Rejects when the session-level signal aborts.
 */
function waitForKeyTransition(
  hook: KeyboardHookService,
  resolvedKey: ResolvedPttKey,
  isRecording: boolean,
  signal: AbortSignal,
): Promise<"start" | "cancel"> {
  return new Promise<"start" | "cancel">((resolve, reject) => {
    if (signal.aborted) {
      reject(signal.reason);
      return;
    }

    const unsubscribe = hook.subscribeCombo({
      onKeyEvent(pressedKeys) {
        const transition = evaluateTransition(
          pressedKeys,
          resolvedKey,
          isRecording,
        );
        if (transition === "none") return;
        cleanup();
        resolve(transition);
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

/**
 * Bridges PTT keyboard events and single-shot `stt.recognize()` calls into an
 * `AsyncGenerator` of `SDKUserMessage`.
 *
 * Uses a press-to-start / press-to-cancel model:
 * - First key press starts recognition.
 * - Second key press during recognition cancels it.
 */
export class PttAdapter implements InputAdapter {
  private mode: "normal" | "permission_wait" = "normal";
  private permissionCallback: ((text: string) => void) | null = null;
  private currentAbort: AbortController | null = null;
  private isRecording = false;
  private onRecordingChange: (recording: boolean) => void;

  constructor(
    private keyboardHook: KeyboardHookService,
    private resolvedKey: ResolvedPttKey,
    private characterId: string,
    onRecordingChange?: (recording: boolean) => void,
  ) {
    this.onRecordingChange =
      onRecordingChange ?? defaultRecordingSignal(characterId);
  }

  /** Switches between normal (yield messages) and permission-wait (callback) modes. */
  setMode(mode: "normal"): void;
  setMode(mode: "permission_wait", callback: (text: string) => void): void;
  setMode(
    mode: "normal" | "permission_wait",
    callback?: (text: string) => void,
  ): void {
    this.mode = mode;
    this.permissionCallback = callback ?? null;
  }

  async *createAsyncGenerator(
    signal: AbortSignal,
  ): AsyncGenerator<SDKUserMessage> {
    try {
      while (!signal.aborted) {
        const text = await this.runOneRecognitionCycle(signal);
        if (text === null) continue;

        if (this.mode === "permission_wait" && this.permissionCallback) {
          this.permissionCallback(text);
        } else {
          yield toUserMessage(text);
        }
      }
    } catch (e) {
      if (!isAbortError(e)) throw e;
    }
  }

  /**
   * Runs one full cycle: wait for start key, then race recognition against
   * a cancel key press. Returns the recognized text, or `null` on cancel/empty.
   */
  private async runOneRecognitionCycle(
    signal: AbortSignal,
  ): Promise<string | null> {
    await waitForKeyTransition(
      this.keyboardHook,
      this.resolvedKey,
      false,
      signal,
    );
    return await this.recognizeWithCancelSupport(signal);
  }

  /**
   * Starts recognition and simultaneously listens for a cancel key press.
   * If the cancel key is pressed during recognition, aborts the in-flight call.
   */
  private async recognizeWithCancelSupport(
    sessionSignal: AbortSignal,
  ): Promise<string | null> {
    this.currentAbort = new AbortController();
    this.setRecordingState(true);

    try {
      this.listenForCancelKey(sessionSignal);
      const text = await this.callRecognize(sessionSignal);
      return text?.trim() || null;
    } catch (e) {
      if (isAbortError(e)) return null;
      throw e;
    } finally {
      this.setRecordingState(false);
      this.currentAbort = null;
    }
  }

  /**
   * Spawns a non-blocking listener for the cancel key press.
   * When detected, aborts the current recognition via `currentAbort`.
   */
  private listenForCancelKey(sessionSignal: AbortSignal): void {
    waitForKeyTransition(
      this.keyboardHook,
      this.resolvedKey,
      true,
      sessionSignal,
    ).then((transition) => {
      if (transition === "cancel") this.cancelRecognition();
    }).catch(() => {
      // Session aborted — recognition will also be aborted via combined signal
    });
  }

  /** Calls `stt.recognize` with a combined abort signal (per-recognition + session). */
  private async callRecognize(sessionSignal: AbortSignal): Promise<string> {
    const combined = AbortSignal.any([
      this.currentAbort!.signal,
      sessionSignal,
    ]);
    const result = await stt.recognize({ language: "ja" }, combined);
    return result.text;
  }

  /** Aborts the current in-flight recognition, if any. */
  private cancelRecognition(): void {
    this.currentAbort?.abort();
    this.currentAbort = null;
  }

  /** Updates the internal recording flag and notifies the callback. */
  private setRecordingState(recording: boolean): void {
    this.isRecording = recording;
    this.onRecordingChange(recording);
  }
}

/** Returns `true` if the given error is an `AbortError` (from `AbortController.abort()`). */
function isAbortError(e: unknown): boolean {
  return e instanceof DOMException && e.name === "AbortError";
}
