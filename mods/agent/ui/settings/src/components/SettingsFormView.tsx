import type { AgentSettings } from "../hooks/useSettingsDraft";
import type { SettingsCategory } from "../types";

// Import shared components from session entry — viteSingleFile will inline them
import { PhraseListField } from "../../../session/src/components/PhraseListField";

interface SettingsFormViewProps {
  category: SettingsCategory;
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  onApiKeySave: () => void;
  savingApiKey: boolean;
}

export function SettingsFormView({
  category, settings, onSettingsChange,
  apiKey, onApiKeyChange, onApiKeySave, savingApiKey,
}: SettingsFormViewProps) {
  return (
    <div className="stg-form">
      <div className="stg-form-header">
        <span className="stg-scope-marker">Global Settings</span>
      </div>
      {category === "phrases" && (
        <PhrasesForm settings={settings} onSettingsChange={onSettingsChange} />
      )}
      {category === "permissions" && (
        <PermissionsForm settings={settings} onSettingsChange={onSettingsChange} />
      )}
      {category === "api-model" && (
        <ApiModelForm
          settings={settings}
          onSettingsChange={onSettingsChange}
          apiKey={apiKey}
          onApiKeyChange={onApiKeyChange}
          onApiKeySave={onApiKeySave}
          savingApiKey={savingApiKey}
        />
      )}
    </div>
  );
}

function PhrasesForm({ settings, onSettingsChange }: { settings: AgentSettings; onSettingsChange: (s: AgentSettings) => void }) {
  function addPhrase(key: "approvalPhrases" | "denyPhrases", item: string) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }
  function removePhrase(key: "approvalPhrases" | "denyPhrases", index: number) {
    onSettingsChange({ ...settings, [key]: settings[key].filter((_: string, i: number) => i !== index) });
  }
  return (
    <>
      <PhraseListField label="Approval Phrases" description="Phrases that confirm agent tool use requests"
        phrases={settings.approvalPhrases} onAdd={(p) => addPhrase("approvalPhrases", p)} onRemove={(i) => removePhrase("approvalPhrases", i)} badgeVariant="violet" />
      <div className="stg-section-divider" />
      <PhraseListField label="Deny Phrases" description="Phrases that reject agent tool use requests"
        phrases={settings.denyPhrases} onAdd={(p) => addPhrase("denyPhrases", p)} onRemove={(i) => removePhrase("denyPhrases", i)} badgeVariant="rose" />
    </>
  );
}

function PermissionsForm({ settings, onSettingsChange }: { settings: AgentSettings; onSettingsChange: (s: AgentSettings) => void }) {
  function addToList(key: "allowList" | "disallowedTools", item: string) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }
  function removeFromList(key: "allowList" | "disallowedTools", index: number) {
    onSettingsChange({ ...settings, [key]: settings[key].filter((_: string, i: number) => i !== index) });
  }
  return (
    <>
      <PhraseListField label="Default Allow List" description="Tools always permitted without asking"
        phrases={settings.allowList} onAdd={(p) => addToList("allowList", p)} onRemove={(i) => removeFromList("allowList", i)} badgeVariant="green" />
      <div className="stg-section-divider" />
      <PhraseListField label="Disallowed Tools" description="Tools the agent is never permitted to use"
        phrases={settings.disallowedTools} onAdd={(p) => addToList("disallowedTools", p)} onRemove={(i) => removeFromList("disallowedTools", i)} badgeVariant="rose" />
    </>
  );
}

function ApiModelForm({ settings, onSettingsChange, apiKey, onApiKeyChange, onApiKeySave, savingApiKey }: {
  settings: AgentSettings; onSettingsChange: (s: AgentSettings) => void;
  apiKey: string; onApiKeyChange: (k: string) => void; onApiKeySave: () => void; savingApiKey: boolean;
}) {
  return (
    <>
      <div className="stg-field-group">
        <label className="stg-field-label">Anthropic API Key</label>
        <div className="stg-api-key-row">
          <input className="stg-input" type="password" value={apiKey} onChange={(e) => onApiKeyChange(e.target.value)}
            placeholder="sk-ant-..." autoComplete="off" spellCheck={false} />
          <button className="stg-action-btn" type="button" onClick={onApiKeySave} disabled={savingApiKey}>
            {savingApiKey ? "Saving..." : "Save Key"}
          </button>
        </div>
      </div>
      <div className="stg-section-divider" />
      <div className="stg-field-group">
        <label className="stg-field-label">Model</label>
        <select className="stg-select" value={settings.claudeModel || "default"}
          onChange={(e) => onSettingsChange({ ...settings, claudeModel: e.target.value === "default" ? "" : e.target.value })}>
          <option value="default">Default</option>
          <option value="claude-sonnet-4-6">Claude Sonnet 4.6</option>
          <option value="claude-opus-4-6">Claude Opus 4.6</option>
          <option value="claude-haiku-4-5-20251001">Claude Haiku 4.5</option>
        </select>
      </div>
    </>
  );
}
