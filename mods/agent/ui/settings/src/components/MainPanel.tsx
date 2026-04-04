import type { MainPanelContent } from "../types";
import type { AgentSettings } from "../hooks/useSettingsDraft";
import type { WorktreeData } from "../hooks/useWorktreeDetail";
import type { WorkspaceData } from "./WorkspaceOverview";
import { WorktreeDetailView } from "./WorktreeDetailView";
import { WorkspaceOverview } from "./WorkspaceOverview";
import { SettingsFormView } from "./SettingsFormView";

interface MainPanelProps {
  content: MainPanelContent;
  worktreeData: WorktreeData | null;
  workspacePath: string | null;
  workspaceData: WorkspaceData | undefined;
  settings: AgentSettings;
  onSettingsChange: (settings: AgentSettings) => void;
  apiKey: string;
  onApiKeyChange: (key: string) => void;
  onApiKeySave: () => void;
  savingApiKey: boolean;
  onAddWorktree: () => void;
  onRemoveWorktree: (wt: WorktreeData) => void;
  onAddWorkspace: () => void;
}

export function MainPanel({
  content, worktreeData, workspacePath, workspaceData,
  settings, onSettingsChange, apiKey, onApiKeyChange, onApiKeySave, savingApiKey,
  onAddWorktree, onRemoveWorktree, onAddWorkspace,
}: MainPanelProps) {
  return (
    <main className="stg-main" role="main">
      <div className="stg-main-content">
        {content.kind === "worktreeDetail" && worktreeData && (
          <WorktreeDetailView worktree={worktreeData} />
        )}
        {content.kind === "workspaceOverview" && workspacePath && (
          <WorkspaceOverview path={workspacePath} data={workspaceData} onAddWorktree={onAddWorktree} onRemoveWorktree={onRemoveWorktree} />
        )}
        {content.kind === "settingsForm" && (
          <SettingsFormView
            category={content.category}
            settings={settings}
            onSettingsChange={onSettingsChange}
            apiKey={apiKey}
            onApiKeyChange={onApiKeyChange}
            onApiKeySave={onApiKeySave}
            savingApiKey={savingApiKey}
          />
        )}
        {content.kind === "empty" && <EmptyState onAddWorkspace={onAddWorkspace} />}
      </div>
    </main>
  );
}

function EmptyState({ onAddWorkspace }: { onAddWorkspace: () => void }) {
  return (
    <div className="stg-empty">
      <p className="stg-empty-text">No workspace configured.</p>
      <button className="stg-action-btn stg-action-btn--primary" type="button" onClick={onAddWorkspace}>
        + Add Workspace
      </button>
    </div>
  );
}
