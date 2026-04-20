import type { PluginDeps } from '../deps.js';
import { getPersonas } from '../hmcs-client.js';
import type { HmcsPersonaSnapshot, OpenClawAgentListEntry } from '../types.js';
import { errorMessage } from '../util/error.js';
import type { OpenClawCli } from './openclaw-cli.js';
import { writePersonaFiles as defaultWritePersonaFiles } from './writer.js';

export interface SeedDeps extends PluginDeps {
  cli: OpenClawCli;
  // Injected for testability (default = writer.writePersonaFiles)
  writePersonaFiles?: typeof defaultWritePersonaFiles;
}

export async function seedFromHmcs(deps: SeedDeps): Promise<void> {
  const { cache, logger, config } = deps;
  const write = deps.writePersonaFiles ?? defaultWritePersonaFiles;

  const personas = await fetchPersonas(deps);
  if (personas === null) return;
  const agents = await fetchAgents(deps);

  reconcilePersonaCache(cache, personas);
  reconcileAgentCache(cache, agents);

  for (const p of personas) {
    if (!p.spawned) continue;
    const personaEntry = cache.personas.get(p.id)!;
    const agentEntry = cache.agents.get(p.id);
    if (agentEntry) {
      await write(cache, logger, {
        workspacePath: agentEntry.workspacePath,
        personaId: p.id,
        soulMaxChars: config.soulMaxChars,
      });
    } else if (!personaEntry.hasWarnedNoAgent) {
      logger.warn(
        `Persona \`${p.id}\` has no matching OpenClaw agent. Create one with \`openclaw agents add ${p.id}\`.`,
      );
      personaEntry.hasWarnedNoAgent = true;
    }
  }
}

async function fetchPersonas(deps: SeedDeps): Promise<HmcsPersonaSnapshot[] | null> {
  try {
    return await getPersonas(deps);
  } catch (err) {
    deps.logger.warn(
      `seed: GET /personas failed, will retry on reconciler tick err=${errorMessage(err)}`,
    );
    return null;
  }
}

async function fetchAgents(deps: SeedDeps): Promise<OpenClawAgentListEntry[]> {
  try {
    return await deps.cli.agentsList();
  } catch (err) {
    deps.logger.warn(`seed: openclaw agents list failed err=${errorMessage(err)}`);
    return [];
  }
}

function reconcilePersonaCache(cache: SeedDeps['cache'], personas: HmcsPersonaSnapshot[]): void {
  const seen = new Set<string>();
  for (const p of personas) {
    cache.upsertPersona(p);
    seen.add(p.id);
  }
  for (const id of cache.personas.keys()) {
    if (!seen.has(id)) cache.deletePersona(id);
  }
}

function reconcileAgentCache(cache: SeedDeps['cache'], agents: OpenClawAgentListEntry[]): void {
  const seen = new Set<string>();
  for (const a of agents) {
    cache.upsertAgent(a);
    seen.add(a.id);
  }
  for (const id of cache.agents.keys()) {
    if (!seen.has(id)) cache.deleteAgent(id);
  }
}
