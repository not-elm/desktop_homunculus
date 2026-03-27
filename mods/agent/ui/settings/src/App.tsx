import { useState } from "react";
import { useAgentSettings } from "./hooks/useAgentSettings";
import { GeneralTab } from "./components/GeneralTab";
import { ClaudeTab } from "./components/ClaudeTab";
import { CodexTab } from "./components/CodexTab";

type Tab = "general" | "claude" | "codex";

const TABS: { id: Tab; label: string }[] = [
  { id: "general", label: "General" },
  { id: "claude", label: "Claude" },
  { id: "codex", label: "Codex" },
];

export function App() {
  const [tab, setTab] = useState<Tab>("general");
  const {
    loading,
    settings,
    setSettings,
    saving,
    saveSettings,
    apiKey,
    setApiKey,
    savingApiKey,
    saveApiKey,
    handleClose,
  } = useAgentSettings();

  if (loading) {
    return (
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  return (
    <div className="settings-panel holo-noise">
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
        <h1 className="settings-title">Agent Settings</h1>
        <button className="settings-close" type="button" onClick={handleClose}>
          Close
        </button>
      </div>

      {/* Tabs */}
      <div className="settings-tabs">
        {TABS.map((t) => (
          <button
            key={t.id}
            className={`settings-tab${tab === t.id ? " settings-tab--active" : ""}`}
            type="button"
            onClick={() => setTab(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="settings-content">
        {tab === "general" && (
          <GeneralTab settings={settings} onSettingsChange={setSettings} />
        )}
        {tab === "claude" && (
          <ClaudeTab
            settings={settings}
            onSettingsChange={setSettings}
            apiKey={apiKey}
            onApiKeyChange={setApiKey}
            onApiKeySave={saveApiKey}
            savingApiKey={savingApiKey}
          />
        )}
        {tab === "codex" && (
          <CodexTab settings={settings} onSettingsChange={setSettings} />
        )}
      </div>

      {/* Footer */}
      <div className="settings-footer">
        <button
          className={`settings-save${saving ? " settings-save--success" : ""}`}
          type="button"
          onClick={saveSettings}
          disabled={saving}
        >
          {saving ? "Saved" : "Save"}
        </button>
      </div>
    </div>
  );
}
