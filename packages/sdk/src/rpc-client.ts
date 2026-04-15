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
 * A single RPC method entry from the engine's registration list.
 */
export interface RpcRegistrationEntry {
  /** Name of the MOD that registered this method. */
  modName: string;
  /** RPC method name. */
  method: string;
  /** Human-readable description of the method. */
  description?: string;
  /** Metadata attached to the method (e.g., `{ category: "tts" }`). */
  meta?: Record<string, unknown>;
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

  /**
   * List registered RPC methods across all MOD services.
   *
   * Fetches the full RPC registry from the engine and flattens it into
   * per-method entries. Optionally filters by `_meta.category`.
   *
   * @param filter - Optional filter. When `category` is set, only methods
   *   whose `_meta.category` matches are returned.
   * @returns Array of registration entries
   *
   * @example
   * ```typescript
   * import { rpc } from "@hmcs/sdk/rpc";
   *
   * // List all TTS engines
   * const ttsEngines = await rpc.registrations({ category: "tts" });
   * // [{ modName: "@hmcs/voicevox", method: "speak", description: "...", meta: { category: "tts" } }]
   * ```
   */
  export async function registrations(
    filter?: { category?: string },
  ): Promise<RpcRegistrationEntry[]> {
    const url = host.createUrl('rpc/registrations');
    const response = await host.get(url);
    const data = (await response.json()) as Record<
      string,
      { port: number; methods: Record<string, { description?: string; _meta?: Record<string, unknown> }> }
    >;

    const entries: RpcRegistrationEntry[] = [];
    for (const [modName, registration] of Object.entries(data)) {
      for (const [method, methodMeta] of Object.entries(registration.methods)) {
        const meta = methodMeta._meta;
        if (filter?.category !== undefined) {
          if (meta?.category !== filter.category) continue;
        }
        entries.push({
          modName,
          method,
          description: methodMeta.description,
          meta,
        });
      }
    }
    return entries;
  }
}
