import { useState, useCallback, useEffect } from "react";
import { dialog } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { useSettingsDraft } from "./hooks/useSettingsDraft";
import { useWorktreeDetail } from "./hooks/useWorktreeDetail";
import type { WorkspaceSelection } from "./hooks/useSettingsDraft";
import type { WorktreeData } from "./hooks/useWorktreeDetail";
import { Sidebar } from "./components/Sidebar";
import { MainPanel } from "./components/MainPanel";
import type { WorkspaceData } from "./components/WorkspaceOverview";
import type { MainPanelContent, SettingsCategory } from "./types";

interface BranchData {
  branches: string[];
  current: string | null;
}

interface WorktreeListResponse {
  worktrees: WorktreeData[];
}

export function App() {
  const draft = useSettingsDraft();
  const [content, setContent] = useState<MainPanelContent>({ kind: "empty" });
  const [activeCategory, setActiveCategory] = useState<SettingsCategory | null>(null);
  const [workspaceDataMap, setWorkspaceDataMap] = useState<Map<string, WorkspaceData>>(new Map());

  const paths = draft.settings.workspaces.paths;
  const selection = draft.settings.workspaces.selection;
  const workspacePath = paths[selection.workspaceIndex] ?? null;
  const worktreeName = content.kind === "worktreeDetail" ? content.worktreeName : null;
  const worktreeDetail = useWorktreeDetail(workspacePath, worktreeName);

  // Fetch workspace data for all paths
  const refreshWorkspaceData = useCallback(async () => {
    const entries = await Promise.all(
      paths.map(async (p): Promise<[string, WorkspaceData]> => {
        try {
          const branchData = await rpc.call<BranchData>({
            modName: "@hmcs/agent",
            method: "list-branches",
            body: { workspacePath: p },
          });
          const worktreeData = await rpc.call<WorktreeListResponse>({
            modName: "@hmcs/agent",
            method: "list-worktrees",
            body: { workspacePath: p },
          });
          return [p, { isGit: true, currentBranch: branchData.current, worktrees: worktreeData.worktrees }];
        } catch {
          return [p, { isGit: false, currentBranch: null, worktrees: [] }];
        }
      }),
    );
    setWorkspaceDataMap(new Map(entries));
  }, [paths]);

  useEffect(() => { refreshWorkspaceData(); }, [refreshWorkspaceData]);

  // Set initial content based on existing selection
  useEffect(() => {
    if (paths.length === 0) {
      setContent({ kind: "empty" });
    } else if (selection.worktreeName) {
      setContent({ kind: "worktreeDetail", workspaceIndex: selection.workspaceIndex, worktreeName: selection.worktreeName });
    } else {
      setContent({ kind: "workspaceOverview", workspaceIndex: selection.workspaceIndex });
    }
  }, [draft.loading]);

  function updateSelection(newSelection: WorkspaceSelection) {
    draft.setSettings({
      ...draft.settings,
      workspaces: { ...draft.settings.workspaces, selection: newSelection },
    });
  }

  function handleSelectionChange(newSelection: WorkspaceSelection) {
    updateSelection(newSelection);
    setActiveCategory(null);
    if (newSelection.worktreeName) {
      setContent({ kind: "worktreeDetail", workspaceIndex: newSelection.workspaceIndex, worktreeName: newSelection.worktreeName });
    } else {
      setContent({ kind: "workspaceOverview", workspaceIndex: newSelection.workspaceIndex });
    }
  }

  function handleCategorySelect(category: SettingsCategory) {
    setActiveCategory(category);
    setContent({ kind: "settingsForm", category });
  }

  /** Called by WorkspaceTree after its internal folder picker resolves. */
  function handleAddWorkspaceFromTree(path: string) {
    const newPaths = [...paths, path];
    const newIndex = newPaths.length - 1;
    draft.setSettings({
      ...draft.settings,
      workspaces: { paths: newPaths, selection: { workspaceIndex: newIndex, worktreeName: null } },
    });
    setActiveCategory(null);
    setContent({ kind: "workspaceOverview", workspaceIndex: newIndex });
  }

  /** Called by the MainPanel empty-state button — drives its own folder picker. */
  const handleAddWorkspaceFromPanel = useCallback(async () => {
    try {
      const path = await dialog.pickFolder({ title: "Select workspace directory" });
      if (!path) return;
      handleAddWorkspaceFromTree(path);
    } catch (e) {
      console.error("pickFolder failed:", e);
    }
  }, [paths, draft]);

  function handleRemoveWorkspace(index: number) {
    const newPaths = paths.filter((_, i) => i !== index);
    const sel = selection;
    const newSelection = sel.workspaceIndex >= newPaths.length
      ? { workspaceIndex: Math.max(0, newPaths.length - 1), worktreeName: null }
      : sel.workspaceIndex > index
        ? { ...sel, workspaceIndex: sel.workspaceIndex - 1 }
        : sel;
    draft.setSettings({ ...draft.settings, workspaces: { paths: newPaths, selection: newSelection } });
    if (newPaths.length === 0) {
      setContent({ kind: "empty" });
      setActiveCategory(null);
    }
  }

  if (draft.loading) return null;

  return (
    <div className="stg-chrome">
      <Sidebar
        paths={paths}
        selection={selection}
        onSelectionChange={handleSelectionChange}
        onAddWorkspace={handleAddWorkspaceFromTree}
        onRemoveWorkspace={handleRemoveWorkspace}
        activeCategory={activeCategory}
        onCategorySelect={handleCategorySelect}
      />
      <div className="stg-divider" />
      <MainPanel
        content={content}
        worktreeData={worktreeDetail.data}
        workspacePath={workspacePath}
        workspaceData={workspacePath ? workspaceDataMap.get(workspacePath) : undefined}
        settings={draft.settings}
        onSettingsChange={draft.setSettings}
        apiKey={draft.apiKey}
        onApiKeyChange={draft.setApiKey}
        onApiKeySave={draft.saveApiKey}
        savingApiKey={draft.savingApiKey}
        onAddWorktree={() => {}}
        onAddWorkspace={handleAddWorkspaceFromPanel}
      />
      {draft.isDirty && (
        <div className="stg-save-bar">
          <button className="stg-action-btn stg-action-btn--primary" type="button"
            onClick={draft.saveSettings} disabled={draft.saving}>
            {draft.saving ? "Saving..." : "Save Changes"}
          </button>
        </div>
      )}
    </div>
  );
}
