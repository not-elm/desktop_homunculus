import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@hmcs/ui';
import { KeyCaptureField } from '../../components/KeyCaptureField';
import type { AgentSettings, PttKey } from '../hooks/useSettingsDraft';
import type { SettingsCategory } from '../types';
import { PhraseListField } from './PhraseListField';

interface SettingsFormViewProps {
  category: SettingsCategory;
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
}

export function SettingsFormView({ category, settings, onSettingsChange }: SettingsFormViewProps) {
  return (
    <div className="stg-form">
      <div className="stg-form-header">
        <span className="stg-scope-marker">Global Settings</span>
      </div>
      {category === 'phrases' && (
        <PhrasesForm settings={settings} onSettingsChange={onSettingsChange} />
      )}
      {category === 'permissions' && (
        <PermissionsForm settings={settings} onSettingsChange={onSettingsChange} />
      )}
      {category === 'backend' && (
        <BackendForm settings={settings} onSettingsChange={onSettingsChange} />
      )}
    </div>
  );
}

function PhrasesForm({
  settings,
  onSettingsChange,
}: {
  settings: AgentSettings;
  onSettingsChange: (s: AgentSettings) => void;
}) {
  function updatePttKey(key: PttKey | null) {
    onSettingsChange({ ...settings, pttKey: key });
  }

  function addPhrase(key: 'approvalPhrases' | 'denyPhrases', item: string) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }

  function removePhrase(key: 'approvalPhrases' | 'denyPhrases', index: number) {
    onSettingsChange({
      ...settings,
      [key]: settings[key].filter((_: string, i: number) => i !== index),
    });
  }

  return (
    <>
      <KeyCaptureField
        label="Push-to-Talk Key"
        description="Key to hold while speaking to the agent"
        pttKey={settings.pttKey}
        onChange={updatePttKey}
      />
      <div className="stg-section-divider" />
      <PhraseListField
        label="Approval Phrases"
        description="Phrases that confirm agent tool use requests"
        phrases={settings.approvalPhrases}
        onAdd={(p) => addPhrase('approvalPhrases', p)}
        onRemove={(i) => removePhrase('approvalPhrases', i)}
        badgeVariant="violet"
      />
      <div className="stg-section-divider" />
      <PhraseListField
        label="Deny Phrases"
        description="Phrases that reject agent tool use requests"
        phrases={settings.denyPhrases}
        onAdd={(p) => addPhrase('denyPhrases', p)}
        onRemove={(i) => removePhrase('denyPhrases', i)}
        badgeVariant="rose"
      />
    </>
  );
}

function PermissionsForm({
  settings,
  onSettingsChange,
}: {
  settings: AgentSettings;
  onSettingsChange: (s: AgentSettings) => void;
}) {
  function addToList(key: 'allowList' | 'disallowedTools', item: string) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }

  function removeFromList(key: 'allowList' | 'disallowedTools', index: number) {
    onSettingsChange({
      ...settings,
      [key]: settings[key].filter((_: string, i: number) => i !== index),
    });
  }

  return (
    <>
      <PhraseListField
        label="Default Allow List"
        description="Tools always permitted without asking"
        phrases={settings.allowList}
        onAdd={(p) => addToList('allowList', p)}
        onRemove={(i) => removeFromList('allowList', i)}
        badgeVariant="green"
      />
      <div className="stg-section-divider" />
      <PhraseListField
        label="Disallowed Tools"
        description="Tools the agent is never permitted to use"
        phrases={settings.disallowedTools}
        onAdd={(p) => addToList('disallowedTools', p)}
        onRemove={(i) => removeFromList('disallowedTools', i)}
        badgeVariant="rose"
      />
    </>
  );
}

const BACKEND_OPTIONS = [{ value: 'codex', label: 'Codex' }];

function BackendForm({
  settings,
  onSettingsChange,
}: {
  settings: AgentSettings;
  onSettingsChange: (s: AgentSettings) => void;
}) {
  function handleChange(value: string) {
    onSettingsChange({ ...settings, runtime: value as AgentSettings['runtime'] });
  }

  return (
    <div className="settings-label">
      Backend
      <span className="settings-label-desc">Runtime engine for agent sessions</span>
      <Select value={settings.runtime} onValueChange={handleChange}>
        <SelectTrigger className="stg-backend-trigger">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {BACKEND_OPTIONS.map((o) => (
            <SelectItem key={o.value} value={o.value}>
              {o.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
