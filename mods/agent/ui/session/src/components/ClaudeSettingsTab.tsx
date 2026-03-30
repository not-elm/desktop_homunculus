import { useState } from "react";
import type { AgentSettings } from "../hooks/useAgentSettings";
import { PhraseListField } from "./PhraseListField";

interface ClaudeSettingsTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  onApiKeySave: () => void;
  savingApiKey: boolean;
}

export function ClaudeSettingsTab({
  settings,
  onSettingsChange,
  apiKey,
  onApiKeyChange,
  onApiKeySave,
  savingApiKey,
}: ClaudeSettingsTabProps) {
  const [showKey, setShowKey] = useState(false);

  function addToList(
    key: keyof Pick<AgentSettings, "allowList" | "disallowedTools">,
    item: string,
  ) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }

  function removeFromList(
    key: keyof Pick<AgentSettings, "allowList" | "disallowedTools">,
    index: number,
  ) {
    onSettingsChange({ ...settings, [key]: settings[key].filter((_: string, i: number) => i !== index) });
  }

  return (
    <div className="settings-section">
      <div className="settings-label">
        Anthropic API Key
        <span className="settings-label-desc">
          Used to authenticate with Claude
        </span>
        <div className="agent-pw-wrapper">
          <input
            className="agent-pw-input"
            type={showKey ? "text" : "password"}
            value={apiKey}
            onChange={(e) => onApiKeyChange(e.target.value)}
            placeholder="sk-ant-..."
            autoComplete="off"
            spellCheck={false}
          />
          <button
            className="agent-pw-toggle"
            type="button"
            onClick={() => setShowKey((v) => !v)}
            aria-label={showKey ? "Hide API key" : "Show API key"}
          >
            {showKey ? "Hide" : "Show"}
          </button>
        </div>
        <div style={{ display: "flex", justifyContent: "flex-end" }}>
          <button
            className="agent-add-btn"
            type="button"
            onClick={onApiKeySave}
            disabled={savingApiKey}
          >
            {savingApiKey ? "Saving..." : "Save Key"}
          </button>
        </div>
      </div>

      <div className="agent-divider" />

      <PhraseListField
        label="Default Allow List"
        description="Tools always permitted without asking"
        phrases={settings.allowList}
        onAdd={(p) => addToList("allowList", p)}
        onRemove={(i) => removeFromList("allowList", i)}
        badgeVariant="green"
      />

      <PhraseListField
        label="Disallowed Tools"
        description="Tools the agent is never permitted to use"
        phrases={settings.disallowedTools}
        onAdd={(p) => addToList("disallowedTools", p)}
        onRemove={(i) => removeFromList("disallowedTools", i)}
        badgeVariant="rose"
      />
    </div>
  );
}
