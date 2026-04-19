/**
 * MCP (Model Context Protocol) extension API for Desktop Homunculus mods.
 *
 * Mods use this module to extend the Homunculus MCP server with Tools, Prompts,
 * and Resources via the official `@modelcontextprotocol/sdk`. Uses the
 * session-per-transport-and-server pattern: a fresh `McpServer` instance is
 * constructed for each incoming MCP session (public SDK v1.x recommended pattern).
 *
 * Required environment variables:
 * - `HMCS_MOD_NAME` — name of the mod (e.g. `@hmcs/voicevox`)
 * - `HMCS_PORT` — engine HTTP API port (default: `3100`)
 *
 * @example
 * ```typescript
 * import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
 * import { mcp } from "@hmcs/sdk/mcp";
 *
 * const service = await mcp.serve({
 *   createServer: () => {
 *     const server = new McpServer({ name: "voicevox", version: "1.0.0" });
 *     server.tool("speak", { text: z.string() }, async ({ text }) => ({
 *       content: [{ type: "text", text: `Speaking: ${text}` }],
 *     }));
 *     return server;
 *   },
 * });
 * console.log(`MCP server listening on port ${service.port}`);
 * ```
 *
 * @packageDocumentation
 */

import { randomUUID } from 'node:crypto';
import http from 'node:http';
import type { AddressInfo } from 'node:net';
import type { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StreamableHTTPServerTransport } from '@modelcontextprotocol/sdk/server/streamableHttp.js';
import { isInitializeRequest } from '@modelcontextprotocol/sdk/types.js';

import { readEnginePort, readModName } from './internal/env';
import { readJsonBody } from './internal/http';
import { deregisterFromEngine, registerWithEngineWithRetry } from './internal/mcp-register';

/**
 * Options for {@link mcp.serve}.
 */
export interface McpServeOptions {
  /**
   * Factory called once per incoming MCP session to construct a fresh `McpServer`.
   * Register Tools, Prompts, and Resources inside this function. A new instance is
   * created for each session, matching the MCP SDK v1.x session-per-server pattern.
   *
   * @example
   * ```typescript
   * createServer: () => {
   *   const server = new McpServer({ name: "my-mod", version: "1.0.0" });
   *   server.tool("ping", {}, async () => ({
   *     content: [{ type: "text", text: "pong" }],
   *   }));
   *   return server;
   * }
   * ```
   */
  createServer: () => McpServer | Promise<McpServer>;

  /**
   * Namespace slug used for resource URI scheme validation and engine registration.
   * Must match `/^[a-z][a-z0-9_]*$/`.
   *
   * Defaults to the last `/` segment of the package name with `-` replaced by `_`.
   * Example: `@hmcs/voicevox` → `voicevox`.
   */
  slug?: string;

  /**
   * Optional fixed port for the local MCP HTTP server.
   * Defaults to an ephemeral port assigned by the OS.
   */
  port?: number;
}

/**
 * A running MCP service returned by {@link mcp.serve}.
 */
export interface McpService {
  /** The port the local MCP HTTP server is listening on. */
  readonly port: number;
  /** Deregister from the engine, close all sessions, and stop the server. */
  close(): Promise<void>;
}

/**
 * Derive a slug from a package name by taking the last `/` segment and
 * replacing `-` with `_`.
 *
 * @example
 * ```typescript
 * deriveSlugFromModName("@hmcs/voicevox"); // "voicevox"
 * deriveSlugFromModName("my-mod");          // "my_mod"
 * ```
 */
export function deriveSlugFromModName(modName: string): string {
  const last = modName.split('/').pop() ?? modName;
  return last.replace(/-/g, '_').toLowerCase();
}

/**
 * Validate that a slug matches `/^[a-z][a-z0-9_]*$/`. Throws on failure.
 */
export function validateSlug(slug: string): void {
  if (!/^[a-z][a-z0-9_]*$/.test(slug)) {
    throw new Error(`Invalid mod slug '${slug}': must match /^[a-z][a-z0-9_]*$/`);
  }
}

/**
 * Warn if any registered Resource URI uses a scheme other than the mod's slug.
 * Relies on `@modelcontextprotocol/sdk` internals — may be brittle across SDK versions.
 */
export function validateResourceSchemes(server: McpServer, slug: string): void {
  const anyServer = server as unknown as {
    _registeredResources?: Record<string, unknown>;
    _registeredResourceTemplates?: Record<string, unknown>;
  };
  const uris = [
    ...Object.keys(anyServer._registeredResources ?? {}),
    ...Object.keys(anyServer._registeredResourceTemplates ?? {}),
  ];
  for (const uriOrPattern of uris) {
    const scheme = uriOrPattern.split('://')[0];
    if (scheme && scheme !== slug) {
      console.warn(
        `[hmcs/sdk/mcp] Resource scheme '${scheme}' does not match mod slug '${slug}'. ` +
          `Recommended: use '${slug}://' for all resources.`,
      );
    }
  }
}

interface Session {
  transport: StreamableHTTPServerTransport;
  server: McpServer;
}

/**
 * MCP extension namespace.
 *
 * @example
 * ```typescript
 * import { mcp } from "@hmcs/sdk/mcp";
 * ```
 */
export namespace mcp {
  /**
   * Start a local MCP HTTP server and register it with the Homunculus engine.
   *
   * Reads `HMCS_MOD_NAME` and `HMCS_PORT` from the environment. The server
   * listens on `127.0.0.1` and registers via `POST /mcp/register` with
   * exponential backoff (100ms → 5s, max 10 attempts).
   *
   * A fresh `McpServer` is constructed per session by calling `createServer()`
   * on each `InitializeRequest`. Sessions are cleaned up when the transport
   * closes. `SIGTERM` and `SIGINT` handlers automatically call `close()`.
   *
   * @param options - {@link McpServeOptions} including the server factory
   * @returns A promise resolving to an {@link McpService} once listening and registered
   * @throws If `HMCS_MOD_NAME` is missing
   * @throws If engine registration fails after all retry attempts
   * @throws If the slug is invalid
   *
   * @example
   * ```typescript
   * import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
   * import { mcp } from "@hmcs/sdk/mcp";
   * import { z } from "zod";
   *
   * const service = await mcp.serve({
   *   createServer: () => {
   *     const server = new McpServer({ name: "my-mod", version: "1.0.0" });
   *     server.tool("greet", { name: z.string() }, async ({ name }) => ({
   *       content: [{ type: "text", text: `Hello, ${name}!` }],
   *     }));
   *     return server;
   *   },
   * });
   * console.log(`Listening on port ${service.port}`);
   * ```
   */
  export async function serve(options: McpServeOptions): Promise<McpService> {
    const modName = readModName();
    const enginePort = readEnginePort();
    const slug = options.slug ?? deriveSlugFromModName(modName);
    validateSlug(slug);

    await probeResourceSchemes(options.createServer, slug);

    const sessions = new Map<string, Session>();
    const httpServer = http.createServer((req, res) => {
      void handleRequest(req, res, sessions, options.createServer);
    });

    await new Promise<void>((resolve) =>
      httpServer.listen(options.port ?? 0, '127.0.0.1', resolve),
    );
    const port = (httpServer.address() as AddressInfo).port;

    const mcpUrl = `http://127.0.0.1:${port}/mcp`;
    await registerWithEngineWithRetry(enginePort, { modName, modSlug: slug, mcpUrl });

    const close = buildCloseHandler(sessions, httpServer, enginePort, slug);
    process.once('SIGTERM', () => {
      void close();
    });
    process.once('SIGINT', () => {
      void close();
    });

    return { port, close };
  }
}

async function probeResourceSchemes(
  createServer: () => McpServer | Promise<McpServer>,
  slug: string,
): Promise<void> {
  try {
    const probe = await createServer();
    validateResourceSchemes(probe, slug);
    const anyProbe = probe as unknown as { close?: () => Promise<void> };
    if (typeof anyProbe.close === 'function') {
      await anyProbe.close().catch(() => undefined);
    }
  } catch (err) {
    console.warn('[hmcs/sdk/mcp] resource scheme probe failed:', err);
  }
}

function buildCloseHandler(
  sessions: Map<string, Session>,
  httpServer: http.Server,
  enginePort: number,
  slug: string,
): () => Promise<void> {
  return async () => {
    await deregisterFromEngine(enginePort, slug).catch(() => undefined);
    // Snapshot before iterating — transport.close() fires onclose which mutates the map
    const snapshot = [...sessions.values()];
    sessions.clear();
    await Promise.all(snapshot.map((s) => s.transport.close().catch(() => undefined)));
    await new Promise<void>((resolve) => httpServer.close(() => resolve()));
  };
}

async function handleRequest(
  req: http.IncomingMessage,
  res: http.ServerResponse,
  sessions: Map<string, Session>,
  createServer: () => McpServer | Promise<McpServer>,
): Promise<void> {
  if (!req.url?.startsWith('/mcp')) {
    res.writeHead(404).end();
    return;
  }
  const sessionId = req.headers['mcp-session-id'] as string | undefined;
  try {
    if (req.method === 'POST') {
      await handlePost(req, res, sessions, createServer, sessionId);
    } else if (req.method === 'GET' || req.method === 'DELETE') {
      await handleGetOrDelete(req, res, sessions, sessionId);
    } else {
      res.writeHead(405).end();
    }
  } catch (err) {
    sendErrorResponse(res, (err as Error).message);
  }
}

function sendErrorResponse(res: http.ServerResponse, message: string): void {
  if (!res.headersSent) {
    res.writeHead(500, { 'Content-Type': 'application/json' });
  }
  res.end(JSON.stringify({ error: message }));
}

async function handlePost(
  req: http.IncomingMessage,
  res: http.ServerResponse,
  sessions: Map<string, Session>,
  createServer: () => McpServer | Promise<McpServer>,
  sessionId: string | undefined,
): Promise<void> {
  const body = await readJsonBody(req);

  if (sessionId && sessions.has(sessionId)) {
    await sessions.get(sessionId)?.transport.handleRequest(req, res, body);
    return;
  }

  if (!sessionId && isInitializeRequest(body)) {
    await startNewSession(req, res, sessions, createServer, body);
    return;
  }

  res.writeHead(400, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify({ error: 'invalid session' }));
}

async function startNewSession(
  req: http.IncomingMessage,
  res: http.ServerResponse,
  sessions: Map<string, Session>,
  createServer: () => McpServer | Promise<McpServer>,
  body: unknown,
): Promise<void> {
  const server = await createServer();
  const transport: StreamableHTTPServerTransport = new StreamableHTTPServerTransport({
    sessionIdGenerator: () => randomUUID(),
    onsessioninitialized: (id) => void sessions.set(id, { transport, server }),
  });
  transport.onclose = () => cleanupSession(transport, server, sessions);
  await server.connect(transport);
  await transport.handleRequest(req, res, body);
}

function cleanupSession(
  transport: StreamableHTTPServerTransport,
  server: McpServer,
  sessions: Map<string, Session>,
): void {
  const id = transport.sessionId;
  if (id) sessions.delete(id);
  const anyServer = server as unknown as { close?: () => Promise<void> };
  if (typeof anyServer.close === 'function') {
    void anyServer.close().catch(() => undefined);
  }
}

async function handleGetOrDelete(
  req: http.IncomingMessage,
  res: http.ServerResponse,
  sessions: Map<string, Session>,
  sessionId: string | undefined,
): Promise<void> {
  if (!sessionId || !sessions.has(sessionId)) {
    res.writeHead(400, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ error: 'invalid session' }));
    return;
  }
  await sessions.get(sessionId)?.transport.handleRequest(req, res);
}
