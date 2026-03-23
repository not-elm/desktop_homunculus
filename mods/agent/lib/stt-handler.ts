import { stt } from "@hmcs/sdk";
import {
  matchWakeWord,
  normalizePhrase,
  type NormalizedPhrase,
} from "@hmcs/sdk/wake-word-matcher";

export type SttState = "idle" | "ptt_active" | "session_active" | "permission_wait";

export interface SttHandlerConfig {
  wakeWordPhrases: NormalizedPhrase[];
  shutdownPhrases: NormalizedPhrase[];
  approvalPhrases: NormalizedPhrase[];
  denyPhrases: NormalizedPhrase[];
  characterId: string;
}

interface AccumulationBuffer {
  characterId: string;
  texts: string[];
}

export class SttHandler {
  private state: SttState = "idle";
  private stream: stt.SttStream | null = null;
  private buffer: AccumulationBuffer | null = null;
  private permissionResolver: ((approved: boolean) => void) | null = null;
  private configs = new Map<string, SttHandlerConfig>();
  private sessionActiveCharacterId: string | null = null;
  private previousState: SttState = "idle";

  onWakeWord: ((characterId: string) => void) | null = null;
  onShutdownWord: ((characterId: string) => void) | null = null;
  onTextReady: ((characterId: string, text: string) => void) | null = null;

  registerCharacter(config: SttHandlerConfig): void {
    this.configs.set(config.characterId, config);
  }

  unregisterCharacter(characterId: string): void {
    this.configs.delete(characterId);
  }

  async start(): Promise<void> {
    const status = await stt.session.status();
    if (status.state === "idle") {
      await stt.session.start();
    }
    this.stream = stt.stream({
      onResult: (result) => this.handleResult(result),
      onStopped: () => {},
    });
  }

  close(): void {
    this.stream?.close();
    this.stream = null;
  }

  getState(): SttState {
    return this.state;
  }

  startAccumulating(characterId: string): void {
    this.state = "ptt_active";
    this.buffer = { characterId, texts: [] };
  }

  async stopAccumulating(): Promise<string> {
    // Wait for final STT results after key release (VAD pipeline latency)
    await new Promise((r) => setTimeout(r, 2_000));
    const text = this.buffer?.texts.join(" ") ?? "";
    this.buffer = null;
    this.state = "idle";
    return text;
  }

  enterPermissionWait(): Promise<boolean> {
    if (this.state !== "permission_wait") {
      this.previousState = this.state;
    }
    this.state = "permission_wait";
    return new Promise((resolve) => {
      this.permissionResolver = resolve;
    });
  }

  exitPermissionWait(): void {
    this.state = this.previousState;
    this.permissionResolver = null;
  }

  enterSessionActive(characterId: string): void {
    if (this.state === "session_active") {
      console.warn(`[stt-handler] Session already active for ${this.sessionActiveCharacterId}, overwriting with ${characterId}`);
    }
    this.state = "session_active";
    this.sessionActiveCharacterId = characterId;
  }

  exitSessionActive(): void {
    this.state = "idle";
    this.sessionActiveCharacterId = null;
  }

  private handleResult(result: stt.SttResult): void {
    switch (this.state) {
      case "idle":
        this.checkWakeWords(result.text);
        break;
      case "ptt_active":
        this.handlePttResult(result.text);
        break;
      case "permission_wait":
        this.handlePermissionResult(result.text);
        break;
      case "session_active":
        this.handleSessionActiveResult(result.text);
        break;
    }
  }

  private checkWakeWords(text: string): void {
    for (const [characterId, config] of this.configs) {
      const match = matchWakeWord(text, config.wakeWordPhrases, "normal");
      if (match) {
        this.onWakeWord?.(characterId);
        return;
      }
    }
  }

  private handlePttResult(text: string): void {
    if (!this.buffer) return;
    if (this.isShutdownWord(text, this.buffer.characterId)) {
      this.onShutdownWord?.(this.buffer.characterId);
      this.buffer = null;
      this.state = "idle";
      return;
    }
    this.buffer.texts.push(text);
  }

  private handleSessionActiveResult(text: string): void {
    if (!this.sessionActiveCharacterId) return;
    if (this.isShutdownWord(text, this.sessionActiveCharacterId)) {
      this.onShutdownWord?.(this.sessionActiveCharacterId);
      this.exitSessionActive();
      return;
    }
    this.onTextReady?.(this.sessionActiveCharacterId, text);
  }

  private isShutdownWord(text: string, characterId: string): boolean {
    const config = this.configs.get(characterId);
    if (!config) return false;
    return matchWakeWord(text, config.shutdownPhrases, "normal") !== null;
  }

  private handlePermissionResult(text: string): void {
    if (!this.permissionResolver) return;
    for (const [, config] of this.configs) {
      if (matchWakeWord(text, config.approvalPhrases, "normal")) {
        this.permissionResolver(true);
        this.exitPermissionWait();
        return;
      }
      if (matchWakeWord(text, config.denyPhrases, "normal")) {
        this.permissionResolver(false);
        this.exitPermissionWait();
        return;
      }
    }
  }
}

export { normalizePhrase };
