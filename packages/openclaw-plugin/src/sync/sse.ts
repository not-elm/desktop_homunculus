import { type Gender, host, type PersonaSnapshot } from '@hmcs/sdk';
import type { PluginDeps, PluginLogger } from '../deps.js';
import type { PluginCache } from '../persona-cache.js';
import { errorMessage } from '../util/error.js';
import {
  deletePersonaFiles as defaultDeletePersonaFiles,
  writePersonaFiles as defaultWritePersonaFiles,
} from './writer.js';

export interface SseDeps extends PluginDeps {
  eventSourceFactory?: (url: string) => MinimalEventSource;
  writePersonaFiles?: typeof defaultWritePersonaFiles;
  deletePersonaFiles?: typeof defaultDeletePersonaFiles;
}

/**
 * Subset of EventSource we rely on. Production uses the `eventsource` package.
 */
export interface MinimalEventSource {
  addEventListener(name: string, cb: (e: { data: string }) => void): void;
  close(): void;
}

export interface SseController {
  start(): void;
  stop(): void;
}

/**
 * Minimal EventSource constructor type. Matches both the `eventsource`
 * package's class shape and mock factories used in tests.
 */
type EventSourceCtor = new (
  url: string,
) => {
  addEventListener(name: string, cb: (e: { data: string }) => void): void;
  close(): void;
};

type SsePayload = Record<string, unknown>;

export function createSseController(deps: SseDeps): SseController {
  const write = deps.writePersonaFiles ?? defaultWritePersonaFiles;
  const del = deps.deletePersonaFiles ?? defaultDeletePersonaFiles;
  let es: MinimalEventSource | null = null;

  return {
    start() {
      const url = host.createUrl('personas/stream').toString();
      es = deps.eventSourceFactory
        ? deps.eventSourceFactory(url)
        : defaultEventSourceFactory(url, deps.logger);

      const handleUpsert = (data: string) => {
        const payload = safeParse(data, deps.logger);
        if (!payload) return;
        const snap = buildPersonaSnapshot(payload);
        if (!snap) return;
        deps.cache.upsertPersona(snap);
        maybeWrite(deps.cache, deps.logger, snap.id, write, deps.config.soulMaxChars);
      };

      es.addEventListener('persona-spawned', (e) => {
        const payload = safeParse(e.data, deps.logger);
        if (!payload) return;
        const personaId = readString(payload, 'personaId');
        if (!personaId) return;
        deps.cache.setSpawned(personaId, true);
        maybeWrite(deps.cache, deps.logger, personaId, write, deps.config.soulMaxChars);
      });
      es.addEventListener('persona-change', (e) => handleUpsert(e.data));

      es.addEventListener('persona-despawned', (e) => {
        const payload = safeParse(e.data, deps.logger);
        if (!payload) return;
        const personaId = readString(payload, 'personaId');
        if (!personaId) return;
        deps.cache.setSpawned(personaId, false);
      });

      es.addEventListener('persona-deleted', (e) => {
        const payload = safeParse(e.data, deps.logger);
        if (!payload) return;
        const personaId = readString(payload, 'personaId');
        if (!personaId) return;
        const agent = deps.cache.agents.get(personaId);
        if (agent) {
          del(deps.logger, agent.workspacePath).catch(() => {
            // already logged inside deletePersonaFiles
          });
        }
        deps.cache.deletePersona(personaId);
      });
    },
    stop() {
      es?.close();
      es = null;
    },
  };
}

function buildPersonaSnapshot(payload: SsePayload): PersonaSnapshot | null {
  const id = readString(payload, 'personaId');
  if (!id) return null;

  // persona-change events nest persona data under "persona" key
  const persona = payload.persona;
  const src: SsePayload =
    persona && typeof persona === 'object' ? (persona as SsePayload) : payload;

  const rawMeta = src.metadata;
  const metadata =
    rawMeta && typeof rawMeta === 'object' ? (rawMeta as Record<string, unknown>) : {};
  return {
    id,
    name: readString(src, 'name') ?? id,
    metadata,
    spawned: typeof payload.spawned === 'boolean' ? payload.spawned : true,
    personality: readString(src, 'personality'),
    profile: readString(src, 'profile') ?? '',
    age: readNumber(src, 'age'),
    gender: toGender(src.gender),
    firstPersonPronoun: readString(src, 'firstPersonPronoun'),
    vrmAssetId: readString(src, 'vrmAssetId'),
    state: readString(src, 'state') ?? '',
  };
}

function readString(obj: SsePayload, key: string): string | null {
  const v = obj[key];
  return typeof v === 'string' ? v : null;
}

function readNumber(obj: SsePayload, key: string): number | null {
  const v = obj[key];
  return typeof v === 'number' ? v : null;
}

/**
 * Coerces a raw SSE value into SDK's `Gender` union. Unknown / missing values
 * fall back to `'unknown'` so downstream consumers see a valid enum member.
 */
function toGender(v: unknown): Gender {
  if (v === 'male' || v === 'female' || v === 'other') return v;
  return 'unknown';
}

function maybeWrite(
  cache: PluginCache,
  logger: PluginLogger,
  personaId: string,
  write: typeof defaultWritePersonaFiles,
  soulMaxChars: number,
): void {
  const persona = cache.personas.get(personaId);
  if (!persona) return;
  const agent = cache.agents.get(personaId);
  if (!agent) return;
  void write(cache, logger, {
    workspacePath: agent.workspacePath,
    personaId,
    soulMaxChars,
  }).catch(() => {
    // already logged inside writePersonaFiles
  });
}

function safeParse(data: string, logger: PluginLogger): SsePayload | null {
  try {
    const parsed: unknown = JSON.parse(data);
    if (parsed && typeof parsed === 'object') {
      return parsed as SsePayload;
    }
    return null;
  } catch (err) {
    const preview = data.slice(0, 100);
    logger.warn(`sse: failed to parse event data preview=${preview} err=${errorMessage(err)}`);
    return null;
  }
}

async function dynamicImportEventSource(): Promise<EventSourceCtor> {
  const mod = await import('eventsource');
  const ctor =
    (mod as { EventSource?: EventSourceCtor }).EventSource ??
    (mod as { default?: EventSourceCtor }).default;
  if (!ctor) {
    throw new Error('eventsource module did not export EventSource class');
  }
  return ctor;
}

function defaultEventSourceFactory(url: string, logger: PluginLogger): MinimalEventSource {
  // Lazy import so tests can inject their own factory without pulling the ESM module.
  // This path is only taken in production entry.ts wiring.
  let es: InstanceType<EventSourceCtor> | null = null;
  const listeners: Array<{ name: string; cb: (e: { data: string }) => void }> = [];
  let closed = false;
  dynamicImportEventSource()
    .then((EventSourceClass) => {
      if (closed) return;
      es = new EventSourceClass(url);
      for (const { name, cb } of listeners) {
        es.addEventListener(name, cb);
      }
    })
    .catch((err: unknown) => {
      logger.warn(
        `sse: dynamic import of \`eventsource\` failed; real-time persona sync is disabled (reconciler still active) err=${errorMessage(err)}`,
      );
    });
  return {
    addEventListener(name, cb) {
      if (es) es.addEventListener(name, cb);
      else listeners.push({ name, cb });
    },
    close() {
      closed = true;
      if (es) es.close();
    },
  };
}
