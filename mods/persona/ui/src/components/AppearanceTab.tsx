import type { BehaviorAnimations } from '@persona/shared/behavior-config';
import { BehaviorSection } from '@persona/shared/components/BehaviorSection';

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
  return (
    <>
      <ScaleSection scale={scale} onScaleChange={onScaleChange} />
      <div className="settings-separator" />
      <BehaviorSection
        process={behaviorProcess}
        animations={behaviorAnimations}
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
