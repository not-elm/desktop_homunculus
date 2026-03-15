import { GeneralTab } from "./components/GeneralTab";
import { SpeechTab } from "./components/SpeechTab";
import { useSettings, type SettingsTab } from "./hooks/useSettings";

export function App() {
  const {
    loading,
    tab,
    setTab,
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

  const tabs: { id: SettingsTab; label: string }[] = [
    { id: "general", label: "General" },
    { id: "speech", label: "Speech" },
  ];

  return (
    <div className="settings-panel holo-refract-border holo-noise">
      <div className="settings-highlight" />
      <div className="settings-bottom-line" />
      <div className="settings-scanline" />
      <span className="settings-corner settings-corner--tl" />
      <span className="settings-corner settings-corner--tr" />
      <span className="settings-corner settings-corner--bl" />
      <span className="settings-corner settings-corner--br" />

      <div className="settings-header">
        <h1 className="settings-title">Settings</h1>
      </div>

      <div className="settings-tabs">
        {tabs.map((t) => (
          <button
            key={t.id}
            className={`settings-tab ${tab === t.id ? "settings-tab--active" : ""}`}
            onClick={() => setTab(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>

      <div className="settings-content">
        {tab === "general" && (
          <GeneralTab
            fps={fps}
            setFps={setFps}
            alpha={alpha}
            setAlpha={setAlpha}
          />
        )}
        {tab === "speech" && <SpeechTab />}
      </div>

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
