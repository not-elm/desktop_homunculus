import type { PluginDeps } from './deps.js';
import type { DhPersonaSnapshot } from './types.js';

interface DhFetchOptions {
  method: 'GET' | 'POST';
  body?: unknown;
}

/**
 * GET /personas - returns the full persona list including DB-only entries.
 */
export async function getPersonas(deps: PluginDeps): Promise<DhPersonaSnapshot[]> {
  return await dhFetch<DhPersonaSnapshot[]>(deps, '/personas', { method: 'GET' });
}

async function dhFetch<T>(deps: PluginDeps, path: string, opts: DhFetchOptions): Promise<T> {
  const url = `${deps.config.dhBaseUrl}${path}`;
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
