import { createHash } from 'node:crypto';
import { promises as fs } from 'node:fs';
import { join } from 'node:path';
import type { PluginLogger } from '../deps.js';
import type { PluginCache } from '../persona-cache.js';
import { renderIdentity } from '../render/identity.js';
import { renderSoul } from '../render/soul.js';
import { errorMessage } from '../util/error.js';

const MAX_WRITE_RETRIES = 3;

export interface WritePersonaOptions {
  workspacePath: string;
  personaId: string;
  soulMaxChars: number;
}

export async function writePersonaFiles(
  cache: PluginCache,
  logger: PluginLogger,
  opts: WritePersonaOptions,
): Promise<void> {
  const entry = cache.personas.get(opts.personaId);
  if (!entry) {
    logger.warn(`writePersonaFiles: persona not in cache persona=${opts.personaId}`);
    return;
  }

  const soul = renderSoul(entry, opts.soulMaxChars);
  const identity = renderIdentity(entry, opts.soulMaxChars);

  if (!soul.trim() && !identity.trim()) {
    logger.warn(
      `writePersonaFiles: both SOUL and IDENTITY rendered empty, skipping persona=${opts.personaId}`,
    );
    return;
  }

  const hash = createHash('sha256').update(soul).update('\0').update(identity).digest('hex');

  if (entry.lastRenderedHash === hash) {
    logger.debug?.(`writePersonaFiles: hash unchanged, skipping persona=${opts.personaId}`);
    return;
  }

  await fs.mkdir(opts.workspacePath, { recursive: true });
  await atomicWrite(logger, join(opts.workspacePath, 'SOUL.md'), soul);
  await atomicWrite(logger, join(opts.workspacePath, 'IDENTITY.md'), identity);
  entry.lastRenderedHash = hash;
}

export async function deletePersonaFiles(
  logger: PluginLogger,
  workspacePath: string,
): Promise<void> {
  await safeUnlink(logger, join(workspacePath, 'SOUL.md'));
  await safeUnlink(logger, join(workspacePath, 'IDENTITY.md'));
}

async function atomicWrite(
  logger: PluginLogger,
  targetPath: string,
  contents: string,
): Promise<void> {
  const tmpPath = `${targetPath}.${process.pid}.${Date.now()}.tmp`;
  let attempt = 0;
  let lastErr: unknown = null;
  while (attempt < MAX_WRITE_RETRIES) {
    attempt++;
    try {
      await fs.writeFile(tmpPath, contents, { encoding: 'utf8' });
      await fs.rename(tmpPath, targetPath);
      return;
    } catch (err) {
      lastErr = err;
      // best-effort cleanup
      try {
        await fs.unlink(tmpPath);
      } catch {
        // ignore
      }
      if (attempt < MAX_WRITE_RETRIES) {
        await sleep(50 * attempt);
      }
    }
  }
  logger.warn(
    `writer: atomic write failed after retries path=${targetPath} err=${errorMessage(lastErr)}`,
  );
}

async function safeUnlink(logger: PluginLogger, path: string): Promise<void> {
  try {
    await fs.unlink(path);
  } catch (err) {
    const code = (err as NodeJS.ErrnoException).code;
    if (code === 'ENOENT') return;
    logger.warn(`writer: unlink failed path=${path} err=${errorMessage(err)}`);
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}
