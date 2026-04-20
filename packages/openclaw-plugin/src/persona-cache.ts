import type { HmcsPersonaSnapshot, OpenClawAgentListEntry } from './types.js';

/**
 * Entry for an HMCS persona tracked by the plugin.
 */
export interface PersonaCacheEntry {
  personaId: string;
  name: string;
  personality: string | null;
  profile: string | null;
  age: number | null;
  gender: string | null;
  firstPersonPronoun: string | null;
  ttsModName: string;
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
  upsertPersona(snap: HmcsPersonaSnapshot): void;
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
      const rawTtsModName = snap.metadata?.ttsModName;
      const ttsModName = typeof rawTtsModName === 'string' ? rawTtsModName : '@hmcs/voicevox';
      personas.set(snap.id, {
        personaId: snap.id,
        name: snap.name,
        personality: snap.personality ?? null,
        profile: snap.profile ?? null,
        age: snap.age ?? null,
        gender: snap.gender ?? null,
        firstPersonPronoun: snap.firstPersonPronoun ?? null,
        ttsModName,
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
