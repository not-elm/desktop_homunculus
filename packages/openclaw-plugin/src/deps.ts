import type { OpenClawPluginApi, PluginLogger } from 'openclaw/plugin-sdk/plugin-entry';
import type { PluginCache } from './persona-cache.js';

export type { PluginLogger };

export interface PluginConfig {
  hmcsBaseUrl: string;
  soulMaxChars: number;
}

/**
 * Dependency container passed to every handler factory and sync function.
 */
export interface PluginDeps {
  api: OpenClawPluginApi;
  cache: PluginCache;
  config: PluginConfig;
  logger: PluginLogger;
}
