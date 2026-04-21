import { describe, expect, test } from 'vitest';
import { createPluginCache } from './persona-cache.js';

describe('PluginCache', () => {
  test('upsertPersona adds a new persona entry with defaults', () => {
    const cache = createPluginCache();
    cache.upsertPersona({
      id: 'alice',
      name: 'Alice',
      metadata: {},
      spawned: true,
    } as any);
    const entry = cache.personas.get('alice');
    expect(entry).toBeDefined();
    expect(entry?.personaId).toBe('alice');
    expect(entry?.spawned).toBe(true);
    expect(entry?.hasWarnedNoAgent).toBe(false);
    expect(entry?.lastRenderedHash).toBeNull();
  });

  test('upsertPersona preserves hasWarnedNoAgent and lastRenderedHash on update', () => {
    const cache = createPluginCache();
    cache.upsertPersona({
      id: 'alice',
      name: 'Alice',
      metadata: {},
      spawned: true,
    } as any);
    const first = cache.personas.get('alice')!;
    first.hasWarnedNoAgent = true;
    first.lastRenderedHash = 'abc123';

    cache.upsertPersona({
      id: 'alice',
      name: 'Alice Updated',
      metadata: {},
      spawned: true,
    } as any);

    const updated = cache.personas.get('alice')!;
    expect(updated.name).toBe('Alice Updated');
    expect(updated.hasWarnedNoAgent).toBe(true);
    expect(updated.lastRenderedHash).toBe('abc123');
  });

  test('deletePersona removes entry', () => {
    const cache = createPluginCache();
    cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    cache.deletePersona('alice');
    expect(cache.personas.has('alice')).toBe(false);
  });

  test('setSpawned toggles flag without touching other fields', () => {
    const cache = createPluginCache();
    cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    cache.setSpawned('alice', false);
    expect(cache.personas.get('alice')?.spawned).toBe(false);
  });

  test('upsertAgent / deleteAgent / resetAgentWarningsOnAdd', () => {
    const cache = createPluginCache();
    cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    cache.personas.get('alice')!.hasWarnedNoAgent = true;

    cache.upsertAgent({ id: 'alice', workspace: '/tmp/alice' });
    expect(cache.agents.get('alice')).toEqual({
      agentId: 'alice',
      workspacePath: '/tmp/alice',
    });
    // agent が加わった時点で persona 側の warning flag が reset される
    expect(cache.personas.get('alice')?.hasWarnedNoAgent).toBe(false);

    cache.deleteAgent('alice');
    expect(cache.agents.has('alice')).toBe(false);
  });
});
