import { Persona, repeat, sleep } from '@hmcs/sdk';
import {
  DEFAULT_ANIMATIONS,
  resolveBehaviorConfig,
  type BehaviorAnimations,
} from '@persona/shared/behavior-config';

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

await applyBehavior(persona, snapshot.state, animations);

const events = persona.events();
events.on('state-change', async (e) => {
  await applyBehavior(persona, e.state, animations);
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

  try {
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
  } catch {
    // VRM not attached or play failed — skip
  }
}
