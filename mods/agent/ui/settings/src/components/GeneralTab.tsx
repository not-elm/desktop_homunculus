import type { AgentSettings } from "../hooks/useAgentSettings";
import { PhraseListField } from "./PhraseListField";
import { DirectoryListField } from "./DirectoryListField";
import { KeyCaptureField } from "./KeyCaptureField";

interface GeneralTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
}

export function GeneralTab({ settings, onSettingsChange }: GeneralTabProps) {
  function update<K extends keyof AgentSettings>(
    key: K,
    value: AgentSettings[K],
  ) {
    onSettingsChange({ ...settings, [key]: value });
  }

  function addPhrase(key: keyof Pick<AgentSettings, "greetingPhrases" | "completionPhrases" | "errorPhrases">, phrase: string) {
    update(key, [...settings[key], phrase]);
  }

  function removePhrase(key: keyof Pick<AgentSettings, "greetingPhrases" | "completionPhrases" | "errorPhrases">, index: number) {
    update(key, settings[key].filter((_, i) => i !== index));
  }

  function addDirectory(path: string) {
    const { paths, default: def } = settings.workingDirectories;
    update("workingDirectories", { paths: [...paths, path], default: def });
  }

  function removeDirectory(index: number) {
    const { paths, default: def } = settings.workingDirectories;
    const newPaths = paths.filter((_, i) => i !== index);
    const newDefault = def >= newPaths.length ? Math.max(0, newPaths.length - 1) : def;
    update("workingDirectories", { paths: newPaths, default: newDefault });
  }

  function setDefaultDirectory(index: number) {
    update("workingDirectories", {
      ...settings.workingDirectories,
      default: index,
    });
  }

  return (
    <div className="settings-section">
      <KeyCaptureField
        label="Push-to-Talk Key"
        description="Press the key to capture it"
        pttKey={settings.pttKey}
        onChange={(key) => update("pttKey", key)}
      />

      <div className="agent-divider" />

      <PhraseListField
        label="Approval Phrases"
        description="Phrases to approve agent actions"
        phrases={settings.approvalPhrases}
        onAdd={(p) => update("approvalPhrases", [...settings.approvalPhrases, p])}
        onRemove={(i) => update("approvalPhrases", settings.approvalPhrases.filter((_, idx) => idx !== i))}
      />

      <PhraseListField
        label="Deny Phrases"
        description="Phrases to deny agent actions"
        phrases={settings.denyPhrases}
        onAdd={(p) => update("denyPhrases", [...settings.denyPhrases, p])}
        onRemove={(i) => update("denyPhrases", settings.denyPhrases.filter((_, idx) => idx !== i))}
      />

      <div className="agent-divider" />

      <PhraseListField
        label="Greeting Phrases"
        description="Said when the agent activates"
        phrases={settings.greetingPhrases}
        onAdd={(p) => addPhrase("greetingPhrases", p)}
        onRemove={(i) => removePhrase("greetingPhrases", i)}
      />

      <PhraseListField
        label="Completion Phrases"
        description="Said when a task finishes"
        phrases={settings.completionPhrases}
        onAdd={(p) => addPhrase("completionPhrases", p)}
        onRemove={(i) => removePhrase("completionPhrases", i)}
      />

      <PhraseListField
        label="Error Phrases"
        description="Said when an error occurs"
        phrases={settings.errorPhrases}
        onAdd={(p) => addPhrase("errorPhrases", p)}
        onRemove={(i) => removePhrase("errorPhrases", i)}
      />

      <div className="agent-divider" />

      <DirectoryListField
        label="Working Directories"
        description="Directories available to the agent. Select default with radio button."
        paths={settings.workingDirectories.paths}
        defaultIndex={settings.workingDirectories.default}
        onAdd={addDirectory}
        onRemove={removeDirectory}
        onSetDefault={setDefaultDirectory}
      />
    </div>
  );
}
