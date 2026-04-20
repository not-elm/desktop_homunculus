import type { PluginDeps } from '../deps.js';
import { renderSoul } from '../render/soul.js';
import { renderIdentity } from '../render/identity.js';

interface BootstrapCtx {
  agentId?: string;
  bootstrapFiles?: Array<{ path: string; content: string }>;
}

/**
 * Creates the `agent.bootstrap` hook handler.
 *
 * Pipeline:
 *   1. identity mapping: `ctx.agentId` -> persona cache entry (1:1)
 *   2. render SOUL.md (tone/profile/pronoun) and IDENTITY.md (name/age/gender)
 *   3. push both files into `ctx.bootstrapFiles` (mutable array) without clobbering
 *      any pre-existing entries (e.g., USER.md injected upstream).
 *
 * No-op when `ctx.agentId` is undefined, when the persona is not cached, or when
 * `ctx.bootstrapFiles` is not an array (defensive against upstream contract changes).
 *
 * The handler's argument is typed as `unknown` because the SDK's
 * `InternalHookEvent` carries hook-specific payloads under `event.context`;
 * we narrow at runtime against our own `BootstrapCtx` shape.
 */
export function createBootstrapHandler(deps: PluginDeps) {
  const { cache, config } = deps;
  return async (ctx: unknown): Promise<void> => {
    const narrowed = ctx as BootstrapCtx;
    if (!narrowed.agentId) return;
    const persona = cache.personas.get(narrowed.agentId);
    if (!persona) return;
    if (!Array.isArray(narrowed.bootstrapFiles)) return;
    narrowed.bootstrapFiles.push(
      { path: 'SOUL.md', content: renderSoul(persona, config.soulMaxChars) },
      { path: 'IDENTITY.md', content: renderIdentity(persona, config.soulMaxChars) },
    );
  };
}
