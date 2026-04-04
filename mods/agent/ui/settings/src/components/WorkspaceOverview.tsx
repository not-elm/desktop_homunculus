import { FolderGit2, Folder, Trash2 } from "lucide-react";
import type { WorktreeData } from "../hooks/useWorktreeDetail";

export interface WorkspaceData {
  isGit: boolean;
  currentBranch: string | null;
  worktrees: WorktreeData[];
}

interface WorkspaceOverviewProps {
  path: string;
  data: WorkspaceData | undefined;
  onAddWorktree: () => void;
  onRemoveWorktree: (wt: WorktreeData) => void;
}

export function WorkspaceOverview({ path, data, onAddWorktree, onRemoveWorktree }: WorkspaceOverviewProps) {
  const dirName = path.split(/[/\\]/).pop() || path;
  const FolderIcon = data?.isGit ? FolderGit2 : Folder;

  return (
    <div className="stg-detail">
      <div className="stg-detail-header">
        <FolderIcon className="stg-detail-icon" />
        <h2 className="stg-detail-title">{dirName}</h2>
        {data?.currentBranch && (
          <span className="stg-detail-branch-tag">{data.currentBranch}</span>
        )}
      </div>

      <div className="stg-detail-section">
        <h3 className="stg-section-header">Path</h3>
        <p className="stg-detail-path">{path}</p>
      </div>

      {data?.isGit && (
        <div className="stg-detail-section">
          <div className="stg-section-header-row">
            <h3 className="stg-section-header">Worktrees ({data.worktrees.length})</h3>
            <button className="ws-add-icon-btn" type="button" onClick={onAddWorktree} title="Add Worktree">+</button>
          </div>
          {data.worktrees.length > 0 && (
            <div className="stg-wt-summary">
              {data.worktrees.map((wt) => (
                <div key={wt.name} className="stg-wt-summary-row">
                  <span className={`stg-status-dot ${wt.hasUncommittedChanges ? "stg-status-dot--dirty" : "stg-status-dot--clean"}`} />
                  <span className="sr-only">{wt.hasUncommittedChanges ? "uncommitted" : "clean"}</span>
                  <span className="stg-wt-summary-name">{wt.name}</span>
                  <span className="stg-wt-summary-branch">{wt.branch}</span>
                  <span className="stg-wt-summary-commits">{wt.commits} commits</span>
                  <button className="stg-wt-remove-btn" type="button" onClick={() => onRemoveWorktree(wt)} title="Remove Worktree">
                    <Trash2 size={14} />
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
