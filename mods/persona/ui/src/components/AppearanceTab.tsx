import { Slider } from '@hmcs/ui';
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
    <div className="flex flex-col gap-6">
      <ScaleSection scale={scale} onScaleChange={onScaleChange} />
      <BehaviorSection
        process={behaviorProcess}
        animations={behaviorAnimations}
        onProcessChange={onBehaviorProcessChange}
        onAnimationsChange={onBehaviorAnimationsChange}
      />
    </div>
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
    <div className="flex flex-col gap-2">
      <div className="flex items-center justify-between text-xs uppercase tracking-[0.1em] text-primary/70">
        <span>Scale</span>
        <span className="font-mono text-foreground">{scale.toFixed(2)}</span>
      </div>
      <Slider
        min={0.1}
        max={3}
        step={0.05}
        value={[scale]}
        onValueChange={([v]) => onScaleChange(v)}
      />
    </div>
  );
}
