import type { PersonaSnapshot } from '@hmcs/sdk';

export interface BehaviorAnimations {
  idle: string;
  drag: string;
  sitting: string;
}

export interface BehaviorConfig {
  process: string | null;
  animations: BehaviorAnimations;
}

export const DEFAULT_PROCESS = '@hmcs/persona:default-behavior';

export const DEFAULT_ANIMATIONS: BehaviorAnimations = {
  idle: 'vrma:idle-maid',
  drag: 'vrma:grabbed',
  sitting: 'vrma:idle-sitting',
};

export function resolveBehaviorConfig(snapshot: PersonaSnapshot): BehaviorConfig {
  const raw = snapshot.metadata?.behavior as Partial<BehaviorConfig> | undefined;
  return {
    process: raw?.process ?? null,
    animations: {
      idle: raw?.animations?.idle ?? DEFAULT_ANIMATIONS.idle,
      drag: raw?.animations?.drag ?? DEFAULT_ANIMATIONS.drag,
      sitting: raw?.animations?.sitting ?? DEFAULT_ANIMATIONS.sitting,
    },
  };
}

export function resolveProcessCommand(config: BehaviorConfig): string {
  return config.process ?? DEFAULT_PROCESS;
}

export function isDefaultProcess(process: string | null): boolean {
  return process === null || process === DEFAULT_PROCESS;
}
