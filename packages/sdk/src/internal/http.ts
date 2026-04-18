import type * as http from 'node:http';

/**
 * Reads the raw body string from an incoming HTTP request.
 * Internal helper shared across rpc.ts and mcp.ts — not part of the public API.
 */
export async function readRawBody(req: http.IncomingMessage): Promise<string> {
  const chunks: Buffer[] = [];
  for await (const chunk of req) {
    chunks.push(chunk as Buffer);
  }
  return Buffer.concat(chunks).toString('utf-8');
}

/**
 * Reads and JSON-parses the body from an incoming HTTP request.
 * Returns `undefined` if the body is empty. Throws if the body is non-JSON.
 */
export async function readJsonBody(req: http.IncomingMessage): Promise<unknown> {
  const raw = await readRawBody(req);
  if (!raw.trim()) return undefined;
  try {
    return JSON.parse(raw);
  } catch (e) {
    throw new Error(`Invalid JSON body: ${(e as Error).message}`);
  }
}
