import { useState } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import type { AgentSettings } from "../hooks/useAgentSettings";
import { PhraseListField } from "./PhraseListField";

interface ClaudeCodeTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  onApiKeySave: () => void;
  savingApiKey: boolean;
}

export function ClaudeCodeTab({
  settings,
  onSettingsChange,
  apiKey,
  onApiKeyChange,
  onApiKeySave,
  savingApiKey,
}: ClaudeCodeTabProps) {
  const [showKey, setShowKey] = useState(false);

  function update<K extends keyof AgentSettings>(
    key: K,
    value: AgentSettings[K],
  ) {
    onSettingsChange({ ...settings, [key]: value });
  }

  function addToList(
    key: keyof Pick<AgentSettings, "approvalPhrases" | "denyPhrases" | "allowList" | "disallowedTools">,
    item: string,
  ) {
    update(key, [...settings[key], item]);
  }

  function removeFromList(
    key: keyof Pick<AgentSettings, "approvalPhrases" | "denyPhrases" | "allowList" | "disallowedTools">,
    index: number,
  ) {
    update(key, settings[key].filter((_, i) => i !== index));
  }

  return (
    <div className="settings-section">
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

      <label className="settings-label">
        Model
        <span className="settings-label-desc">
          Claude model for the agent to use
        </span>
        <Select
          value={settings.model || "default"}
          onValueChange={(v) => update("model", v === "default" ? "" : v)}
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

      <div className="agent-divider" />

      <PhraseListField
        label="Approval Phrases"
        description="Phrases that confirm agent tool use requests"
        phrases={settings.approvalPhrases}
        onAdd={(p) => addToList("approvalPhrases", p)}
        onRemove={(i) => removeFromList("approvalPhrases", i)}
        badgeVariant="violet"
      />

      <PhraseListField
        label="Deny Phrases"
        description="Phrases that reject agent tool use requests"
        phrases={settings.denyPhrases}
        onAdd={(p) => addToList("denyPhrases", p)}
        onRemove={(i) => removeFromList("denyPhrases", i)}
        badgeVariant="rose"
      />

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
