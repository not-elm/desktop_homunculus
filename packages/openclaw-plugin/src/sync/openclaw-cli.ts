import type { OpenClawConfig, PluginLogger } from 'openclaw/plugin-sdk/plugin-entry';
import type { OpenClawAgentListEntry } from '../types.js';
import { errorMessage } from '../util/error.js';

/**
 * Injected surface the CLI wrapper reads from. In production wired from the
 * OpenClaw plugin SDK — `api.config` + `api.runtime.agent.resolveAgentWorkspaceDir`.
 */
export interface OpenClawAgentSource {
  config: OpenClawConfig;
  resolveAgentWorkspaceDir: (cfg: OpenClawConfig, agentId: string) => string;
}

export interface OpenClawCli {
  agentsList(): Promise<OpenClawAgentListEntry[]>;
}

/**
 * Reads the agent list from the OpenClaw plugin SDK config. Singleflights
 * concurrent calls. On any error (missing config, resolver throw) returns []
 * and warns once.
 *
 * NOTE: this file intentionally does NOT use any subprocess or shell call.
 * See docs/superpowers/specs/2026-04-18-openclaw-agent-integration-design.md
 * §6.2 for the rationale (OpenClaw security scanner blocks dangerous-exec).
 */
export function createOpenClawCli(logger: PluginLogger, source: OpenClawAgentSource): OpenClawCli {
  let inFlight: Promise<OpenClawAgentListEntry[]> | null = null;
  let warnedOnEmpty = false;

  async function readOnce(): Promise<OpenClawAgentListEntry[]> {
    try {
      const cfg = source.config;
      const list = cfg?.agents?.list;
      const rawList = Array.isArray(list) ? list : [];
      const ids: string[] = [];
      for (const raw of rawList) {
        if (raw && typeof raw.id === 'string') {
          const id = raw.id.trim();
          if (id.length > 0) ids.push(id);
        }
      }

      if (ids.length === 0 && !warnedOnEmpty) {
        warnedOnEmpty = true;
        logger.warn(
          'OpenClaw config has no agents.list entries. Agent sync will be inactive until `openclaw agents add <persona.id>` is run.',
        );
      }

      const unique = Array.from(new Set(ids));
      return unique.map((id) => ({
        id,
        workspace: source.resolveAgentWorkspaceDir(cfg, id),
      }));
    } catch (err) {
      logger.warn(`Failed to read OpenClaw agents from SDK config err=${errorMessage(err)}`);
      return [];
    }
  }

  return {
    agentsList() {
      if (inFlight) return inFlight;
      inFlight = readOnce().finally(() => {
        inFlight = null;
      });
      return inFlight;
    },
  };
}
