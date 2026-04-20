import { beforeEach, describe, expect, test, vi } from 'vitest';
import type { PluginDeps } from './deps.js';
import { getPersonas } from './dh-client.js';

function makeDeps(): PluginDeps {
  const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
  return {
    api: { runtime: { logger } },
    cache: { personas: new Map(), agents: new Map() } as any,
    config: { dhBaseUrl: 'http://127.0.0.1:3100', reconcileIntervalSec: 30, soulMaxChars: 10000 },
    logger,
  };
}

describe('getPersonas', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  test('GETs {dhBaseUrl}/personas and returns JSON body', async () => {
    const mockPersonas = [{ id: 'alice', name: 'Alice', metadata: {}, spawned: true }];
    const fetchMock = vi
      .spyOn(globalThis, 'fetch')
      .mockResolvedValueOnce(new Response(JSON.stringify(mockPersonas), { status: 200 }));

    const deps = makeDeps();
    const result = await getPersonas(deps);

    expect(fetchMock).toHaveBeenCalledWith(
      'http://127.0.0.1:3100/personas',
      expect.objectContaining({ method: 'GET' }),
    );
    expect(result).toEqual(mockPersonas);
  });

  test('throws on non-2xx', async () => {
    vi.spyOn(globalThis, 'fetch').mockResolvedValueOnce(new Response('err', { status: 500 }));
    const deps = makeDeps();
    await expect(getPersonas(deps)).rejects.toThrow(/500/);
  });
});
