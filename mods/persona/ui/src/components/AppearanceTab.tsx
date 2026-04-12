import { assets } from '@hmcs/sdk';
import { AssetSelect, type AssetSelectGroup } from '@hmcs/ui';
import { DEFAULT_PROCESS, isDefaultProcess, type BehaviorAnimations } from '@persona/shared/behavior-config';
import { useBehaviorCommands, type BehaviorCommandItem } from '@persona/shared/hooks/useBehaviorCommands';
import { useEffect, useState } from 'react';

interface AppearanceTabProps {
  scale: number;
  onScaleChange: (scale: number) => void;
  behaviorProcess: string | null;
  behaviorAnimations: BehaviorAnimations;
  onBehaviorProcessChange: (process: string | null) => void;
  onBehaviorAnimationsChange: (animations: BehaviorAnimations) => void;
}

export function AppearanceTab({
  scale,
  onScaleChange,
  behaviorProcess,
  behaviorAnimations,
  onBehaviorProcessChange,
  onBehaviorAnimationsChange,
}: AppearanceTabProps) {
  const commands = useBehaviorCommands();
  const vrmaAssets = useVrmaAssets();
  const showAnimations = isDefaultProcess(behaviorProcess);

  return (
    <>
      <ScaleSection scale={scale} onScaleChange={onScaleChange} />
      <div className="settings-separator" />
      <BehaviorSection
        process={behaviorProcess}
        animations={behaviorAnimations}
        commands={commands}
        vrmaAssets={vrmaAssets}
        showAnimations={showAnimations}
        onProcessChange={onBehaviorProcessChange}
        onAnimationsChange={onBehaviorAnimationsChange}
      />
    </>
  );
}

function ScaleSection({
  scale,
  onScaleChange,
}: {
  scale: number;
  onScaleChange: (scale: number) => void;
}) {
  return (
    <div className="settings-section">
      <label className="settings-label">
        Scale
        <div className="settings-slider-row">
          <input
            type="range"
            className="settings-slider"
            min={0.1}
            max={3}
            step={0.05}
            value={scale}
            onChange={(e) => onScaleChange(parseFloat(e.target.value))}
          />
          <span className="settings-slider-value">{scale.toFixed(2)}</span>
        </div>
      </label>
    </div>
  );
}

function BehaviorSection({
  process,
  animations,
  commands,
  vrmaAssets,
  showAnimations,
  onProcessChange,
  onAnimationsChange,
}: {
  process: string | null;
  animations: BehaviorAnimations;
  commands: BehaviorCommandItem[];
  vrmaAssets: AssetSelectGroup[];
  showAnimations: boolean;
  onProcessChange: (process: string | null) => void;
  onAnimationsChange: (animations: BehaviorAnimations) => void;
}) {
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
