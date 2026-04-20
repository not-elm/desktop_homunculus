import type { OpenClawPluginApi } from 'openclaw/plugin-sdk/plugin-entry';
import type { PluginDeps } from '../deps.js';
import { createOpenClawCli, type OpenClawAgentSource } from './openclaw-cli.js';
import { startReconciler as startReconcilerRaw } from './reconciler.js';
import { type SeedDeps, seedFromHmcs as seedRaw } from './seed.js';
import { createSseController } from './sse.js';

/**
 * Builds an OpenClawAgentSource from the plugin api. Resolvers are guaranteed
 * by the SDK's `PluginRuntime.agent` surface; config is `OpenClawConfig`.
 */
function buildAgentSource(api: OpenClawPluginApi): OpenClawAgentSource {
  return {
    config: api.config,
    resolveAgentWorkspaceDir: api.runtime.agent.resolveAgentWorkspaceDir,
  };
}

/**
 * Wires seed + sse + reconciler into a single start/stop unit.
 * Called from entry.ts inside registerService's start().
 */
export function createSyncRunner(deps: PluginDeps) {
  const cli = createOpenClawCli(deps.logger, buildAgentSource(deps.api));
  const seedDeps: SeedDeps = { ...deps, cli };
  const sse = createSseController({ ...deps });
  let stopReconciler: (() => void) | null = null;

  return {
    async start(): Promise<void> {
      await seedRaw(seedDeps);
      sse.start();
      stopReconciler = startReconcilerRaw(seedDeps);
    },
    async stop(): Promise<void> {
      stopReconciler?.();
      stopReconciler = null;
      sse.stop();
    },
  };
}
