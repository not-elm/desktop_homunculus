import { useState } from "react";

interface ClaudeSettingsTabProps {
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  onApiKeySave: () => void;
  savingApiKey: boolean;
}

export function ClaudeSettingsTab({
  apiKey,
  onApiKeyChange,
  onApiKeySave,
  savingApiKey,
}: ClaudeSettingsTabProps) {
  const [showKey, setShowKey] = useState(false);

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
    </div>
  );
}
