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
      <div className="my-4 h-px bg-gradient-to-r from-transparent via-[oklch(0.35_0.02_250/0.25)] to-transparent" />
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
        <div className="flex flex-row items-center gap-3">
          <input
            type="range"
            className="settings-slider"
            min={0.1}
            max={3}
            step={0.05}
            value={scale}
            onChange={(e) => onScaleChange(parseFloat(e.target.value))}
          />
          <span className="min-w-[3.5em] text-right font-mono text-xs text-[oklch(0.72_0.14_192)]">
            {scale.toFixed(2)}
          </span>
        </div>
      </label>
    </div>
  );
}
