import { Persona } from '@hmcs/sdk';
import { afterEach, describe, expect, test, vi } from 'vitest';
import { createPluginCache } from '../persona-cache.js';
import { seedFromHmcs } from './seed.js';

afterEach(() => {
  vi.restoreAllMocks();
});

function makePersona(overrides: Record<string, unknown> = {}) {
  return {
    id: 'alice',
    name: 'Alice',
    age: null,
    gender: 'unknown',
    firstPersonPronoun: null,
    profile: '',
    personality: null,
    state: '',
    metadata: {},
    spawned: true,
    ...overrides,
  };
}

function makeDeps(overrides: any = {}) {
  const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
  return {
    api: { runtime: { logger } } as any,
    cache: createPluginCache(),
    config: { hmcsBaseUrl: 'http://127.0.0.1:3100', soulMaxChars: 10000 },
    logger,
    cli: {
      agentsList: vi.fn(async () => []),
    },
    writePersonaFiles: vi.fn(async () => undefined),
    ...overrides,
  };
}

describe('seedFromHmcs', () => {
  test('inserts personas; for those with matching agents, calls writePersonaFiles', async () => {
    const personas = [
      makePersona({ id: 'alice', name: 'Alice', personality: 'kind' }),
      makePersona({ id: 'bob', name: 'Bob' }),
    ];
    vi.spyOn(Persona, 'list').mockResolvedValue(personas as any);

    const deps = makeDeps({
      cli: { agentsList: vi.fn(async () => [{ id: 'alice', workspace: '/tmp/alice' }]) },
    });
    await seedFromHmcs(deps as any);

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
    vi.spyOn(Persona, 'list').mockResolvedValue([makePersona({ id: 'bob', name: 'Bob' })] as any);
    const deps = makeDeps();
    await seedFromHmcs(deps as any);
    expect(deps.logger.warn).toHaveBeenCalledWith(expect.stringContaining('Persona `bob`'));
    expect(deps.cache.personas.get('bob')?.hasWarnedNoAgent).toBe(true);
  });

  test('does not warn again for the same persona across seed calls', async () => {
    vi.spyOn(Persona, 'list').mockResolvedValue([makePersona({ id: 'bob', name: 'Bob' })] as any);
    const deps = makeDeps();
    await seedFromHmcs(deps as any);
    await seedFromHmcs(deps as any);
    expect(deps.logger.warn).toHaveBeenCalledTimes(1);
  });

  test('skips DB-only persona (spawned=false)', async () => {
    vi.spyOn(Persona, 'list').mockResolvedValue([
      makePersona({ id: 'db', name: 'DB', spawned: false }),
    ] as any);
    const deps = makeDeps({
      cli: { agentsList: vi.fn(async () => [{ id: 'db', workspace: '/tmp/db' }]) },
    });
    await seedFromHmcs(deps as any);
    expect(deps.writePersonaFiles).not.toHaveBeenCalled();
  });

  test('does not throw when HMCS Persona.list fails (logs warning)', async () => {
    vi.spyOn(Persona, 'list').mockRejectedValue(new Error('ECONNREFUSED'));
    const deps = makeDeps();
    await expect(seedFromHmcs(deps as any)).resolves.not.toThrow();
    expect(deps.logger.warn).toHaveBeenCalled();
  });
});
