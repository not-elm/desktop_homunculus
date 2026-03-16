/**
 * RPC server utilities for Desktop Homunculus mods.
 *
 * This module provides helpers for exposing typed RPC methods from a mod's
 * service process. The engine proxies incoming calls to `POST /{method}` on
 * the local HTTP server started by {@link rpc.serve}.
 *
 * **Server:** {@link rpc.serve}
 * **Method definition:** {@link rpc.method}
 *
 * @remarks
 * This module uses Node.js APIs (`node:http`, `process`) and is not
 * browser-compatible. Import from `@hmcs/sdk/rpc` — it is intentionally
 * not re-exported from the main `@hmcs/sdk` entry point.
 *
 * Required environment variables:
 * - `HMCS_RPC_PORT` — port the local RPC HTTP server will listen on
 * - `HMCS_MOD_NAME` — name of the mod (used for registration with the engine)
 * - `HMCS_PORT` — engine HTTP API port (default: `3100`)
 *
 * @example
 * ```typescript
 * import { z } from "zod";
 * import { rpc } from "@hmcs/sdk/rpc";
 *
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

import * as http from "node:http";
import { type ZodType } from "zod";

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
  input: ZodType<I>;
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
  return typeof entry === "object" && entry !== null && "handler" in entry;
}

function jsonResponse(
  res: http.ServerResponse,
  status: number,
  body: unknown,
): void {
  const payload = JSON.stringify(body);
  res.writeHead(status, {
    "Content-Type": "application/json",
    "Content-Length": Buffer.byteLength(payload),
  });
  res.end(payload);
}

async function readBody(req: http.IncomingMessage): Promise<string> {
  const chunks: Buffer[] = [];
  for await (const chunk of req) {
    chunks.push(chunk as Buffer);
  }
  return Buffer.concat(chunks).toString("utf-8");
}

async function registerWithRetry(
  enginePort: number,
  modName: string,
  methods: Record<string, RpcMethodEntry>,
): Promise<void> {
  const url = `http://127.0.0.1:${enginePort}/rpc/register`;
  const methodsMeta: Record<string, { description?: string; timeout?: number }> = {};
  for (const [name, entry] of Object.entries(methods)) {
    if (isRpcMethodDef(entry)) {
      methodsMeta[name] = {
        ...(entry.description !== undefined ? { description: entry.description } : {}),
        ...(entry.timeout !== undefined ? { timeout: entry.timeout } : {}),
      };
    } else {
      methodsMeta[name] = {};
    }
  }
  const body = JSON.stringify({ modName, methods: methodsMeta });

  const maxAttempts = 10;
  let delay = 100;
  const maxDelay = 5000;

  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      const res = await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body,
      });
      if (res.ok) {
        return;
      }
      const text = await res.text().catch(() => "(unreadable)");
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
 * RPC server utilities for Desktop Homunculus mod service processes.
 *
 * @example
 * ```typescript
 * import { z } from "zod";
 * import { rpc } from "@hmcs/sdk/rpc";
 *
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
  }): RpcMethodDef<I, O> {
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
    const rpcPortStr = process.env["HMCS_RPC_PORT"];
    if (!rpcPortStr) {
      throw new Error("HMCS_RPC_PORT environment variable is required");
    }
    const rpcPort = parseInt(rpcPortStr, 10);
    if (isNaN(rpcPort)) {
      throw new Error(`HMCS_RPC_PORT is not a valid port number: ${rpcPortStr}`);
    }

    const modName = process.env["HMCS_MOD_NAME"];
    if (!modName) {
      throw new Error("HMCS_MOD_NAME environment variable is required");
    }

    const enginePortStr = process.env["HMCS_PORT"] ?? "3100";
    const enginePort = parseInt(enginePortStr, 10);

    const { methods } = options;

    const server = http.createServer(async (req, res) => {
      const url = req.url ?? "/";
      // Only handle POST /{methodName}
      if (req.method !== "POST") {
        jsonResponse(res, 405, { error: "METHOD_NOT_ALLOWED", message: "Only POST is supported" });
        return;
      }

      const methodName = url.replace(/^\//, "");
      if (!methodName || !(methodName in methods)) {
        jsonResponse(res, 404, {
          error: "METHOD_NOT_FOUND",
          message: `Unknown method: ${methodName}`,
        });
        return;
      }

      const entry = methods[methodName];

      let rawBody: string;
      try {
        rawBody = await readBody(req);
      } catch (err) {
        jsonResponse(res, 400, { error: "READ_ERROR", message: (err as Error).message });
        return;
      }

      let parsedBody: unknown;
      try {
        parsedBody = rawBody.trim().length > 0 ? JSON.parse(rawBody) : {};
      } catch {
        jsonResponse(res, 400, {
          error: "VALIDATION_ERROR",
          message: "Invalid input",
          details: [{ message: "Request body is not valid JSON" }],
        });
        return;
      }

      if (isRpcMethodDef(entry)) {
        const result = entry.input.safeParse(parsedBody);
        if (!result.success) {
          jsonResponse(res, 400, {
            error: "VALIDATION_ERROR",
            message: "Invalid input",
            details: result.error.errors,
          });
          return;
        }
        try {
          const output = await entry.handler(result.data);
          jsonResponse(res, 200, output);
        } catch (err) {
          jsonResponse(res, 500, {
            error: "HANDLER_ERROR",
            message: (err as Error).message ?? "Unknown error",
          });
        }
      } else {
        // Plain async function
        try {
          const output = await entry(parsedBody);
          jsonResponse(res, 200, output);
        } catch (err) {
          jsonResponse(res, 500, {
            error: "HANDLER_ERROR",
            message: (err as Error).message ?? "Unknown error",
          });
        }
      }
    });

    // Start listening
    await new Promise<void>((resolve, reject) => {
      server.once("error", reject);
      server.listen(rpcPort, "127.0.0.1", () => {
        server.off("error", reject);
        resolve();
      });
    });

    // Register with engine (with retry)
    await registerWithRetry(enginePort, modName, methods);

    const rpcServer: RpcServer = {
      port: rpcPort,
      close: () =>
        new Promise<void>((resolve, reject) => {
          server.close((err) => {
            if (err) reject(err);
            else resolve();
          });
        }),
    };

    // Graceful shutdown on SIGTERM
    process.once("SIGTERM", () => {
      rpcServer.close().catch(() => {
        // ignore close errors during shutdown
      });
    });

    return rpcServer;
  }
}
