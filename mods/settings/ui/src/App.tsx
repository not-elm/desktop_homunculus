import { useSettings } from "./hooks/useSettings";

export function App() {
  const {
    loading,
    fps,
    setFps,
    alpha,
    setAlpha,
    saving,
    saved,
    handleSave,
    handleClose,
  } = useSettings();

  if (loading) {
    return (
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  return (
    <div className="settings-panel holo-refract-border holo-noise">
      {/* Decorative layers */}
      <div className="settings-highlight" />
      <div className="settings-bottom-line" />
      <div className="settings-scanline" />
      <span className="settings-corner settings-corner--tl" />
      <span className="settings-corner settings-corner--tr" />
      <span className="settings-corner settings-corner--bl" />
      <span className="settings-corner settings-corner--br" />

      {/* Header */}
      <div className="settings-header">
        <h1 className="settings-title">Settings</h1>
      </div>

      {/* Content */}
      <div className="settings-content">
        <div className="settings-section">
          {/* Frame Rate */}
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

          {/* Shadow Opacity */}
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
      </div>

      {/* Footer */}
      <div className="settings-footer">
        <button className="settings-close" onClick={handleClose}>
          Close
        </button>
        <button
          className={`settings-save ${saved ? "settings-save--success" : ""}`}
          onClick={handleSave}
          disabled={saving}
        >
          {saving ? "Saving..." : saved ? "Saved!" : "Save"}
        </button>
      </div>
    </div>
  );
}
