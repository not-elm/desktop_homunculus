import { useState } from "react";
import type { AgentSettings } from "../hooks/useAgentSettings";
import { GeneralSettingsTab } from "./GeneralSettingsTab";
import { ClaudeSettingsTab } from "./ClaudeSettingsTab";

type Tab = "general" | "claude";

const TABS: { id: Tab; label: string }[] = [
  { id: "general", label: "General" },
  { id: "claude", label: "Claude" },
];

interface SettingsViewProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  saving: boolean;
  onSave: () => void;
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  savingApiKey: boolean;
  onApiKeySave: () => void;
}

export function SettingsView({
  settings, onSettingsChange, saving, onSave,
  apiKey, onApiKeyChange, savingApiKey, onApiKeySave,
}: SettingsViewProps) {
  const [tab, setTab] = useState<Tab>("general");

  return (
    <div className="hud-settings-view">
      <SettingsTabs current={tab} onChange={setTab} />
      <div className="hud-settings-content">
        {tab === "general" && (
          <GeneralSettingsTab settings={settings} onSettingsChange={onSettingsChange} />
        )}
        {tab === "claude" && (
          <ClaudeSettingsTab
            settings={settings}
            onSettingsChange={onSettingsChange}
            apiKey={apiKey}
            onApiKeyChange={onApiKeyChange}
            onApiKeySave={onApiKeySave}
            savingApiKey={savingApiKey}
          />
        )}
      </div>
      <div className="hud-settings-footer">
        <button
          className={`hud-settings-save${saving ? " hud-settings-save--done" : ""}`}
          type="button"
          onClick={onSave}
          disabled={saving}
        >
          {saving ? "Saved" : "Save"}
        </button>
      </div>
    </div>
  );
}

function SettingsTabs({ current, onChange }: { current: Tab; onChange: (t: Tab) => void }) {
  return (
    <div className="hud-settings-tabs">
      {TABS.map((t) => (
        <button
          key={t.id}
          className={`hud-settings-tab${current === t.id ? " hud-settings-tab--active" : ""}`}
          type="button"
          onClick={() => onChange(t.id)}
        >
          {t.label}
        </button>
      ))}
    </div>
  );
}
