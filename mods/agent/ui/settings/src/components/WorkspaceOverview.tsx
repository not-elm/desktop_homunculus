import { FolderGit2, Folder } from "lucide-react";
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
}

export function WorkspaceOverview({ path, data, onAddWorktree }: WorkspaceOverviewProps) {
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

      {data && data.worktrees.length > 0 && (
        <div className="stg-detail-section">
          <h3 className="stg-section-header">Worktrees ({data.worktrees.length})</h3>
          <div className="stg-wt-summary">
            {data.worktrees.map((wt) => (
              <div key={wt.name} className="stg-wt-summary-row">
                <span className={`stg-status-dot ${wt.hasUncommittedChanges ? "stg-status-dot--dirty" : "stg-status-dot--clean"}`} />
                <span className="sr-only">{wt.hasUncommittedChanges ? "uncommitted" : "clean"}</span>
                <span className="stg-wt-summary-name">{wt.name}</span>
                <span className="stg-wt-summary-branch">{wt.branch}</span>
                <span className="stg-wt-summary-commits">{wt.commits} commits</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {data?.isGit && (
        <button className="stg-action-btn" type="button" onClick={onAddWorktree}>
          + Add Worktree
        </button>
      )}
    </div>
  );
}
