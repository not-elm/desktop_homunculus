import { describe, expect, test, vi } from 'vitest';
import * as dhClient from '../dh-client.js';
import { createPluginCache } from '../persona-cache.js';
import { seedFromDh } from './seed.js';

function makeDeps(overrides: any = {}) {
  const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
  return {
    api: { runtime: { logger } } as any,
    cache: createPluginCache(),
    config: { dhBaseUrl: 'http://127.0.0.1:3100', reconcileIntervalSec: 30, soulMaxChars: 10000 },
    logger,
    cli: {
      agentsList: vi.fn(async () => []),
    },
    writePersonaFiles: vi.fn(async () => undefined),
    ...overrides,
  };
}

describe('seedFromDh', () => {
  test('inserts personas; for those with matching agents, calls writePersonaFiles', async () => {
    const personas = [
      { id: 'alice', name: 'Alice', metadata: {}, spawned: true, personality: 'kind' },
      { id: 'bob', name: 'Bob', metadata: {}, spawned: true },
    ];
    vi.spyOn(dhClient, 'getPersonas').mockResolvedValue(personas as any);

    const deps = makeDeps({
      cli: { agentsList: vi.fn(async () => [{ id: 'alice', workspace: '/tmp/alice' }]) },
    });
    await seedFromDh(deps as any);

    expect(deps.cache.personas.get('alice')).toBeDefined();
    expect(deps.cache.personas.get('bob')).toBeDefined();
    expect(deps.cache.agents.get('alice')).toBeDefined();
    expect(deps.cache.agents.get('bob')).toBeUndefined();
    expect(deps.writePersonaFiles).toHaveBeenCalledWith(
      deps.cache,
      deps.logger,
      expect.objectContaining({ personaId: 'alice', workspacePath: '/tmp/alice' }),
    );
    expect(deps.writePersonaFiles).not.toHaveBeenCalledWith(
      deps.cache,
      deps.logger,
      expect.objectContaining({ personaId: 'bob' }),
    );
  });

  test('warns once when persona has no matching agent', async () => {
    vi.spyOn(dhClient, 'getPersonas').mockResolvedValue([
      { id: 'bob', name: 'Bob', metadata: {}, spawned: true },
    ] as any);
    const deps = makeDeps();
    await seedFromDh(deps as any);
    expect(deps.logger.warn).toHaveBeenCalledWith(expect.stringContaining('Persona `bob`'));
    expect(deps.cache.personas.get('bob')!.hasWarnedNoAgent).toBe(true);
  });

  test('does not warn again for the same persona across seed calls', async () => {
    vi.spyOn(dhClient, 'getPersonas').mockResolvedValue([
      { id: 'bob', name: 'Bob', metadata: {}, spawned: true },
    ] as any);
    const deps = makeDeps();
    await seedFromDh(deps as any);
    await seedFromDh(deps as any);
    expect(deps.logger.warn).toHaveBeenCalledTimes(1);
  });

  test('skips DB-only persona (spawned=false)', async () => {
    vi.spyOn(dhClient, 'getPersonas').mockResolvedValue([
      { id: 'db', name: 'DB', metadata: {}, spawned: false },
    ] as any);
    const deps = makeDeps({
      cli: { agentsList: vi.fn(async () => [{ id: 'db', workspace: '/tmp/db' }]) },
    });
    await seedFromDh(deps as any);
    expect(deps.writePersonaFiles).not.toHaveBeenCalled();
  });

  test('does not throw when DH getPersonas fails (logs warning)', async () => {
    vi.spyOn(dhClient, 'getPersonas').mockRejectedValue(new Error('ECONNREFUSED'));
    const deps = makeDeps();
    await expect(seedFromDh(deps as any)).resolves.not.toThrow();
    expect(deps.logger.warn).toHaveBeenCalled();
  });
});
