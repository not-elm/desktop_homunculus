import { mods } from '@hmcs/sdk';
import { DEFAULT_PROCESS } from '@persona/shared/behavior-config';
import { useEffect, useState } from 'react';

export interface BehaviorCommandItem {
  id: string;
  description?: string;
}

export function useBehaviorCommands(): BehaviorCommandItem[] {
  const [commands, setCommands] = useState<BehaviorCommandItem[]>([]);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const modList = await mods.list();
        const items: BehaviorCommandItem[] = [];
        for (const mod of modList) {
          for (const cmd of mod.commands) {
            const fullCmd = cmd.includes(':') ? cmd : `${mod.name}:${cmd}`;
            if (fullCmd === DEFAULT_PROCESS) continue;
            items.push({ id: fullCmd, description: mod.name });
          }
        }
        if (!cancelled) setCommands(items);
      } catch (e) {
        console.error('Failed to load behavior commands:', e);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  return commands;
}
