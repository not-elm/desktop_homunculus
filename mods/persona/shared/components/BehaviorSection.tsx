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
    <div className="settings-section">
      <div className="settings-section-heading">Behavior</div>

      <div className="detail-field">
        <div className="detail-field-label">Process</div>
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
        <div className="settings-behavior-hint">
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
    { key: 'idle', label: 'Idle Animation', dotClass: 'settings-state-dot--idle' },
    { key: 'drag', label: 'Drag Animation', dotClass: 'settings-state-dot--drag' },
    { key: 'sitting', label: 'Sitting Animation', dotClass: 'settings-state-dot--sitting' },
  ];

  return (
    <div className="settings-vrma-fields">
      {fields.map(({ key, label, dotClass }) => (
        <div key={key} className="detail-field">
          <div className="detail-field-label">
            <span className={`settings-state-dot ${dotClass}`} />
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
