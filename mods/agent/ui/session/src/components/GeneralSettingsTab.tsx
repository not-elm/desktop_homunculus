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
    key: keyof Pick<AgentSettings, "approvalPhrases" | "denyPhrases">,
    item: string,
  ) {
    onSettingsChange({ ...settings, [key]: [...settings[key], item] });
  }

  function removeFromList(
    key: keyof Pick<AgentSettings, "approvalPhrases" | "denyPhrases">,
    index: number,
  ) {
    onSettingsChange({ ...settings, [key]: settings[key].filter((_: string, i: number) => i !== index) });
  }

  function addDirectory(path: string) {
    const { paths, selection } = settings.workspaces;
    update("workspaces", { paths: [...paths, path], selection });
  }

  function removeDirectory(index: number) {
    const { paths, selection } = settings.workspaces;
    const newPaths = paths.filter((_, i) => i !== index);
    const newIndex = selection.workspaceIndex >= newPaths.length
      ? Math.max(0, newPaths.length - 1)
      : selection.workspaceIndex;
    update("workspaces", { paths: newPaths, selection: { ...selection, workspaceIndex: newIndex } });
  }

  function setDefaultDirectory(index: number) {
    update("workspaces", { ...settings.workspaces, selection: { ...settings.workspaces.selection, workspaceIndex: index } });
  }

  return (
    <div className="settings-section">
      <DirectoryListField
        label="Working Directories"
        description="Directories available to the agent"
        paths={settings.workspaces.paths}
        defaultIndex={settings.workspaces.selection.workspaceIndex}
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

    </div>
  );
}
