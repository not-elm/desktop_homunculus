import { signals } from "@hmcs/sdk";

interface PendingRequest {
  resolve: (result: PermissionResult | QuestionResult) => void;
  timeout: ReturnType<typeof setTimeout>;
  suggestions?: unknown[];
}

interface PermissionResult {
  behavior: "allow" | "deny";
  updatedPermissions?: unknown[];
  message?: string;
}

interface QuestionResult {
  behavior: "allow" | "deny";
  updatedInput?: { answers: Record<string, string> };
  message?: string;
}

/** Handler function signature for Claude Agent SDK's canUseTool callback. */
export type PermissionHandler = (
  toolName: string,
  input: unknown,
  options?: { toolUseID?: string; suggestions?: unknown[] },
) => Promise<PermissionResult | QuestionResult>;

const DEFAULT_TIMEOUT_MS = 60_000;

/**
 * Bridges Claude Agent SDK permission/question requests to UI signals and
 * external resolution channels (e.g. voice approval via PttAdapter).
 *
 * Has zero STT dependency — voice approval is wired externally by service.ts
 * through the `onPermissionWaitStart` / `onPermissionResolved` callbacks and
 * the `resolveExternally()` method.
 */
export class PermissionBridge {
  private pending = new Map<string, PendingRequest>();
  private timeoutMs = DEFAULT_TIMEOUT_MS;

  /** Called when a permission request starts waiting. */
  onPermissionWaitStart?: (requestId: string) => void;

  /** Called when a permission request is resolved by any channel. */
  onPermissionResolved?: () => void;

  /** Create the `canUseTool` callback for Claude Agent SDK. */
  createHandler(characterId: string): PermissionHandler {
    return async (toolName, input, options) => {
      const requestId = options?.toolUseID ?? crypto.randomUUID();
      if (toolName === "AskUserQuestion") {
        return this.handleQuestion(characterId, input, requestId);
      }
      return this.handlePermission(
        characterId,
        toolName,
        input,
        requestId,
        options?.suggestions,
      );
    };
  }

  /** Resolve a permission request from the UI (RPC call). */
  resolvePermission(requestId: string, approved: boolean): void {
    const pending = this.pending.get(requestId);
    if (!pending) return;
    this.finalizePending(requestId);
    pending.resolve(permissionResult(approved, pending.suggestions));
  }

  /** Resolve a question request from the UI (RPC call). */
  resolveQuestion(requestId: string, answers: Record<string, string>): void {
    const pending = this.pending.get(requestId);
    if (!pending) return;
    clearTimeout(pending.timeout);
    this.pending.delete(requestId);
    pending.resolve({ behavior: "allow", updatedInput: { answers } });
  }

  /** Resolve a permission request from an external source (e.g. voice). */
  resolveExternally(requestId: string, approved: boolean): void {
    const pending = this.pending.get(requestId);
    if (!pending) return;
    this.finalizePending(requestId);
    const message = approved ? undefined : "Denied by voice";
    pending.resolve(permissionResult(approved, pending.suggestions, message));
  }

  /** Cancel all pending requests (called on session stop). */
  cancelAll(): void {
    for (const [, pending] of this.pending) {
      clearTimeout(pending.timeout);
      pending.resolve({ behavior: "deny", message: "Session interrupted" });
    }
    this.pending.clear();
  }

  private async handlePermission(
    characterId: string,
    toolName: string,
    input: unknown,
    requestId: string,
    suggestions?: unknown[],
  ): Promise<PermissionResult> {
    signalPermissionRequest(characterId, requestId, toolName, input);
    this.onPermissionWaitStart?.(requestId);
    return this.awaitResolution(requestId, suggestions);
  }

  private async handleQuestion(
    characterId: string,
    input: unknown,
    requestId: string,
  ): Promise<QuestionResult> {
    const questions = extractQuestions(input);
    signals.send("agent:question", { characterId, requestId, questions });
    return this.awaitQuestionResolution(requestId);
  }

  private awaitResolution(
    requestId: string,
    suggestions?: unknown[],
  ): Promise<PermissionResult> {
    return new Promise((resolve) => {
      const timeout = setTimeout(() => {
        this.pending.delete(requestId);
        this.onPermissionResolved?.();
        resolve({ behavior: "deny", message: "Permission request timed out" });
      }, this.timeoutMs);
      this.pending.set(requestId, { resolve, timeout, suggestions });
    });
  }

  private awaitQuestionResolution(requestId: string): Promise<QuestionResult> {
    return new Promise((resolve) => {
      const timeout = setTimeout(() => {
        this.pending.delete(requestId);
        resolve({ behavior: "deny", message: "Question timed out" });
      }, this.timeoutMs);
      this.pending.set(requestId, { resolve, timeout });
    });
  }

  private finalizePending(requestId: string): void {
    const pending = this.pending.get(requestId);
    if (!pending) return;
    clearTimeout(pending.timeout);
    this.pending.delete(requestId);
    this.onPermissionResolved?.();
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

function permissionResult(
  approved: boolean,
  suggestions?: unknown[],
  message?: string,
): PermissionResult {
  return approved
    ? { behavior: "allow", updatedPermissions: suggestions }
    : { behavior: "deny", message: message ?? "User denied permission" };
}

function extractQuestions(input: unknown): unknown {
  return (input as Record<string, unknown>).questions;
}
