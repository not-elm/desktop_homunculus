import { assets } from '@hmcs/sdk';
import { AssetSelect, type AssetSelectGroup } from '@hmcs/ui';
import {
  type BehaviorAnimations,
  DEFAULT_PROCESS,
  isDefaultProcess,
} from '@persona/shared/behavior-config';
import { useBehaviorCommands } from '@persona/shared/hooks/useBehaviorCommands';
import { useEffect, useState } from 'react';

export interface BehaviorSectionProps {
  process: string | null;
  animations: BehaviorAnimations;
  onProcessChange: (process: string | null) => void;
  onAnimationsChange: (animations: BehaviorAnimations) => void;
}

export function BehaviorSection({
  process,
  animations,
  onProcessChange,
  onAnimationsChange,
}: BehaviorSectionProps) {
  const commands = useBehaviorCommands();
  const vrmaAssets = useVrmaAssets();
  const showAnimations = isDefaultProcess(process);

  const processItems: AssetSelectGroup[] = [
    { label: 'DEFAULT', items: [{ id: DEFAULT_PROCESS, description: 'Built-in behavior' }] },
    ...(commands.length > 0 ? [{ label: 'MOD', items: commands }] : []),
  ];

  const displayValue = isDefaultProcess(process) ? DEFAULT_PROCESS : process;

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2 text-xs font-semibold uppercase tracking-[0.15em] text-primary">
        <span className="inline-block h-3 w-0.5 rounded-sm bg-primary" />
        Behavior
      </div>

      <div className="flex flex-col gap-1.5">
        <div className="text-xs uppercase tracking-[0.1em] text-primary/70">Process</div>
        <AssetSelect
          value={displayValue}
          onValueChange={(v) => onProcessChange(v === DEFAULT_PROCESS ? null : v)}
          items={processItems}
        />
      </div>

      {showAnimations ? (
        <AnimationFields
          animations={animations}
          vrmaAssets={vrmaAssets}
          onChange={onAnimationsChange}
        />
      ) : (
        <div className="mt-2 border-l border-primary/12 px-3 py-2 text-[0.7rem] tracking-[0.05em] text-muted-foreground/50">
          Animations are controlled by the selected process.
        </div>
      )}
    </div>
  );
}

function AnimationFields({
  animations,
  vrmaAssets,
  onChange,
}: {
  animations: BehaviorAnimations;
  vrmaAssets: AssetSelectGroup[];
  onChange: (animations: BehaviorAnimations) => void;
}) {
  const fields: Array<{ key: keyof BehaviorAnimations; label: string; dotClass: string }> = [
    { key: 'idle', label: 'Idle Animation', dotClass: 'bg-primary' },
    { key: 'drag', label: 'Drag Animation', dotClass: 'bg-holo-rose' },
    { key: 'sitting', label: 'Sitting Animation', dotClass: 'bg-holo-violet' },
  ];

  return (
    <div className="mt-2 flex flex-col gap-2 border-l border-primary/12 pl-4">
      {fields.map(({ key, label, dotClass }) => (
        <div key={key} className="flex flex-col gap-1.5">
          <div className="flex items-center gap-2 text-xs uppercase tracking-[0.1em] text-primary/70">
            <span className={`inline-block h-1.5 w-1.5 rounded-full align-middle ${dotClass}`} />
            {label}
          </div>
          <AssetSelect
            value={animations[key]}
            onValueChange={(v) => onChange({ ...animations, [key]: v ?? animations[key] })}
            items={vrmaAssets}
          />
        </div>
      ))}
    </div>
  );
}

function useVrmaAssets(): AssetSelectGroup[] {
  const [groups, setGroups] = useState<AssetSelectGroup[]>([]);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const list = await assets.list({ type: 'vrma' });
        const modItems = list.filter((a) => !a.id.startsWith('vrma:local:'));
        const localItems = list.filter((a) => a.id.startsWith('vrma:local:'));
        const result: AssetSelectGroup[] = [
          ...(modItems.length > 0 ? [{ label: 'MOD', items: modItems }] : []),
          ...(localItems.length > 0 ? [{ label: 'LOCAL', items: localItems }] : []),
        ];
        if (!cancelled) setGroups(result);
      } catch (e) {
        console.error('Failed to load VRMA assets:', e);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  return groups;
}
