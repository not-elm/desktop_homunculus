import { useCharacterSettings, type Tab } from "./hooks/useCharacterSettings";
import { BasicTab } from "./components/BasicTab";
import { PersonaTab } from "./components/PersonaTab";
import { OceanTab } from "./components/OceanTab";

export function App() {
  const {
    loading,
    name,
    tab,
    setTab,
    scale,
    setScale,
    profile,
    setProfile,
    personality,
    setPersonality,
    ocean,
    setOcean,
    saving,
    saved,
    handleSave,
    handleClose,
  } = useCharacterSettings();

  if (loading) {
    return (
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  const tabs: { id: Tab; label: string }[] = [
    { id: "basic", label: "Basic" },
    { id: "persona", label: "Persona" },
    { id: "ocean", label: "OCEAN" },
  ];

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
        <span className="settings-entity-name">{name}</span>
      </div>

      {/* Tabs */}
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

      {/* Content */}
      <div className="settings-content">
        {tab === "basic" && (
          <BasicTab name={name} scale={scale} onScaleChange={setScale} />
        )}
        {tab === "persona" && (
          <PersonaTab
            profile={profile}
            personality={personality}
            onProfileChange={setProfile}
            onPersonalityChange={setPersonality}
          />
        )}
        {tab === "ocean" && <OceanTab ocean={ocean} onChange={setOcean} />}
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
