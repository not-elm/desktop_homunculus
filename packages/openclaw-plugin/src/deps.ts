import type { OpenClawPluginApi, PluginLogger } from 'openclaw/plugin-sdk/plugin-entry';
import type { PluginCache } from './persona-cache.js';

export type { PluginLogger };

export interface PluginConfig {
  dhBaseUrl: string;
  reconcileIntervalSec: number;
  soulMaxChars: number;
}

/**
 * Dependency container passed to every handler factory and sync function.
 * See spec §5 "DI パターン (v12 で確定)".
 */
export interface PluginDeps {
  api: OpenClawPluginApi;
  cache: PluginCache;
  config: PluginConfig;
  logger: PluginLogger;
}
