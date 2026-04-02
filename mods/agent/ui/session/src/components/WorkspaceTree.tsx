import { useCallback, useEffect, useState } from "react";
import { dialog } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import type { WorkspaceSelection } from "../hooks/useAgentSettings";
import { AddWorktreeForm } from "./AddWorktreeForm.tsx";
import { RemoveWorktreeDialog } from "./RemoveWorktreeDialog.tsx";
import { RemoveWorkspaceDialog } from "./RemoveWorkspaceDialog.tsx";

interface WorkspaceTreeProps {
  paths: string[];
  selection: WorkspaceSelection;
  onSelectionChange: (selection: WorkspaceSelection) => void;
  onAddWorkspace: (path: string) => void;
  onRemoveWorkspace: (index: number) => void;
}

interface WorktreeData {
  name: string;
  branch: string;
  baseBranch: string;
  commits: number;
  filesChanged: number;
  insertions: number;
  deletions: number;
  hasUncommittedChanges: boolean;
  canMerge: boolean;
}

interface WorkspaceData {
  isGit: boolean;
  currentBranch: string | null;
  worktrees: WorktreeData[];
}

export interface BranchData {
  branches: string[];
  current: string | null;
}

interface WorktreeListResponse {
  worktrees: WorktreeData[];
}

type DialogState =
  | { type: "none" }
  | { type: "removeWorkspace"; index: number }
  | { type: "removeWorktree"; workspaceIndex: number; worktree: WorktreeData }
  | { type: "addWorktree"; workspaceIndex: number };

export function WorkspaceTree({
  paths,
  selection,
  onSelectionChange,
  onAddWorkspace,
  onRemoveWorkspace,
}: WorkspaceTreeProps) {
  const [workspaceData, setWorkspaceData] = useState<
    Map<string, WorkspaceData>
  >(new Map());
  const [dialogState, setDialogState] = useState<DialogState>({ type: "none" });

  const fetchWorkspaceData = useCallback(
    async function fetchWorkspaceData(path: string): Promise<WorkspaceData> {
      try {
        const branchData = await rpc.call<BranchData>({
          modName: "@hmcs/agent",
          method: "list-branches",
          body: { workspacePath: path },
        });
        const worktreeData = await rpc.call<WorktreeListResponse>({
          modName: "@hmcs/agent",
          method: "list-worktrees",
          body: { workspacePath: path },
        });
        return {
          isGit: true,
          currentBranch: branchData.current,
          worktrees: worktreeData.worktrees,
        };
      } catch {
        return { isGit: false, currentBranch: null, worktrees: [] };
      }
    },
    [],
  );

  const refreshAll = useCallback(
    async function refreshAll() {
      const entries = await Promise.all(
        paths.map(async (p) => [p, await fetchWorkspaceData(p)] as const),
      );
      setWorkspaceData(new Map(entries));
    },
    [paths, fetchWorkspaceData],
  );

  useEffect(() => {
    refreshAll();
  }, [refreshAll]);

  async function handleAddWorkspace() {
    try {
      const path = await dialog.pickFolder({
        title: "Select workspace directory",
      });
      if (path) onAddWorkspace(path);
    } catch (e) {
      console.error("pickFolder failed:", e);
    }
  }

  function handleSelectWorkspace(index: number) {
    onSelectionChange({ workspaceIndex: index, worktreeName: null });
  }

  function handleSelectWorktree(workspaceIndex: number, name: string) {
    onSelectionChange({ workspaceIndex, worktreeName: name });
  }

  function handleWorktreeCreated() {
    setDialogState({ type: "none" });
    refreshAll();
  }

  function handleWorktreeRemoved() {
    setDialogState({ type: "none" });
    clearWorktreeSelectionIfNeeded();
    refreshAll();
  }

  function clearWorktreeSelectionIfNeeded() {
    if (dialogState.type !== "removeWorktree") return;
    const { workspaceIndex, worktree } = dialogState;
    if (
      selection.workspaceIndex === workspaceIndex &&
      selection.worktreeName === worktree.name
    ) {
      onSelectionChange({ workspaceIndex, worktreeName: null });
    }
  }

  function handleRemoveWorkspaceConfirmed() {
    if (dialogState.type !== "removeWorkspace") return;
    onRemoveWorkspace(dialogState.index);
    setDialogState({ type: "none" });
  }

  return (
    <div className="ws-section">
      <div className="ws-header">
        <div>
          <div className="ws-label">Workspaces</div>
          <div className="ws-desc">
            Select a workspace or worktree for the agent to work in.
          </div>
        </div>
        <button className="ws-add-btn" type="button" onClick={handleAddWorkspace}>
          <span className="ws-add-icon">+</span> Add Workspace
        </button>
      </div>

      <div className="ws-tree">
        {paths.map((path, index) => (
          <WorkspaceNode
            key={path}
            path={path}
            data={workspaceData.get(path)}
            isSelected={
              selection.workspaceIndex === index &&
              selection.worktreeName === null
            }
            selectedWorktree={
              selection.workspaceIndex === index ? selection.worktreeName : null
            }
            onSelectWorkspace={() => handleSelectWorkspace(index)}
            onSelectWorktree={(name) => handleSelectWorktree(index, name)}
            onRemoveWorkspace={() =>
              setDialogState({ type: "removeWorkspace", index })
            }
            onAddWorktree={() =>
              setDialogState({ type: "addWorktree", workspaceIndex: index })
            }
            onRemoveWorktree={(wt) =>
              setDialogState({
                type: "removeWorktree",
                workspaceIndex: index,
                worktree: wt,
              })
            }
            showAddForm={
              dialogState.type === "addWorktree" &&
              dialogState.workspaceIndex === index
            }
            onFormCreated={handleWorktreeCreated}
            onFormCancelled={() => setDialogState({ type: "none" })}
          />
        ))}
      </div>

      {dialogState.type === "removeWorkspace" && (
        <RemoveWorkspaceDialog
          path={paths[dialogState.index]}
          worktreeCount={
            workspaceData.get(paths[dialogState.index])?.worktrees.length ?? 0
          }
          onConfirm={handleRemoveWorkspaceConfirmed}
          onCancel={() => setDialogState({ type: "none" })}
        />
      )}
      {dialogState.type === "removeWorktree" && (
        <RemoveWorktreeDialog
          workspacePath={paths[dialogState.workspaceIndex]}
          worktree={dialogState.worktree}
          onRemoved={handleWorktreeRemoved}
          onCancel={() => setDialogState({ type: "none" })}
        />
      )}
    </div>
  );
}

interface WorkspaceNodeProps {
  path: string;
  data: WorkspaceData | undefined;
  isSelected: boolean;
  selectedWorktree: string | null;
  onSelectWorkspace: () => void;
  onSelectWorktree: (name: string) => void;
  onRemoveWorkspace: () => void;
  onAddWorktree: () => void;
  onRemoveWorktree: (wt: WorktreeData) => void;
  showAddForm: boolean;
  onFormCreated: () => void;
  onFormCancelled: () => void;
}

function WorkspaceNode({
  path,
  data,
  isSelected,
  selectedWorktree,
  onSelectWorkspace,
  onSelectWorktree,
  onRemoveWorkspace,
  onAddWorktree,
  onRemoveWorktree,
  showAddForm,
  onFormCreated,
  onFormCancelled,
}: WorkspaceNodeProps) {
  return (
    <div className="ws-node">
      <div className={`ws-dir-item${isSelected ? " ws-dir-item--selected" : ""}`}>
        <input
          className="ws-radio"
          type="radio"
          checked={isSelected}
          onChange={onSelectWorkspace}
          aria-label={`Select workspace ${path}`}
        />
        <span className="ws-icon">📁</span>
        <div className="ws-info">
          <span className="ws-name" title={path}>{path.split(/[/\\]/).pop() || path}</span>
          <span className="ws-meta ws-meta-path" title={path}>{path}</span>
          {data?.isGit && data.currentBranch && (
            <span className="ws-meta">{data.currentBranch}</span>
          )}
        </div>
        {data?.isGit && (
          <span className="agent-badge agent-badge--green">git</span>
        )}
        <div className="ws-actions">
          {data?.isGit && (
            <button
              className="ws-action-btn ws-action-add"
              type="button"
              onClick={onAddWorktree}
            >
              + Worktree
            </button>
          )}
          <button
            className="ws-action-btn ws-action-remove"
            type="button"
            onClick={onRemoveWorkspace}
            aria-label={`Remove ${path}`}
          >
            &times;
          </button>
        </div>
      </div>

      {data && data.worktrees.length > 0 && (
        <div className="ws-tree-connector">
          {data.worktrees.map((wt) => (
            <WorktreeNode
              key={wt.name}
              worktree={wt}
              isSelected={selectedWorktree === wt.name}
              onSelect={() => onSelectWorktree(wt.name)}
              onRemove={() => onRemoveWorktree(wt)}
            />
          ))}
        </div>
      )}

      {showAddForm && (
        <div className="ws-tree-connector">
          <AddWorktreeForm
            workspacePath={path}
            onCreated={onFormCreated}
            onCancel={onFormCancelled}
          />
        </div>
      )}
    </div>
  );
}

interface WorktreeNodeProps {
  worktree: WorktreeData;
  isSelected: boolean;
  onSelect: () => void;
  onRemove: () => void;
}

function WorktreeNode({
  worktree,
  isSelected,
  onSelect,
  onRemove,
}: WorktreeNodeProps) {
  return (
    <div className={`ws-dir-item ws-wt-item${isSelected ? " ws-dir-item--selected" : ""}`}>
      <input
        className="ws-radio"
        type="radio"
        checked={isSelected}
        onChange={onSelect}
        aria-label={`Select worktree ${worktree.name}`}
      />
      <span className="ws-icon">🌿</span>
      <div className="ws-info">
        <span className="ws-name">{worktree.name}</span>
        <span className="ws-meta">
          from {worktree.baseBranch}
          {worktree.commits > 0 && ` · ${worktree.commits} commit${worktree.commits !== 1 ? "s" : ""}`}
        </span>
      </div>
      <span className="agent-badge agent-badge--violet">{worktree.branch}</span>
      <div className="ws-actions">
        {worktree.canMerge && (
          <button
            className="ws-action-btn ws-action-merge"
            type="button"
            title="Fast-forward merge"
          >
            ↗ Merge
          </button>
        )}
        <button
          className="ws-action-btn ws-action-remove"
          type="button"
          onClick={onRemove}
          aria-label={`Remove worktree ${worktree.name}`}
        >
          &times;
        </button>
      </div>
    </div>
  );
}
