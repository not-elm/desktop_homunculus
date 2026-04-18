/**
 * Reads environment variables provided by the Homunculus mod runtime.
 * Internal helpers shared across rpc.ts and mcp.ts — not part of the public API.
 */

export function readModName(): string {
  const name = process.env.HMCS_MOD_NAME;
  if (!name) throw new Error('HMCS_MOD_NAME environment variable is required');
  return name;
}

export function readEnginePort(): number {
  const s = process.env.HMCS_PORT ?? '3100';
  const port = parseInt(s, 10);
  if (Number.isNaN(port)) throw new Error(`HMCS_PORT is not a valid port number: ${s}`);
  return port;
}

export function readRpcPort(): number {
  const s = process.env.HMCS_RPC_PORT;
  if (!s) throw new Error('HMCS_RPC_PORT environment variable is required');
  const port = parseInt(s, 10);
  if (Number.isNaN(port)) throw new Error(`HMCS_RPC_PORT is not a valid port number: ${s}`);
  return port;
}
