import type { PluginDeps } from './deps.js';
import type { HmcsPersonaSnapshot } from './types.js';

interface HmcsFetchOptions {
  method: 'GET' | 'POST';
  body?: unknown;
}

/**
 * GET /personas - returns the full persona list including DB-only entries.
 */
export async function getPersonas(deps: PluginDeps): Promise<HmcsPersonaSnapshot[]> {
  return await hmcsFetch<HmcsPersonaSnapshot[]>(deps, '/personas', { method: 'GET' });
}

async function hmcsFetch<T>(deps: PluginDeps, path: string, opts: HmcsFetchOptions): Promise<T> {
  const url = `${deps.config.hmcsBaseUrl}${path}`;
  const init: RequestInit = { method: opts.method };
  if (opts.body !== undefined) {
    init.headers = { 'content-type': 'application/json' };
    init.body = JSON.stringify(opts.body);
  }
  const res = await fetch(url, init);
  if (!res.ok) {
    throw new Error(`${opts.method} ${path} failed: ${res.status} ${res.statusText}`);
  }
  return (await res.json()) as T;
}
