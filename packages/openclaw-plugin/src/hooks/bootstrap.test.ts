import { describe, expect, test, vi } from 'vitest';
import type { PluginDeps } from '../deps.js';
import { createPluginCache } from '../persona-cache.js';
import { createBootstrapHandler } from './bootstrap.js';

function makeDeps(): PluginDeps {
  const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
  return {
    api: { logger } as unknown as PluginDeps['api'],
    cache: createPluginCache(),
    config: { dhBaseUrl: 'http://x', reconcileIntervalSec: 30, soulMaxChars: 10000 },
    logger,
  };
}

describe('createBootstrapHandler', () => {
  test('pushes SOUL.md and IDENTITY.md into ctx.bootstrapFiles when persona is cached', async () => {
    const deps = makeDeps();
    deps.cache.upsertPersona({
      id: 'alice',
      name: 'Alice',
      metadata: {},
      spawned: true,
      personality: 'Kind',
    } as any);
    const handler = createBootstrapHandler(deps as any);
    const ctx = { agentId: 'alice', bootstrapFiles: [] };
    await handler(ctx as any);
    expect(ctx.bootstrapFiles).toHaveLength(2);
    expect(ctx.bootstrapFiles.find((f: any) => f.path === 'SOUL.md')).toBeDefined();
    expect(ctx.bootstrapFiles.find((f: any) => f.path === 'IDENTITY.md')).toBeDefined();
  });

  test('no-op when cache has no matching persona', async () => {
    const deps = makeDeps();
    const handler = createBootstrapHandler(deps as any);
    const ctx = { agentId: 'ghost', bootstrapFiles: [] };
    await handler(ctx as any);
    expect(ctx.bootstrapFiles).toHaveLength(0);
  });

  test('no-op when ctx.agentId is undefined', async () => {
    const deps = makeDeps();
    const handler = createBootstrapHandler(deps as any);
    const ctx = { bootstrapFiles: [] };
    await handler(ctx as any);
    expect(ctx.bootstrapFiles).toHaveLength(0);
  });

  test('preserves existing bootstrapFiles entries', async () => {
    const deps = makeDeps();
    deps.cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    const handler = createBootstrapHandler(deps as any);
    const ctx = {
      agentId: 'alice',
      bootstrapFiles: [{ path: 'USER.md', content: 'preserve me' }],
    };
    await handler(ctx as any);
    expect(ctx.bootstrapFiles).toHaveLength(3);
    expect(ctx.bootstrapFiles[0]?.path).toBe('USER.md');
  });
});
