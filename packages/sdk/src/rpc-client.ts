/**
 * Browser-safe RPC client for Desktop Homunculus mods.
 *
 * This module provides the {@link rpc.call} function for invoking RPC methods
 * on MOD services via the engine's HTTP API. It uses {@link host.post} internally
 * and works in both browser (WebView) and Node.js environments.
 *
 * Import from `@hmcs/sdk/rpc` — the bundler or Node.js runtime will resolve
 * the appropriate module via conditional exports.
 *
 * @example
 * ```typescript
 * import { rpc } from "@hmcs/sdk/rpc";
 *
 * const result = await rpc.call<{ greeting: string }>({
 *   modName: "voicevox",
 *   method: "speak",
 *   body: { text: "Hello!" },
 * });
 * console.log(result.greeting);
 * ```
 *
 * @packageDocumentation
 */

import { host } from './host';

/**
 * Options for {@link rpc.call}.
 */
export interface RpcCallOptions {
  /** Name of the target MOD. */
  modName: string;
  /** RPC method name to invoke on the MOD service. */
  method: string;
  /** Optional request body passed to the method handler. */
  body?: unknown;
}

/**
 * Browser-safe RPC client namespace.
 *
 * @example
 * ```typescript
 * import { rpc } from "@hmcs/sdk/rpc";
 *
 * const result = await rpc.call<{ sum: number }>({
 *   modName: "calculator",
 *   method: "add",
 *   body: { a: 1, b: 2 },
 * });
 * ```
 */
export namespace rpc {
  /**
   * Call an RPC method on a MOD service.
   *
   * Sends a `POST /rpc/call` request to the engine, which proxies the call
   * to the target MOD's service process.
   *
   * @typeParam T - Expected return type of the RPC method
   * @param options - {@link RpcCallOptions} specifying the target mod, method, and body
   * @returns The parsed JSON response from the MOD method handler
   * @throws {HomunculusApiError} status 503 — MOD not registered
   * @throws {HomunculusApiError} status 404 — method not found
   * @throws {HomunculusApiError} status 504 — timeout exceeded
   * @throws {HomunculusApiError} status 502 — MOD service unreachable
   *
   * @example
   * ```typescript
   * import { rpc } from "@hmcs/sdk/rpc";
   *
   * // Call with a request body
   * const result = await rpc.call<{ greeting: string }>({
   *   modName: "voicevox",
   *   method: "speak",
   *   body: { text: "Hello!" },
   * });
   *
   * // Call without a body
   * const status = await rpc.call<{ running: boolean }>({
   *   modName: "voicevox",
   *   method: "status",
   * });
   * ```
   */
  export async function call<T = unknown>(options: RpcCallOptions): Promise<T> {
    const url = host.createUrl('rpc/call');
    const response = await host.post(url, {
      modName: options.modName,
      method: options.method,
      ...(options.body !== undefined ? { body: options.body } : {}),
    });
    return response.json() as Promise<T>;
  }
}
