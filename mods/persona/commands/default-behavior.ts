import { Persona, repeat, sleep } from '@hmcs/sdk';
import {
  resolveBehaviorConfig,
  type BehaviorAnimations,
} from '../shared/behavior-config.ts';

const personaId = process.argv[2];
if (!personaId) {
  console.error('Usage: default-behavior <personaId>');
  process.exit(1);
}

const persona = new Persona(personaId);

let snapshot;
try {
  snapshot = await persona.snapshot();
} catch {
  console.error(`Persona not found: ${personaId}`);
  process.exit(1);
}

const config = resolveBehaviorConfig(snapshot);
const animations: BehaviorAnimations = config.animations;
let currentState = snapshot.state;

console.log(`[default-behavior] Applying initial behavior: state=${currentState}`);
await applyBehaviorWithLogging(persona, currentState, animations, 'startup');

const events = persona.events();
console.log(`[default-behavior] Subscribing to events...`);

events.on('state-change', async (e) => {
  console.log(`[default-behavior] EVENT state-change: ${e.state}`);
  currentState = e.state;
  await applyBehaviorWithLogging(persona, currentState, animations, 'state-change');
});
events.on('vrm-attached', async (e) => {
  console.log(`[default-behavior] EVENT vrm-attached:`, JSON.stringify(e));
  await applyBehaviorAfterAttach(persona, () => currentState, animations);
});
events.on('vrm-detached', async (e) => {
  console.log(`[default-behavior] EVENT vrm-detached:`, JSON.stringify(e));
});
events.on('persona-change', async (e) => {
  console.log(`[default-behavior] EVENT persona-change received`);
});

console.log(`[default-behavior] Ready and listening for events`);

process.on('SIGTERM', () => {
  events.close();
  process.exit(0);
});

async function applyBehavior(
  p: Persona,
  state: string,
  anims: BehaviorAnimations,
): Promise<void> {
  const vrm = p.vrm();
  const option = { repeat: repeat.forever(), transitionSecs: 0.5 } as const;

  if (state === 'idle') {
    await vrm.playVrma({ asset: anims.idle, ...option });
    await sleep(500);
    await vrm.lookAtCursor();
  } else if (state === 'drag') {
    await vrm.unlook();
    await vrm.playVrma({ asset: anims.drag, ...option, resetSpringBones: true });
  } else if (state === 'sitting') {
    await vrm.playVrma({ asset: anims.sitting, ...option });
    await sleep(500);
    await vrm.lookAtCursor();
  }
}

async function applyBehaviorWithLogging(
  p: Persona,
  state: string,
  anims: BehaviorAnimations,
  source: string,
): Promise<void> {
  try {
    await applyBehavior(p, state, anims);
  } catch (err) {
    console.error(`[default-behavior] applyBehavior failed (${source}):`, err);
  }
}

async function applyBehaviorAfterAttach(
  p: Persona,
  getState: () => string,
  anims: BehaviorAnimations,
): Promise<void> {
  try {
    const state = getState();
    console.log(`[default-behavior] vrm-attach: applying state=${state}`);
    await applyBehavior(p, state, anims);
    console.log(`[default-behavior] vrm-attach: SUCCESS`);
  } catch (err) {
    console.error('[default-behavior] failed to apply after vrm-attached:', err);
  }
}
