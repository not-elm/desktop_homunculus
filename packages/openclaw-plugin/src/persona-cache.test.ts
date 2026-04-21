import { describe, expect, test } from 'vitest';
import { createPluginCache } from './persona-cache.js';
import { getRequired, makePersonaSnapshot } from './testing.js';

describe('PluginCache', () => {
  test('upsertPersona adds a new persona entry with defaults', () => {
    const cache = createPluginCache();
    cache.upsertPersona(makePersonaSnapshot({ id: 'alice', name: 'Alice', spawned: true }));
    const entry = cache.personas.get('alice');
    expect(entry).toBeDefined();
    expect(entry?.personaId).toBe('alice');
    expect(entry?.spawned).toBe(true);
    expect(entry?.hasWarnedNoAgent).toBe(false);
    expect(entry?.lastRenderedHash).toBeNull();
  });

  test('upsertPersona preserves hasWarnedNoAgent and lastRenderedHash on update', () => {
    const cache = createPluginCache();
    cache.upsertPersona(makePersonaSnapshot({ id: 'alice', name: 'Alice', spawned: true }));
    const first = getRequired(cache.personas, 'alice');
    first.hasWarnedNoAgent = true;
    first.lastRenderedHash = 'abc123';

    cache.upsertPersona(makePersonaSnapshot({ id: 'alice', name: 'Alice Updated', spawned: true }));

    const updated = getRequired(cache.personas, 'alice');
    expect(updated.name).toBe('Alice Updated');
    expect(updated.hasWarnedNoAgent).toBe(true);
    expect(updated.lastRenderedHash).toBe('abc123');
  });

  test('deletePersona removes entry', () => {
    const cache = createPluginCache();
    cache.upsertPersona(makePersonaSnapshot({ id: 'alice', name: 'A', spawned: true }));
    cache.deletePersona('alice');
    expect(cache.personas.has('alice')).toBe(false);
  });

  test('setSpawned toggles flag without touching other fields', () => {
    const cache = createPluginCache();
    cache.upsertPersona(makePersonaSnapshot({ id: 'alice', name: 'A', spawned: true }));
    cache.setSpawned('alice', false);
    expect(cache.personas.get('alice')?.spawned).toBe(false);
  });

  test('upsertAgent / deleteAgent / resetAgentWarningsOnAdd', () => {
    const cache = createPluginCache();
    cache.upsertPersona(makePersonaSnapshot({ id: 'alice', name: 'A', spawned: true }));
    getRequired(cache.personas, 'alice').hasWarnedNoAgent = true;

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
