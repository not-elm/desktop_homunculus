interface BasicTabProps {
  name: string;
  displayName: string;
  onDisplayNameChange: (displayName: string) => void;
  scale: number;
  onScaleChange: (scale: number) => void;
}

export function BasicTab({
  name,
  displayName,
  onDisplayNameChange,
  scale,
  onScaleChange,
}: BasicTabProps) {
  return (
    <div className="settings-section">
      <label className="settings-label">
        Display Name
        <input
          type="text"
          className="settings-input"
          value={displayName}
          placeholder={name}
          onChange={(e) => onDisplayNameChange(e.target.value)}
        />
      </label>

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
