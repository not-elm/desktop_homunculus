import type { AgentSettings } from "../hooks/useAgentSettings";
import { PhraseListField } from "./PhraseListField";
import { DirectoryListField } from "./DirectoryListField";

interface GeneralSettingsTabProps {
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
}

export function GeneralSettingsTab({ settings, onSettingsChange }: GeneralSettingsTabProps) {
  function update<K extends keyof AgentSettings>(key: K, value: AgentSettings[K]) {
    onSettingsChange({ ...settings, [key]: value });
  }

  function addToList(
    key: keyof Pick<AgentSettings, "approvalPhrases" | "denyPhrases" | "allowList" | "disallowedTools">,
    item: string,
  ) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }

  function removeFromList(
    key: keyof Pick<AgentSettings, "approvalPhrases" | "denyPhrases" | "allowList" | "disallowedTools">,
    index: number,
  ) {
    onSettingsChange({ ...settings, [key]: settings[key].filter((_: string, i: number) => i !== index) });
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
    update("workingDirectories", { ...settings.workingDirectories, default: index });
  }

  return (
    <div className="settings-section">
      <DirectoryListField
        label="Working Directories"
        description="Directories available to the agent"
        paths={settings.workingDirectories.paths}
        defaultIndex={settings.workingDirectories.default}
        onAdd={addDirectory}
        onRemove={removeDirectory}
        onSetDefault={setDefaultDirectory}
      />

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
