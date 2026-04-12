import { HomunculusApiError, Persona, repeat, sleep } from '@hmcs/sdk';
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

await applyBehaviorWithLogging(persona, currentState, animations, 'startup');

const events = persona.events();
events.on('state-change', async (e) => {
  currentState = e.state;
  await applyBehaviorWithLogging(persona, currentState, animations, 'state-change');
});
events.on('vrm-attached', async () => {
  await applyBehaviorAfterAttach(persona, () => currentState, animations);
});

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
    if (isExpectedVrmTimingError(err)) return;
    console.error(`[default-behavior] applyBehavior failed (${source}):`, err);
  }
}

async function applyBehaviorAfterAttach(
  p: Persona,
  getState: () => string,
  anims: BehaviorAnimations,
): Promise<void> {
  const delaysMs = [0, 100, 250, 500, 1000];

  for (let i = 0; i < delaysMs.length; i++) {
    if (delaysMs[i] > 0) await sleep(delaysMs[i]);

    try {
      await applyBehavior(p, getState(), anims);
      return;
    } catch (err) {
      if (!isExpectedVrmTimingError(err) || i === delaysMs.length - 1) {
        console.error('[default-behavior] failed to reapply after vrm-attached:', err);
        return;
      }
    }
  }
}

function isExpectedVrmTimingError(err: unknown): boolean {
  return err instanceof HomunculusApiError && err.statusCode === 404;
}
