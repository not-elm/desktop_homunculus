import { signals } from "@hmcs/sdk";
import type { SttHandler } from "./stt-handler.ts";

interface PendingRequest {
  resolve: (result: PermissionResult | QuestionResult) => void;
  timeout: ReturnType<typeof setTimeout>;
}

interface PermissionResult {
  behavior: "allow" | "deny";
  message?: string;
}

interface QuestionResult {
  behavior: "allow" | "deny";
  updatedInput?: { answers: Record<string, string> };
  message?: string;
}

const DEFAULT_TIMEOUT_MS = 60_000;

export class PermissionBridge {
  private pending = new Map<string, PendingRequest>();
  private sttHandler: SttHandler;
  private timeoutMs = DEFAULT_TIMEOUT_MS;

  constructor(sttHandler: SttHandler) {
    this.sttHandler = sttHandler;
  }

  createHandler(characterId: string) {
    return async (
      toolName: string,
      input: unknown,
      options?: { toolUseID?: string },
    ) => {
      const requestId = options?.toolUseID ?? crypto.randomUUID();
      if (toolName === "AskUserQuestion") {
        return this.handleAskUserQuestion(characterId, input, requestId);
      }
      return this.handlePermissionRequest(characterId, toolName, input, requestId);
    };
  }

  resolvePermission(requestId: string, approved: boolean): void {
    const pending = this.pending.get(requestId);
    if (!pending) return;
    clearTimeout(pending.timeout);
    this.pending.delete(requestId);
    this.sttHandler.exitPermissionWait();
    pending.resolve(permissionResult(approved));
  }

  resolveQuestion(requestId: string, answers: Record<string, string>): void {
    const pending = this.pending.get(requestId);
    if (!pending) return;
    clearTimeout(pending.timeout);
    this.pending.delete(requestId);
    pending.resolve({ behavior: "allow", updatedInput: { answers } });
  }

  cancelAll(): void {
    for (const [, pending] of this.pending) {
      clearTimeout(pending.timeout);
      pending.resolve({ behavior: "deny", message: "Session interrupted" });
    }
    this.pending.clear();
    this.sttHandler.exitPermissionWait();
  }

  private async handlePermissionRequest(
    characterId: string,
    toolName: string,
    input: unknown,
    requestId: string,
  ): Promise<PermissionResult> {
    signalPermissionRequest(characterId, requestId, toolName, input);
    const voicePromise = this.sttHandler.enterPermissionWait();
    return Promise.race([
      this.createPermissionUiPromise(requestId),
      this.wrapVoicePromise(requestId, voicePromise),
    ]);
  }

  private createPermissionUiPromise(requestId: string): Promise<PermissionResult> {
    return new Promise((resolve) => {
      const timeout = setTimeout(() => {
        this.pending.delete(requestId);
        this.sttHandler.exitPermissionWait();
        resolve({ behavior: "deny", message: "Permission request timed out" });
      }, this.timeoutMs);
      this.pending.set(requestId, { resolve, timeout });
    });
  }

  private async wrapVoicePromise(
    requestId: string,
    voicePromise: Promise<boolean>,
  ): Promise<PermissionResult> {
    const approved = await voicePromise;
    const pending = this.pending.get(requestId);
    if (pending) {
      clearTimeout(pending.timeout);
      this.pending.delete(requestId);
    }
    return permissionResult(approved, approved ? undefined : "Denied by voice");
  }

  private async handleAskUserQuestion(
    characterId: string,
    input: unknown,
    requestId: string,
  ): Promise<QuestionResult> {
    const questions = extractQuestions(input);
    signals.send("agent:question", { characterId, requestId, questions });
    return new Promise((resolve) => {
      const timeout = setTimeout(() => {
        this.pending.delete(requestId);
        resolve({ behavior: "deny", message: "Question timed out" });
      }, this.timeoutMs);
      this.pending.set(requestId, { resolve, timeout });
    });
  }
}

function signalPermissionRequest(
  characterId: string,
  requestId: string,
  toolName: string,
  input: unknown,
): void {
  signals.send("agent:permission", {
    characterId,
    requestId,
    action: toolName,
    target: JSON.stringify(input),
  });
}

function permissionResult(approved: boolean, message?: string): PermissionResult {
  return approved
    ? { behavior: "allow" }
    : { behavior: "deny", message: message ?? "User denied permission" };
}

function extractQuestions(input: unknown): unknown {
  return (input as Record<string, unknown>).questions;
}
