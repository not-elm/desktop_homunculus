import type { AgentSettings } from "../hooks/useAgentSettings";
import { PhraseListField } from "./PhraseListField";
import { WorkspaceTree } from "./WorkspaceTree.tsx";
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

  function addWorkspace(path: string) {
    update("workspaces", {
      ...settings.workspaces,
      paths: [...settings.workspaces.paths, path],
    });
  }

  function removeWorkspace(index: number) {
    const newPaths = settings.workspaces.paths.filter((_, i) => i !== index);
    const sel = settings.workspaces.selection;
    const newSelection =
      sel.workspaceIndex >= newPaths.length
        ? { workspaceIndex: Math.max(0, newPaths.length - 1), worktreeName: null }
        : sel.workspaceIndex > index
          ? { ...sel, workspaceIndex: sel.workspaceIndex - 1 }
          : sel;
    update("workspaces", { paths: newPaths, selection: newSelection });
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

      <WorkspaceTree
        paths={settings.workspaces.paths}
        selection={settings.workspaces.selection}
        onSelectionChange={(selection) =>
          update("workspaces", { ...settings.workspaces, selection })
        }
        onAddWorkspace={addWorkspace}
        onRemoveWorkspace={removeWorkspace}
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
