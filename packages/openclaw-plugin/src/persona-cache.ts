import type { Gender, PersonaSnapshot } from '@hmcs/sdk';
import type { OpenClawAgentListEntry } from './types.js';

/**
 * Entry for an HMCS persona tracked by the plugin.
 */
export interface PersonaCacheEntry {
  personaId: string;
  name: string | null;
  personality: string | null;
  profile: string | null;
  age: number | null;
  gender: Gender;
  firstPersonPronoun: string | null;
  spawned: boolean;
  hasWarnedNoAgent: boolean;
  lastRenderedHash: string | null;
}

export interface AgentCacheEntry {
  agentId: string;
  workspacePath: string;
}

export interface PluginCache {
  personas: Map<string, PersonaCacheEntry>;
  agents: Map<string, AgentCacheEntry>;
  upsertPersona(snap: PersonaSnapshot): void;
  deletePersona(personaId: string): void;
  setSpawned(personaId: string, spawned: boolean): void;
  upsertAgent(entry: OpenClawAgentListEntry): void;
  deleteAgent(agentId: string): void;
}

export function createPluginCache(): PluginCache {
  const personas = new Map<string, PersonaCacheEntry>();
  const agents = new Map<string, AgentCacheEntry>();

  return {
    personas,
    agents,
    upsertPersona(snap) {
      const existing = personas.get(snap.id);
      personas.set(snap.id, {
        personaId: snap.id,
        name: snap.name ?? null,
        personality: snap.personality ?? null,
        profile: snap.profile ?? null,
        age: snap.age ?? null,
        gender: snap.gender ?? 'unknown',
        firstPersonPronoun: snap.firstPersonPronoun ?? null,
        spawned: snap.spawned,
        hasWarnedNoAgent: existing?.hasWarnedNoAgent ?? false,
        lastRenderedHash: existing?.lastRenderedHash ?? null,
      });
    },
    deletePersona(personaId) {
      personas.delete(personaId);
    },
    setSpawned(personaId, spawned) {
      const entry = personas.get(personaId);
      if (entry) entry.spawned = spawned;
    },
    upsertAgent(entry) {
      agents.set(entry.id, {
        agentId: entry.id,
        workspacePath: entry.workspace,
      });
      const persona = personas.get(entry.id);
      if (persona) persona.hasWarnedNoAgent = false;
    },
    deleteAgent(agentId) {
      agents.delete(agentId);
    },
  };
}
