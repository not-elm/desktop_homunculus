/**
 * Managed process namespace for starting and stopping long-running MOD command processes.
 *
 * Provides lifecycle management for long-running processes with exit detection
 * via the signals WebSocket. Each {@link ProcessHandle} is `AsyncDisposable` for
 * automatic cleanup.
 *
 * @example
 * ```typescript
 * import { processes } from "@hmcs/sdk";
 *
 * const proc = await processes.start({
 *   command: "@hmcs/persona:default-behavior",
 *   args: ["persona-1"],
 * });
 *
 * proc.onExit((info) => {
 *   console.log(`Process ${info.reason}: exit code ${info.exitCode}`);
 * });
 *
 * // Stop and clean up
 * await proc.stop();
 * ```
 *
 * @packageDocumentation
 */

import { host } from './host';
import { signals } from './signals';

/** Information about why a managed process exited unexpectedly. */
export interface ProcessExitInfo {
  /** Process exit code, or null if killed by a signal. */
  exitCode: number | null;
  /** Unix signal name if killed by a signal. */
  signal: string | null;
  /** Exit reason: `"exited"` for clean exit (code 0), `"crashed"` for non-zero/signal. */
  reason: 'exited' | 'crashed';
}

/** Information about a running managed process. */
export interface ProcessInfo {
  /** Unique handle identifier. */
  handleId: string;
  /** The MOD command reference that was started. */
  command: string;
  /** Arguments passed to the process. */
  args: string[];
  /** OS process ID. */
  pid: number;
  /** ISO 8601 timestamp of when the process was started. */
  startedAt: string;
}

/**
 * Handle to a running managed process.
 *
 * Implements `AsyncDisposable` for automatic cleanup via `await using`.
 * Disposing stops the process (if still running) and disconnects the exit listener.
 *
 * @example
 * ```typescript
 * {
 *   await using proc = await processes.start({ command: "my-mod:script" });
 *   proc.onExit((info) => console.log("Crashed!", info));
 *   // proc is automatically stopped and cleaned up at end of block
 * }
 * ```
 */
export interface ProcessHandle extends AsyncDisposable {
  /** Unique handle identifier for this process. */
  readonly handleId: string;

  /**
   * Register a callback for unexpected process termination.
   *
   * Multiple callbacks can be registered (addEventListener-style).
   * Callbacks are NOT invoked when the process is stopped via {@link stop}
   * or `[Symbol.asyncDispose]`.
   *
   * @param callback - Function called with exit information
   *
   * @example
   * ```typescript
   * proc.onExit((info) => {
   *   if (info.reason === "crashed") {
   *     console.error(`Crash! Exit code: ${info.exitCode}`);
   *   }
   * });
   * ```
   */
  onExit(callback: (info: ProcessExitInfo) => void): void;

  /**
   * Stop the process and clean up the exit listener.
   *
   * Alias for `[Symbol.asyncDispose]()`. Idempotent — safe to call multiple times
   * or on an already-exited process.
   *
   * If the process is still running, this sends a stop request to the engine
   * and awaits shutdown. If the process has already exited, this only
   * disconnects the exit listener. `onExit` callbacks are NOT invoked.
   *
   * @example
   * ```typescript
   * const proc = await processes.start({ command: "my-mod:script" });
   * // ...later
   * await proc.stop();
   * ```
   */
  stop(): Promise<void>;
}

/** Signal payload shape for the process:exited channel. */
interface ExitSignalPayload {
  handleId: string;
  command: string;
  exitCode: number | null;
  signal: string | null;
  reason: 'exited' | 'crashed';
}

function createProcessHandle(handleId: string): ProcessHandle {
  const exitCallbacks: Array<(info: ProcessExitInfo) => void> = [];
  let disposed = false;
  let exited = false;

  // Subscribe to the shared signals WebSocket (single connection, multiplexed
  // by channel). Filter incoming events by handleId client-side.
  const subscription = signals.stream<ExitSignalPayload>('process:exited', (payload) => {
    if (payload.handleId !== handleId) return;
    if (disposed) return;

    exited = true;
    const info: ProcessExitInfo = {
      exitCode: payload.exitCode,
      signal: payload.signal,
      reason: payload.reason,
    };
    for (const cb of exitCallbacks) {
      try {
        cb(info);
      } catch (e) {
        console.error('Error in onExit callback:', e);
      }
    }
    subscription.close();
  });

  async function stop(): Promise<void> {
    if (disposed) return;
    disposed = true;
    subscription.close();

    if (!exited) {
      try {
        await host.deleteMethod(host.createUrl(`processes/${handleId}`));
      } catch {
        // 404 = already exited — idempotent success
      }
    }
  }

  return {
    handleId,

    onExit(callback: (info: ProcessExitInfo) => void): void {
      exitCallbacks.push(callback);
    },

    stop,

    [Symbol.asyncDispose]: stop,
  };
}

export namespace processes {
  /**
   * Start a long-running MOD command process.
   *
   * Returns a {@link ProcessHandle} for lifecycle management.
   * The handle is `AsyncDisposable` — use `await using` for automatic cleanup.
   *
   * @param params - Process start parameters
   * @returns A handle to the running process
   *
   * @example
   * ```typescript
   * const proc = await processes.start({
   *   command: "@hmcs/persona:default-behavior",
   *   args: ["persona-1"],
   * });
   * await proc.stop();
   * ```
   */
  export async function start(params: {
    /** Full MOD command reference (`mod-name:bin-name`). */
    command: string;
    /** Arguments forwarded to the process. */
    args?: string[];
  }): Promise<ProcessHandle> {
    const response = await host.post(host.createUrl('processes/start'), {
      command: params.command,
      args: params.args ?? [],
    });
    const { handleId } = (await response.json()) as { handleId: string };
    return createProcessHandle(handleId);
  }

  /**
   * List all running managed processes.
   *
   * @returns Array of process information objects
   *
   * @example
   * ```typescript
   * const running = await processes.list();
   * for (const p of running) {
   *   console.log(`${p.command} (pid ${p.pid}) since ${p.startedAt}`);
   * }
   * ```
   */
  export async function list(): Promise<ProcessInfo[]> {
    const response = await host.get(host.createUrl('processes'));
    return (await response.json()) as ProcessInfo[];
  }
}
