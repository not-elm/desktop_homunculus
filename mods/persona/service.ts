import { host, Persona, type PersonaSnapshot, type ProcessHandle, processes } from '@hmcs/sdk';
import { EventSource } from 'eventsource';
import { resolveBehaviorConfig, resolveProcessCommand } from './shared/behavior-config.ts';

const handleMap = new Map<string, ProcessHandle>();

// --- Startup ---

const personas = await Persona.list();
const running = await processes.list();
const runningArgs = new Set(running.map((p) => p.args[0]));

for (const snapshot of personas) {
  const effective =
    snapshot.metadata?.['auto-spawn'] === true ? await spawnIfNeeded(snapshot) : snapshot;

  if (effective.spawned && !runningArgs.has(effective.id)) {
    await startBehaviorProcess(effective);
  }
}

// --- Runtime: SSE combined stream ---

const streamUrl = host.createUrl('personas/stream');
const source = new EventSource(streamUrl.toString());

source.addEventListener('persona-spawned', async (event) => {
  const data = JSON.parse(event.data) as { personaId: string };
  if (handleMap.has(data.personaId)) return;
  const snap = await new Persona(data.personaId).snapshot();
  await startBehaviorProcess(snap);
});

source.addEventListener('persona-despawned', async (event) => {
  const data = JSON.parse(event.data) as { personaId: string };
  await stopBehaviorProcess(data.personaId);
});

source.addEventListener('persona-deleted', async (event) => {
  const data = JSON.parse(event.data) as { personaId: string };
  await stopBehaviorProcess(data.personaId);
});

// --- Cleanup on SIGTERM ---

process.on('SIGTERM', async () => {
  source.close();
  const stops = [...handleMap.values()].map((h) => h.stop());
  await Promise.allSettled(stops);
  process.exit(0);
});

// --- Helpers ---

async function spawnIfNeeded(snapshot: PersonaSnapshot): Promise<PersonaSnapshot> {
  if (snapshot.spawned) return snapshot;
  const p = new Persona(snapshot.id);
  try {
    return await p.spawn();
  } catch {
    // Already spawned or failed — return fresh snapshot
    return await p.snapshot();
  }
}

async function startBehaviorProcess(snapshot: PersonaSnapshot): Promise<void> {
  if (handleMap.has(snapshot.id)) return;
  const config = resolveBehaviorConfig(snapshot);
  const command = resolveProcessCommand(config);
  try {
    const handle = await processes.start({
      command,
      args: [snapshot.id],
    });
    handleMap.set(snapshot.id, handle);
  } catch (e) {
    console.error(`Failed to start behavior for ${snapshot.id}:`, e);
  }
}

async function stopBehaviorProcess(personaId: string): Promise<void> {
  const handle = handleMap.get(personaId);
  if (!handle) return;
  handleMap.delete(personaId);
  try {
    await handle.stop();
  } catch {
    // Already stopped
  }
}
