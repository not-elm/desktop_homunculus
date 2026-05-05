import { Slider } from '@hmcs/ui';

interface GeneralTabProps {
  fps: number;
  setFps: (v: number) => void;
  alpha: number;
  setAlpha: (v: number) => void;
}

const labelClasses = 'flex flex-col gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70';

const descriptionClasses =
  'text-[0.7rem] tracking-[0.04em] normal-case leading-[1.4] text-hud-text-subdued';

const sliderBoxClasses =
  'flex flex-row items-center gap-3 rounded-md border border-primary/20 bg-input px-3 py-2.5';

const sliderValueClasses = 'min-w-[3.5em] text-right font-mono text-xs text-foreground';

export function GeneralTab({ fps, setFps, alpha, setAlpha }: GeneralTabProps) {
  return (
    <div className="flex flex-col gap-4">
      <div className={labelClasses}>
        Frame Rate
        <div className={sliderBoxClasses}>
          <Slider
            className="flex-1"
            min={1}
            max={120}
            step={1}
            value={[fps]}
            onValueChange={(arr) => setFps(arr[0])}
          />
          <span className={sliderValueClasses}>{Math.round(fps)} fps</span>
        </div>
        <span className={descriptionClasses}>
          Controls the rendering frame rate. Lower values reduce CPU/GPU usage.
        </span>
      </div>

      <div className={labelClasses}>
        Shadow Opacity
        <div className={sliderBoxClasses}>
          <Slider
            className="flex-1"
            min={0}
            max={1}
            step={0.01}
            value={[alpha]}
            onValueChange={(arr) => setAlpha(arr[0])}
          />
          <span className={sliderValueClasses}>{Math.round(alpha * 100)}%</span>
        </div>
        <span className={descriptionClasses}>
          Controls the transparency of the shadow panel overlay behind the character.
        </span>
      </div>
    </div>
  );
}
