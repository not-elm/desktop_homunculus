/**
 * RPC utilities for Desktop Homunculus mods (Node.js entry point).
 *
 * This module provides:
 * - **Client:** {@link rpc.call} — invoke an RPC method on another MOD
 *   (browser-safe, also available via the `default` conditional export)
 * - **Server:** {@link rpc.serve} — start a local RPC HTTP server (Node.js only)
 * - **Method definition:** {@link rpc.method} — create typed RPC method
 *   definitions with Zod validation (Node.js only)
 *
 * Import from `@hmcs/sdk/rpc`. In Node.js, all three APIs are available.
 * In browser/bundler environments, only {@link rpc.call} is available
 * (resolved via conditional exports to `rpc-client`).
 *
 * @remarks
 * The server APIs (`serve`, `method`) use Node.js APIs (`node:http`, `process`)
 * and are not browser-compatible.
 *
 * Required environment variables (for `serve` only):
 * - `HMCS_RPC_PORT` — port the local RPC HTTP server will listen on
 * - `HMCS_MOD_NAME` — name of the mod (used for registration with the engine)
 * - `HMCS_PORT` — engine HTTP API port (default: `3100`)
 *
 * @example
 * ```typescript
 * import { z } from "zod";
 * import { rpc } from "@hmcs/sdk/rpc";
 *
 * // Client: call an RPC method on another MOD (works in browser + Node.js)
 * const result = await rpc.call<{ greeting: string }>({
 *   modName: "voicevox",
 *   method: "speak",
 *   body: { text: "Hello!" },
 * });
 *
 * // Server: expose RPC methods (Node.js only)
 * const greet = rpc.method({
 *   description: "Greet a user by name",
 *   input: z.object({ name: z.string() }),
 *   handler: async ({ name }) => ({ greeting: `Hello, ${name}!` }),
 * });
 *
 * const server = await rpc.serve({ methods: { greet } });
 * console.log(`RPC server running on port ${server.port}`);
 * ```
 *
 * @packageDocumentation
 */

import * as http from 'node:http';
import type { ZodType } from 'zod';
import { readEnginePort, readModName, readRpcPort } from './internal/env';
import { readRawBody } from './internal/http';
import { rpc as rpcClient } from './rpc-client';

export type { RpcCallOptions, RpcRegistrationEntry } from './rpc-client';

/**
 * A single RPC method definition created by {@link rpc.method}.
 *
 * @typeParam I - The validated input type (inferred from the Zod schema)
 * @typeParam O - The output type returned by the handler
 */
export interface RpcMethodDef<I = unknown, O = unknown> {
  /** Optional human-readable description of the method. */
  description?: string;
  /** Optional handler timeout in milliseconds. */
  timeout?: number;
  /** Zod schema used to validate incoming request bodies. */
  input?: ZodType<I>;
  /** Async function called with the validated input. */
  handler: (params: I) => Promise<O>;
}

/**
 * A plain async function that can be used directly as a method handler without
 * any input validation.
 */
export type RpcHandlerFn<O = unknown> = (params: unknown) => Promise<O>;

/**
 * A value accepted as an RPC method — either a full {@link RpcMethodDef} or a
 * plain async function.
 */
export type RpcMethodEntry = RpcMethodDef | RpcHandlerFn;

/**
 * Options for {@link rpc.serve}.
 */
export interface RpcServeOptions {
  /**
   * Map of method names to {@link RpcMethodDef} objects or plain async
   * functions.
   */
  methods: Record<string, RpcMethodEntry>;
}

/**
 * A running RPC server returned by {@link rpc.serve}.
 */
export interface RpcServer {
  /** The port the HTTP server is listening on. */
  port: number;
  /** Gracefully shuts down the server and resolves when closed. */
  close: () => Promise<void>;
}

function isRpcMethodDef(entry: RpcMethodEntry): entry is RpcMethodDef {
  return typeof entry === 'object' && entry !== null && 'handler' in entry;
}

function jsonResponse(res: http.ServerResponse, status: number, body: unknown): void {
  const payload = JSON.stringify(body);
  res.writeHead(status, {
    'Content-Type': 'application/json',
    'Content-Length': Buffer.byteLength(payload),
  });
  res.end(payload);
}

function validateMethodName(name: string): void {
  if (!name || name.includes('/') || name.includes('\\')) {
    throw new Error(
      `Invalid RPC method name: "${name}". Must be non-empty and cannot contain slashes.`,
    );
  }
}

function listenOnPort(server: http.Server, port: number): Promise<void> {
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(port, '127.0.0.1', () => {
      server.off('error', reject);
      resolve();
    });
  });
}

function buildRpcServer(server: http.Server, port: number): RpcServer {
  return {
    port,
    close: () =>
      new Promise((resolve, reject) => {
        server.close((err) => {
          if (err) reject(err);
          else resolve();
        });
      }),
  };
}

async function handleRequest(
  methods: Record<string, RpcMethodEntry>,
  req: http.IncomingMessage,
  res: http.ServerResponse,
): Promise<void> {
  if (req.method !== 'POST') {
    jsonResponse(res, 405, {
      error: 'METHOD_NOT_ALLOWED',
      message: 'Only POST is supported',
    });
    return;
  }
  const methodName = (req.url ?? '/').replace(/^\//, '');
  if (!methodName || !(methodName in methods)) {
    jsonResponse(res, 404, {
      error: 'METHOD_NOT_FOUND',
      message: `Unknown method: ${methodName}`,
    });
    return;
  }

  let rawBody: string;
  try {
    rawBody = await readRawBody(req);
  } catch (err) {
    jsonResponse(res, 400, {
      error: 'READ_ERROR',
      message: (err as Error).message,
    });
    return;
  }

  let parsedBody: unknown;
  try {
    parsedBody = rawBody.trim().length > 0 ? JSON.parse(rawBody) : {};
  } catch {
    jsonResponse(res, 400, {
      error: 'VALIDATION_ERROR',
      message: 'Invalid input',
      details: [{ message: 'Request body is not valid JSON' }],
    });
    return;
  }

  await dispatchMethod(methods[methodName], parsedBody, res);
}

async function dispatchMethod(
  entry: RpcMethodEntry,
  body: unknown,
  res: http.ServerResponse,
): Promise<void> {
  if (isRpcMethodDef(entry)) {
    if (entry.input) {
      const result = entry.input.safeParse(body);
      if (!result.success) {
        jsonResponse(res, 400, {
          error: 'VALIDATION_ERROR',
          message: 'Invalid input',
          details: result.error.errors,
        });
        return;
      }
      try {
        jsonResponse(res, 200, await entry.handler(result.data));
      } catch (err) {
        jsonResponse(res, 500, {
          error: 'HANDLER_ERROR',
          message: (err as Error).message ?? 'Unknown error',
        });
      }
    } else {
      try {
        jsonResponse(res, 200, await entry.handler(body));
      } catch (err) {
        jsonResponse(res, 500, {
          error: 'HANDLER_ERROR',
          message: (err as Error).message ?? 'Unknown error',
        });
      }
    }
  } else {
    try {
      jsonResponse(res, 200, await entry(body));
    } catch (err) {
      jsonResponse(res, 500, {
        error: 'HANDLER_ERROR',
        message: (err as Error).message ?? 'Unknown error',
      });
    }
  }
}

function buildMethodsMeta(
  methods: Record<string, RpcMethodEntry>,
): Record<string, Record<string, unknown>> {
  const meta: Record<string, Record<string, unknown>> = {};
  for (const [name, entry] of Object.entries(methods)) {
    if (isRpcMethodDef(entry)) {
      meta[name] = {
        ...(entry.description !== undefined ? { description: entry.description } : {}),
        ...(entry.timeout !== undefined ? { timeout: entry.timeout } : {}),
      };
    } else {
      meta[name] = {};
    }
  }
  return meta;
}

async function registerWithRetry(
  enginePort: number,
  modName: string,
  methods: Record<string, RpcMethodEntry>,
): Promise<void> {
  const url = `http://127.0.0.1:${enginePort}/rpc/register`;
  const body = JSON.stringify({ modName, methods: buildMethodsMeta(methods) });

  const maxAttempts = 10;
  let delay = 100;
  const maxDelay = 5000;

  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      const res = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body,
      });
      if (res.ok) {
        return;
      }
      const text = await res.text().catch(() => '(unreadable)');
      throw new Error(`Engine returned ${res.status}: ${text}`);
    } catch (err) {
      if (attempt === maxAttempts) {
        throw new Error(
          `Failed to register RPC methods after ${maxAttempts} attempts: ${(err as Error).message}`,
        );
      }
      await new Promise((resolve) => setTimeout(resolve, delay));
      delay = Math.min(delay * 2, maxDelay);
    }
  }
}

/**
 * RPC utilities for Desktop Homunculus mod service processes.
 *
 * In Node.js: `call`, `method`, and `serve` are all available.
 * In browser/bundler: only `call` is available (via conditional exports).
 *
 * @example
 * ```typescript
 * import { z } from "zod";
 * import { rpc } from "@hmcs/sdk/rpc";
 *
 * // Client — call another MOD's RPC method
 * const result = await rpc.call<{ sum: number }>({
 *   modName: "calculator",
 *   method: "add",
 *   body: { a: 1, b: 2 },
 * });
 *
 * // Server — expose RPC methods (Node.js only)
 * const add = rpc.method({
 *   description: "Add two numbers",
 *   timeout: 5000,
 *   input: z.object({ a: z.number(), b: z.number() }),
 *   handler: async ({ a, b }) => ({ result: a + b }),
 * });
 *
 * const server = await rpc.serve({ methods: { add } });
 * // server.port — the port the server is listening on
 * // server.close() — shuts down the server
 * ```
 */
export namespace rpc {
  /**
   * Call an RPC method on a MOD service.
   *
   * Delegates to the browser-safe `rpc-client` implementation. See
   * {@link rpcClient.call} for full documentation.
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
   * ```
   */
  export const call = rpcClient.call;
  export const registrations = rpcClient.registrations;

  /**
   * Create a typed RPC method definition with Zod input validation.
   *
   * Both `rpc.method()` definitions AND plain async functions are accepted by
   * {@link rpc.serve}. Use `rpc.method()` when you want input validation,
   * descriptions, or timeout metadata.
   *
   * @typeParam I - The input type inferred from the Zod schema
   * @typeParam O - The return type of the handler
   * @param def - Method definition including input schema, handler, and optional metadata
   * @returns An {@link RpcMethodDef} object suitable for passing to {@link rpc.serve}
   *
   * @example
   * ```typescript
   * import { z } from "zod";
   * import { rpc } from "@hmcs/sdk/rpc";
   *
   * const echo = rpc.method({
   *   description: "Echo the input back",
   *   timeout: 3000,
   *   input: z.object({ text: z.string() }),
   *   handler: async ({ text }) => ({ text }),
   * });
   * ```
   */
  export function method<I, O>(def: {
    description?: string;
    timeout?: number;
    input: ZodType<I>;
    handler: (params: I) => Promise<O>;
  }): RpcMethodDef<I, O>;
  export function method<O>(def: {
    description?: string;
    timeout?: number;
    handler: () => Promise<O>;
  }): RpcMethodDef<unknown, O>;
  export function method(def: {
    description?: string;
    timeout?: number;
    input?: ZodType<unknown>;
    handler: (params?: unknown) => Promise<unknown>;
  }): RpcMethodDef {
    return {
      description: def.description,
      timeout: def.timeout,
      input: def.input,
      handler: def.handler,
    };
  }

  /**
   * Start an RPC HTTP server and register with the engine.
   *
   * Reads `HMCS_RPC_PORT` and `HMCS_MOD_NAME` from the environment (throws if
   * either is missing). The engine port defaults to `3100` but can be overridden
   * via `HMCS_PORT`.
   *
   * The server listens on `127.0.0.1:{HMCS_RPC_PORT}` and routes
   * `POST /{methodName}` to the matching handler. After the server is listening,
   * it registers all methods with the engine via
   * `POST /rpc/register` with exponential backoff (100ms → 5s, max 10 attempts).
   *
   * A `SIGTERM` handler is installed to call {@link RpcServer.close} automatically.
   *
   * @param options - {@link RpcServeOptions} containing the method map
   * @returns A promise that resolves to an {@link RpcServer} once listening and registered
   * @throws If `HMCS_RPC_PORT` or `HMCS_MOD_NAME` env vars are missing
   * @throws If registration with the engine fails after all retry attempts
   *
   * @example
   * ```typescript
   * import { z } from "zod";
   * import { rpc } from "@hmcs/sdk/rpc";
   *
   * const greet = rpc.method({
   *   description: "Greet a user",
   *   input: z.object({ name: z.string() }),
   *   handler: async ({ name }) => ({ message: `Hello, ${name}!` }),
   * });
   *
   * const server = await rpc.serve({ methods: { greet } });
   * console.log(`Listening on port ${server.port}`);
   * ```
   */
  export async function serve(options: RpcServeOptions): Promise<RpcServer> {
    const rpcPort = readRpcPort();
    const modName = readModName();
    const enginePort = readEnginePort();
    const { methods } = options;

    for (const name of Object.keys(methods)) {
      validateMethodName(name);
    }

    const server = http.createServer((req, res) => {
      handleRequest(methods, req, res);
    });

    await listenOnPort(server, rpcPort);
    await registerWithRetry(enginePort, modName, methods);

    const rpcServer = buildRpcServer(server, rpcPort);
    process.once('SIGTERM', () => {
      rpcServer.close().catch(() => {
        // ignore close errors during shutdown
      });
    });
    return rpcServer;
  }
}
