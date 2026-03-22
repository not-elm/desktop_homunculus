interface GeneralTabProps {
  fps: number;
  setFps: (v: number) => void;
  alpha: number;
  setAlpha: (v: number) => void;
}

export function GeneralTab({ fps, setFps, alpha, setAlpha }: GeneralTabProps) {
  return (
    <div className="settings-section">
      <label className="settings-label">
        Frame Rate
        <div className="settings-slider-row">
          <input
            type="range"
            className="settings-slider"
            min={1}
            max={120}
            step={1}
            value={fps}
            onChange={(e) => setFps(Number(e.target.value))}
          />
          <span className="settings-slider-value">{Math.round(fps)} fps</span>
        </div>
        <span className="settings-description">
          Controls the rendering frame rate. Lower values reduce CPU/GPU usage.
        </span>
      </label>

      <label className="settings-label">
        Shadow Opacity
        <div className="settings-slider-row">
          <input
            type="range"
            className="settings-slider"
            min={0}
            max={1}
            step={0.01}
            value={alpha}
            onChange={(e) => setAlpha(Number(e.target.value))}
          />
          <span className="settings-slider-value">
            {Math.round(alpha * 100)}%
          </span>
        </div>
        <span className="settings-description">
          Controls the transparency of the shadow panel overlay behind the character.
        </span>
      </label>
    </div>
  );
}
