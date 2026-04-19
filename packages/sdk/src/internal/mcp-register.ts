/**
 * Registration helpers for the MCP extension API.
 * Internal module — not part of the public SDK surface.
 */

interface RegisterPayload {
  modName: string;
  modSlug: string;
  mcpUrl: string;
}

/**
 * Register this mod's MCP server with the engine, retrying with exponential
 * backoff (100ms → 5s, max 10 attempts).
 */
export async function registerWithEngineWithRetry(
  enginePort: number,
  payload: RegisterPayload,
): Promise<void> {
  const url = `http://127.0.0.1:${enginePort}/mcp/register`;
  const body = JSON.stringify(payload);
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
      if (res.ok) return;
      if (attempt === maxAttempts) {
        const text = await res.text().catch(() => '');
        throw new Error(`Failed to register MCP (HTTP ${res.status}): ${text}`);
      }
    } catch (err) {
      if (attempt === maxAttempts) {
        throw new Error(
          `Failed to register MCP after ${maxAttempts} attempts: ${(err as Error).message}`,
        );
      }
    }
    await new Promise((resolve) => setTimeout(resolve, delay));
    delay = Math.min(delay * 2, maxDelay);
  }
}

/**
 * Deregister this mod's MCP server from the engine. Best-effort — errors are
 * silently ignored by the caller during shutdown.
 */
export async function deregisterFromEngine(enginePort: number, modSlug: string): Promise<void> {
  const url = `http://127.0.0.1:${enginePort}/mcp/deregister`;
  await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ modSlug }),
  });
}
