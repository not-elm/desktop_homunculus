import { describe, expect, test, vi } from 'vitest';
import { createPluginCache } from '../persona-cache.js';
import { createSseController } from './sse.js';

class FakeEventSource {
  listeners = new Map<string, Array<(e: any) => void>>();
  closed = false;
  addEventListener(name: string, cb: (e: any) => void): void {
    if (!this.listeners.has(name)) this.listeners.set(name, []);
    this.listeners.get(name)!.push(cb);
  }
  emit(name: string, data: unknown): void {
    for (const cb of this.listeners.get(name) ?? []) cb({ data: JSON.stringify(data) });
  }
  close(): void {
    this.closed = true;
  }
}

function makeDeps(fakeFactory: () => FakeEventSource) {
  const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
  return {
    api: { runtime: { logger } } as any,
    cache: createPluginCache(),
    config: { hmcsBaseUrl: 'http://127.0.0.1:3100', soulMaxChars: 10000 },
    logger,
    eventSourceFactory: fakeFactory,
    writePersonaFiles: vi.fn(async () => undefined),
    deletePersonaFiles: vi.fn(async () => undefined),
  };
}

describe('createSseController', () => {
  test('persona-spawned sets spawned flag on existing cache entry', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertPersona({
      id: 'alice',
      name: 'Alice',
      metadata: {},
      spawned: false,
      personality: 'cheerful',
    } as any);
    createSseController(deps as any).start();

    fake.emit('persona-spawned', { personaId: 'alice' });
    expect(deps.cache.personas.get('alice')!.spawned).toBe(true);
    expect(deps.cache.personas.get('alice')!.personality).toBe('cheerful');
  });

  test('persona-spawned writes SOUL/IDENTITY if agent is in cache', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertPersona({
      id: 'alice',
      name: 'Alice',
      metadata: {},
      spawned: false,
      personality: 'cheerful',
    } as any);
    deps.cache.upsertAgent({ id: 'alice', workspace: '/tmp/alice' });
    createSseController(deps as any).start();
    fake.emit('persona-spawned', { personaId: 'alice' });
    expect(deps.writePersonaFiles).toHaveBeenCalled();
  });

  test('persona-spawned without prior cache entry does not crash', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    createSseController(deps as any).start();
    fake.emit('persona-spawned', { personaId: 'unknown' });
    expect(deps.cache.personas.has('unknown')).toBe(false);
    expect(deps.writePersonaFiles).not.toHaveBeenCalled();
  });

  test('persona-change reads persona data from nested persona key', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertAgent({ id: 'alice', workspace: '/tmp/alice' });
    deps.cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    const ctl = createSseController(deps as any);
    ctl.start();

    fake.emit('persona-change', {
      personaId: 'alice',
      persona: { id: 'alice', name: 'Alice Updated', personality: 'kind', metadata: {} },
    });
    expect(deps.cache.personas.get('alice')!.name).toBe('Alice Updated');
    expect(deps.cache.personas.get('alice')!.personality).toBe('kind');
    expect(deps.writePersonaFiles).toHaveBeenCalled();
  });

  test('persona-change coerces unknown gender strings to "unknown"', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertAgent({ id: 'alice', workspace: '/tmp/alice' });
    deps.cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    createSseController(deps as any).start();

    fake.emit('persona-change', {
      personaId: 'alice',
      persona: { id: 'alice', gender: 'bogus-value', metadata: {} },
    });
    expect(deps.cache.personas.get('alice')!.gender).toBe('unknown');
  });

  test('persona-despawned flips spawned flag but does not delete', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    createSseController(deps as any).start();

    fake.emit('persona-despawned', { personaId: 'alice' });
    expect(deps.cache.personas.get('alice')!.spawned).toBe(false);
  });

  test('persona-deleted removes files and cache entry', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertAgent({ id: 'alice', workspace: '/tmp/alice' });
    deps.cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    createSseController(deps as any).start();

    fake.emit('persona-deleted', { personaId: 'alice' });
    expect(deps.deletePersonaFiles).toHaveBeenCalledWith(deps.logger, '/tmp/alice');
    expect(deps.cache.personas.has('alice')).toBe(false);
  });

  test('stop() closes the EventSource', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    const ctl = createSseController(deps as any);
    ctl.start();
    ctl.stop();
    expect(fake.closed).toBe(true);
  });

  test('state-change / vrm-attached / vrm-detached are ignored (no write)', () => {
    const fake = new FakeEventSource();
    const deps = makeDeps(() => fake);
    deps.cache.upsertAgent({ id: 'alice', workspace: '/tmp/alice' });
    deps.cache.upsertPersona({ id: 'alice', name: 'A', metadata: {}, spawned: true } as any);
    createSseController(deps as any).start();
    fake.emit('state-change', { personaId: 'alice', state: 'idle' });
    fake.emit('vrm-attached', { personaId: 'alice' });
    fake.emit('vrm-detached', { personaId: 'alice' });
    expect(deps.writePersonaFiles).not.toHaveBeenCalled();
  });
});
