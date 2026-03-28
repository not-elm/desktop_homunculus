import { useState } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import type { AgentSettings } from "../hooks/useAgentSettings";

interface ClaudeTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  onApiKeySave: () => void;
  savingApiKey: boolean;
}

export function ClaudeTab({
  settings,
  onSettingsChange,
  apiKey,
  onApiKeyChange,
  onApiKeySave,
  savingApiKey,
}: ClaudeTabProps) {
  const [showKey, setShowKey] = useState(false);
  const needsApiKey = settings.executor === "sdk";

  function update<K extends keyof AgentSettings>(
    key: K,
    value: AgentSettings[K],
  ) {
    onSettingsChange({ ...settings, [key]: value });
  }

  return (
    <div className="settings-section">
      <label className="settings-label">
        Executor
        <span className="settings-label-desc">
          Agent backend to use for execution
        </span>
        <Select
          value={settings.executor}
          onValueChange={(v) => update("executor", v as AgentSettings["executor"])}
        >
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="sdk">Claude SDK</SelectItem>
            <SelectItem value="cli">Claude CLI</SelectItem>
            <SelectItem value="codex">Codex</SelectItem>
            <SelectItem value="codex-appserver">Codex AppServer</SelectItem>
          </SelectContent>
        </Select>
      </label>

      <div className="agent-divider" />

      {needsApiKey && (
        <>
          <div className="settings-label">
            Anthropic API Key
            <span className="settings-label-desc">
              Used to authenticate with Claude. Stored separately per-machine.
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
        </>
      )}

      <label className="settings-label">
        Model
        <span className="settings-label-desc">
          Model for the agent to use
        </span>
        <Select
          value={settings.claudeModel || "default"}
          onValueChange={(v) => update("claudeModel", v === "default" ? "" : v)}
        >
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="default">Default</SelectItem>
            <SelectItem value="claude-sonnet-4-6">Claude Sonnet 4.6</SelectItem>
            <SelectItem value="claude-opus-4-6">Claude Opus 4.6</SelectItem>
            <SelectItem value="claude-haiku-4-5">Claude Haiku 4.5</SelectItem>
          </SelectContent>
        </Select>
      </label>
    </div>
  );
}
