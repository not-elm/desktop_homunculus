import { useCallback, useEffect, useState } from "react";
import { dialog } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import type { WorkspaceSelection } from "../hooks/useSettingsDraft";
import { useTreeKeyboard } from "../hooks/useTreeKeyboard.ts";
import { WorkspaceNode, type WorkspaceData } from "./WorkspaceNode.tsx";
import { RemoveWorktreeDialog } from "./RemoveWorktreeDialog.tsx";
import { RemoveWorkspaceDialog } from "./RemoveWorkspaceDialog.tsx";
import { AddWorktreeDialog } from "./AddWorktreeDialog.tsx";
import type { WorktreeData } from "./WorktreeNode.tsx";

interface WorkspaceTreeProps {
  paths: string[];
  selection: WorkspaceSelection;
  onSelectionChange: (selection: WorkspaceSelection) => void;
  onAddWorkspace: (path: string) => void;
  onRemoveWorkspace: (index: number) => void;
  refreshKey?: number;
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
  refreshKey,
}: WorkspaceTreeProps) {
  const [workspaceData, setWorkspaceData] = useState<Map<string, WorkspaceData>>(new Map());
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
      } catch (e) {
        console.error("[WorkspaceTree] fetchWorkspaceData failed for", path, e);
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
  }, [refreshAll, refreshKey]);

  function handleSelect(element: HTMLElement) {
    const wsIndex = readWorkspaceIndex(element);
    const wtName = element.getAttribute("data-wt-name");
    if (wsIndex != null) {
      onSelectionChange({ workspaceIndex: wsIndex, worktreeName: wtName });
    }
  }

  const { treeRef, handleKeyDown } = useTreeKeyboard({ onSelect: handleSelect });

  async function handleAddWorkspace() {
    try {
      const path = await dialog.pickFolder({ title: "Select workspace directory" });
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
        <div className="ws-label">Workspaces</div>
        <button className="ws-add-icon-btn" type="button" onClick={handleAddWorkspace} title="Add Workspace">
          +
        </button>
      </div>

      <div className="ws-tree" role="tree" aria-label="Workspaces" ref={treeRef} onKeyDown={handleKeyDown}>
        {paths.map((path, index) => (
          <WorkspaceNode
            key={path}
            index={index}
            path={path}
            data={workspaceData.get(path)}
            isSelected={selection.workspaceIndex === index && selection.worktreeName === null}
            selectedWorktree={selection.workspaceIndex === index ? selection.worktreeName : null}
            tabIndex={index === 0 ? 0 : -1}
            onSelectWorkspace={() => handleSelectWorkspace(index)}
            onSelectWorktree={(name) => handleSelectWorktree(index, name)}
            onRemoveWorkspace={() => setDialogState({ type: "removeWorkspace", index })}
            onAddWorktree={() => setDialogState({ type: "addWorktree", workspaceIndex: index })}
            onRemoveWorktree={(wt) =>
              setDialogState({ type: "removeWorktree", workspaceIndex: index, worktree: wt })
            }
            onKeyDown={handleKeyDown}
          />
        ))}
      </div>

      {dialogState.type === "removeWorkspace" && (
        <RemoveWorkspaceDialog
          path={paths[dialogState.index]}
          worktreeCount={workspaceData.get(paths[dialogState.index])?.worktrees.length ?? 0}
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
      {dialogState.type === "addWorktree" && (
        <AddWorktreeDialog
          workspacePath={paths[dialogState.workspaceIndex]}
          onCreated={handleWorktreeCreated}
          onCancel={() => setDialogState({ type: "none" })}
        />
      )}
    </div>
  );
}

function readWorkspaceIndex(element: HTMLElement): number | null {
  const wsAttr = element.getAttribute("data-ws-index");
  if (wsAttr != null) return Number(wsAttr);
  const parent = element.closest("[data-ws-index]");
  if (parent) return Number(parent.getAttribute("data-ws-index"));
  return null;
}
