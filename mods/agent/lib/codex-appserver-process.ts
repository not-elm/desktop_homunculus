/**
 * Singleton child process wrapper for `codex --app-server` over stdio.
 *
 * Manages the Codex AppServer process lifecycle, JSON-RPC 2.0 message routing,
 * request/response correlation, and per-thread handler dispatch. The executor
 * class ({@link CodexAppServerExecuter}) registers thread handlers and sends
 * requests through this shared process.
 *
 * @module
 */

import { spawn, type ChildProcess } from "node:child_process";
import { createRequire } from "node:module";
import { createInterface, type Interface as ReadlineInterface } from "node:readline";
import { Deferred } from "./async-queue.ts";
import type {
  RequestId,
  JsonRpcRequest,
  JsonRpcNotification,
  JsonRpcResponse,
  InitializeResponse,
} from "./codex-appserver-types.ts";

/**
 * Callback interface registered per-threadId for dispatching server messages.
 *
 * Each active thread registers a handler so that server-initiated requests
 * (approval, elicitation) and notifications (turn events) are routed to the
 * correct executor instance.
 */
export interface ThreadHandler {
  /** Called when the server sends a request requiring a response (approval/elicitation). */
  onServerRequest(method: string, id: RequestId, params: unknown): void;
  /** Called when the server sends a notification (turn events, deltas). */
  onServerNotification(method: string, params: unknown): void;
  /** Called when the child process exits unexpectedly or encounters an error. */
  onProcessError(error: Error): void;
}

/**
 * Manages a single `codex --app-server` child process over stdio JSON-RPC 2.0.
 *
 * This class is designed to be used as a singleton shared across multiple
 * concurrent threads (executor instances). It handles:
 * - Lazy, idempotent process startup with initialize/initialized handshake
 * - Client request/response correlation via pending promise map
 * - Per-thread handler dispatch for server-initiated requests and notifications
 * - Graceful shutdown and cleanup on process exit
 */
export class CodexAppServerProcess {
  private process: ChildProcess | null = null;
  private readline: ReadlineInterface | null = null;
  private initializePromise: Promise<void> | null = null;
  private nextId = 1;
  private readonly pendingRequests = new Map<RequestId, Deferred<unknown>>();
  private readonly threadHandlers = new Map<string, ThreadHandler>();

  /** Number of currently registered thread handlers. */
  get refCount(): number {
    return this.threadHandlers.size;
  }

  /**
   * Ensure the AppServer process is running and initialized.
   *
   * Spawns the child process and performs the `initialize`/`initialized`
   * handshake on first call. Subsequent calls return immediately if the
   * process is already running. This method is idempotent and safe to
   * call concurrently (the handshake promise is shared).
   */
  async ensureRunning(): Promise<void> {
    if (this.initializePromise) {
      return this.initializePromise;
    }
    this.initializePromise = this.spawnAndInitialize();
    try {
      await this.initializePromise;
    } catch (error) {
      this.initializePromise = null;
      throw error;
    }
  }

  /**
   * Register a thread handler for dispatching server messages.
   *
   * @param threadId - The thread identifier to route messages for.
   * @param handler - Callback interface receiving server requests and notifications.
   * @returns A cleanup function that unregisters the handler when called.
   */
  registerThread(threadId: string, handler: ThreadHandler): () => void {
    this.threadHandlers.set(threadId, handler);
    return () => {
      this.threadHandlers.delete(threadId);
    };
  }

  /**
   * Send a JSON-RPC request and await the response.
   *
   * @param method - The JSON-RPC method name.
   * @param params - Optional parameters for the request.
   * @returns A promise that resolves with the response result.
   */
  async sendRequest<T = unknown>(method: string, params?: unknown): Promise<T> {
    const id = this.nextId++;
    const message: JsonRpcRequest = { jsonrpc: "2.0", id, method, params };

    const deferred = new Deferred<unknown>();
    this.pendingRequests.set(id, deferred);

    this.writeMessage(message);

    return deferred.promise as Promise<T>;
  }

  /**
   * Send a JSON-RPC notification (fire-and-forget, no response expected).
   *
   * @param method - The JSON-RPC method name.
   * @param params - Optional parameters for the notification.
   */
  sendNotification(method: string, params?: unknown): void {
    const message: JsonRpcNotification = { jsonrpc: "2.0", method, params };
    this.writeMessage(message);
  }

  /**
   * Send a successful response to a server-initiated request.
   *
   * @param id - The request ID from the server request being responded to.
   * @param result - The result payload.
   */
  sendResponse(id: RequestId, result: unknown): void {
    const message: JsonRpcResponse = { jsonrpc: "2.0", id, result };
    this.writeMessage(message);
  }

  /**
   * Send an error response to a server-initiated request.
   *
   * @param id - The request ID from the server request being responded to.
   * @param code - JSON-RPC error code.
   * @param message - Human-readable error message.
   */
  sendErrorResponse(id: RequestId, code: number, message: string): void {
    const response: JsonRpcResponse = {
      jsonrpc: "2.0",
      id,
      error: { code, message },
    };
    this.writeMessage(response);
  }

  /**
   * Gracefully shut down the AppServer process.
   *
   * Kills the child process, rejects all pending requests, and notifies
   * all registered thread handlers of the shutdown.
   */
  shutdown(): void {
    const proc = this.process;
    this.process = null;
    this.initializePromise = null;

    this.readline?.close();
    this.readline = null;

    if (proc && !proc.killed) {
      proc.kill("SIGTERM");
    }

    this.rejectAllPending(new Error("AppServer process shut down"));
    this.notifyAllHandlersOfError(new Error("AppServer process shut down"));
  }

  private async spawnAndInitialize(): Promise<void> {
    const codexBinPath = resolveCodexBinPath();
    const proc = spawn(process.execPath, [codexBinPath, "app-server", "--listen", "stdio://"], {
      stdio: ["pipe", "pipe", "pipe"],
      env: { ...process.env },
    });

    this.process = proc;

    const rl = createInterface({ input: proc.stdout! });
    this.readline = rl;

    rl.on("line", (line) => this.handleLine(line));

    proc.stderr?.on("data", (chunk: Buffer) => {
      console.error(`[codex-appserver] stderr: ${chunk.toString().trimEnd()}`);
    });

    proc.on("error", (err) => this.handleProcessError(err));
    proc.on("exit", (code, signal) => this.handleProcessExit(code, signal));

    const initResult = await this.sendRequest<InitializeResponse>("initialize", {
      clientInfo: {
        name: "homunculus",
        version: "0.1.0",
        title: null,
      },
      capabilities: {
        experimentalApi: true,
        optOutNotificationMethods: null,
      },
    });

    console.log(
      `[codex-appserver] Initialized: ${initResult.userAgent} (${initResult.platformOs})`,
    );

    this.sendNotification("initialized");
  }

  private writeMessage(message: JsonRpcRequest | JsonRpcNotification | JsonRpcResponse): void {
    const proc = this.process;
    if (!proc?.stdin?.writable) {
      throw new Error("AppServer process is not running");
    }
    proc.stdin.write(JSON.stringify(message) + "\n");
  }

  private handleLine(line: string): void {
    const trimmed = line.trim();
    if (!trimmed) return;

    let parsed: Record<string, unknown>;
    try {
      parsed = JSON.parse(trimmed) as Record<string, unknown>;
    } catch {
      console.warn(`[codex-appserver] Failed to parse line: ${trimmed}`);
      return;
    }

    this.dispatchMessage(parsed);
  }

  private dispatchMessage(msg: Record<string, unknown>): void {
    const hasId = "id" in msg && msg.id != null;
    const hasMethod = "method" in msg && typeof msg.method === "string";

    if (hasId && !hasMethod) {
      this.handleResponse(msg as unknown as JsonRpcResponse);
    } else if (hasId && hasMethod) {
      this.handleServerRequest(msg.id as RequestId, msg.method as string, msg.params);
    } else if (!hasId && hasMethod) {
      this.handleServerNotification(msg.method as string, msg.params);
    } else {
      console.warn("[codex-appserver] Unrecognized message:", JSON.stringify(msg));
    }
  }

  private handleResponse(response: JsonRpcResponse): void {
    const deferred = this.pendingRequests.get(response.id);
    if (!deferred) {
      console.warn(`[codex-appserver] No pending request for id=${response.id}`);
      return;
    }

    this.pendingRequests.delete(response.id);

    if (response.error) {
      deferred.reject(
        new Error(`JSON-RPC error ${response.error.code}: ${response.error.message}`),
      );
    } else {
      deferred.resolve(response.result);
    }
  }

  private handleServerRequest(id: RequestId, method: string, params: unknown): void {
    const threadId = extractThreadId(params);
    const handler = threadId ? this.threadHandlers.get(threadId) : undefined;

    if (handler) {
      handler.onServerRequest(method, id, params);
    } else {
      console.warn(
        `[codex-appserver] No handler for threadId=${threadId ?? "unknown"}, auto-declining request id=${id} method=${method}`,
      );
      this.sendErrorResponse(id, -32000, "No handler registered for this thread");
    }
  }

  private handleServerNotification(method: string, params: unknown): void {
    const threadId = extractThreadId(params);
    const handler = threadId ? this.threadHandlers.get(threadId) : undefined;

    if (handler) {
      handler.onServerNotification(method, params);
    } else {
      console.debug(
        `[codex-appserver] No handler for notification threadId=${threadId ?? "unknown"} method=${method}`,
      );
    }
  }

  private handleProcessError(error: Error): void {
    console.error(`[codex-appserver] Process error:`, error.message);
    this.cleanup(error);
  }

  private handleProcessExit(code: number | null, signal: string | null): void {
    console.warn(`[codex-appserver] Process exited (code=${code}, signal=${signal})`);
    this.cleanup(new Error(`AppServer process exited (code=${code}, signal=${signal})`));
  }

  private cleanup(error: Error): void {
    this.process = null;
    this.initializePromise = null;

    this.readline?.close();
    this.readline = null;

    this.rejectAllPending(error);
    this.notifyAllHandlersOfError(error);
  }

  private rejectAllPending(error: Error): void {
    for (const deferred of this.pendingRequests.values()) {
      deferred.reject(error);
    }
    this.pendingRequests.clear();
  }

  private notifyAllHandlersOfError(error: Error): void {
    for (const handler of this.threadHandlers.values()) {
      try {
        handler.onProcessError(error);
      } catch (e) {
        console.error("[codex-appserver] Error in handler.onProcessError:", e);
      }
    }
  }
}

/**
 * Extract the `threadId` field from a JSON-RPC params object.
 *
 * Server requests and notifications include `threadId` at the top level
 * of their params. Returns `undefined` if not present or params is not
 * an object.
 */
/** Resolve the absolute path to `@openai/codex/bin/codex.js` via the SDK package. */
function resolveCodexBinPath(): string {
  const req = createRequire(import.meta.url);
  return req.resolve("@openai/codex/bin/codex.js");
}

export function extractThreadId(params: unknown): string | undefined {
  if (params != null && typeof params === "object" && "threadId" in params) {
    const value = (params as Record<string, unknown>).threadId;
    return typeof value === "string" ? value : undefined;
  }
  return undefined;
}
