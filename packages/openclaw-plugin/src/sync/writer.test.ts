import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';
import { mkdtempSync, readFileSync, rmSync, existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { writePersonaFiles, deletePersonaFiles } from './writer.js';
import { createPluginCache } from '../persona-cache.js';

let baseDir: string;

beforeEach(() => {
  baseDir = mkdtempSync(join(tmpdir(), 'hmcs-openclaw-writer-'));
});

afterEach(() => {
  rmSync(baseDir, { recursive: true, force: true });
});

function setupPersona() {
  const cache = createPluginCache();
  cache.upsertPersona({
    id: 'alice',
    name: 'Alice',
    metadata: {},
    spawned: true,
    personality: 'Kind',
    profile: 'Lives on a desktop',
  } as any);
  return cache;
}

const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };

describe('writePersonaFiles', () => {
  test('creates SOUL.md and IDENTITY.md atomically', async () => {
    const cache = setupPersona();
    const entry = cache.personas.get('alice')!;
    mkdirSync(baseDir, { recursive: true });

    await writePersonaFiles(
      { personas: cache.personas, agents: cache.agents } as any,
      logger,
      { workspacePath: baseDir, personaId: 'alice', soulMaxChars: 10000 },
    );

    expect(existsSync(join(baseDir, 'SOUL.md'))).toBe(true);
    expect(existsSync(join(baseDir, 'IDENTITY.md'))).toBe(true);
    const soul = readFileSync(join(baseDir, 'SOUL.md'), 'utf8');
    expect(soul).toContain('Kind');
    expect(entry.lastRenderedHash).not.toBeNull();
  });

  test('skips write when content unchanged (lastRenderedHash match)', async () => {
    const cache = setupPersona();
    await writePersonaFiles(
      { personas: cache.personas, agents: cache.agents } as any,
      logger,
      { workspacePath: baseDir, personaId: 'alice', soulMaxChars: 10000 },
    );
    const soulPath = join(baseDir, 'SOUL.md');
    const originalMtime = (await import('node:fs/promises')).stat(soulPath);
    const firstStat = await originalMtime;

    // Wait to ensure mtime would differ if a write actually happened.
    await new Promise((r) => setTimeout(r, 20));

    await writePersonaFiles(
      { personas: cache.personas, agents: cache.agents } as any,
      logger,
      { workspacePath: baseDir, personaId: 'alice', soulMaxChars: 10000 },
    );
    const secondStat = await (await import('node:fs/promises')).stat(soulPath);
    expect(secondStat.mtimeMs).toBe(firstStat.mtimeMs);
  });

  test('rewrites when persona content changes (hash differs)', async () => {
    const cache = setupPersona();
    await writePersonaFiles(
      { personas: cache.personas, agents: cache.agents } as any,
      logger,
      { workspacePath: baseDir, personaId: 'alice', soulMaxChars: 10000 },
    );
    // Mutate persona
    cache.personas.get('alice')!.personality = 'Changed';

    await writePersonaFiles(
      { personas: cache.personas, agents: cache.agents } as any,
      logger,
      { workspacePath: baseDir, personaId: 'alice', soulMaxChars: 10000 },
    );
    const soul = readFileSync(join(baseDir, 'SOUL.md'), 'utf8');
    expect(soul).toContain('Changed');
  });

  test('overwrites even when a pre-existing user-edited SOUL.md exists', async () => {
    writeFileSync(join(baseDir, 'SOUL.md'), 'USER EDITED', 'utf8');
    const cache = setupPersona();
    await writePersonaFiles(
      { personas: cache.personas, agents: cache.agents } as any,
      logger,
      { workspacePath: baseDir, personaId: 'alice', soulMaxChars: 10000 },
    );
    const soul = readFileSync(join(baseDir, 'SOUL.md'), 'utf8');
    expect(soul).not.toContain('USER EDITED');
    expect(soul).toContain('Kind');
  });
});

describe('deletePersonaFiles', () => {
  test('removes both files in SOUL→IDENTITY order; ENOENT is success', async () => {
    writeFileSync(join(baseDir, 'SOUL.md'), 'x', 'utf8');
    writeFileSync(join(baseDir, 'IDENTITY.md'), 'y', 'utf8');
    await deletePersonaFiles(logger, baseDir);
    expect(existsSync(join(baseDir, 'SOUL.md'))).toBe(false);
    expect(existsSync(join(baseDir, 'IDENTITY.md'))).toBe(false);
  });

  test('missing files are silently ignored', async () => {
    await expect(deletePersonaFiles(logger, baseDir)).resolves.not.toThrow();
  });
});
