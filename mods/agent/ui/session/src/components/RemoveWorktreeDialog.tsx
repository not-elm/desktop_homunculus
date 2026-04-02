import { useState } from "react";
import { rpc } from "@hmcs/sdk/rpc";

interface WorktreeDetails {
  name: string;
  commits: number;
  filesChanged: number;
  insertions: number;
  deletions: number;
  hasUncommittedChanges: boolean;
  canMerge: boolean;
}

interface RemoveWorktreeDialogProps {
  workspacePath: string;
  worktree: WorktreeDetails;
  onRemoved: () => void;
  onCancel: () => void;
}

export function RemoveWorktreeDialog({
  workspacePath,
  worktree,
  onRemoved,
  onCancel,
}: RemoveWorktreeDialogProps) {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function handleAction(action: "merge" | "remove") {
    setBusy(true);
    setError(null);
    try {
      const result = await rpc.call<{ success: boolean; error?: string }>({
        modName: "@hmcs/agent",
        method: "remove-worktree",
        body: { workspacePath, name: worktree.name, action },
      });
      if (!result.success) {
        setError(result.error ?? "Operation failed");
        return;
      }
      onRemoved();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  }

  const actionsDisabled = busy || worktree.hasUncommittedChanges;

  return (
    <div className="wt-dialog-overlay">
      <div className="wt-dialog">
        <div className="wt-dialog-title">
          Remove worktree &ldquo;{worktree.name}&rdquo;?
        </div>
        <ChangeSummary worktree={worktree} />
        {worktree.hasUncommittedChanges && (
          <p className="wt-dialog-warning">
            Cannot remove: worktree has uncommitted changes. Commit or stash
            them first.
          </p>
        )}
        {error && <p className="wt-dialog-error">{error}</p>}
        <div className="wt-dialog-buttons">
          {worktree.canMerge && (
            <button
              className="wt-btn wt-btn-merge"
              type="button"
              disabled={actionsDisabled}
              onClick={() => handleAction("merge")}
            >
              Merge &amp; Remove
              <span className="wt-btn-hint">Merge into base, then remove</span>
            </button>
          )}
          <button
            className="wt-btn wt-btn-remove"
            type="button"
            disabled={actionsDisabled}
            onClick={() => handleAction("remove")}
          >
            Remove
            <span className="wt-btn-hint">Discard changes and remove</span>
          </button>
          <button
            className="wt-btn wt-btn-cancel"
            type="button"
            onClick={onCancel}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}

function ChangeSummary({ worktree }: { worktree: WorktreeDetails }) {
  if (worktree.commits === 0 && worktree.filesChanged === 0) {
    return <div className="wt-dialog-info">No changes from base branch.</div>;
  }

  return (
    <div className="wt-dialog-info">
      {worktree.commits} commit{worktree.commits !== 1 ? "s" : ""} &middot;{" "}
      {worktree.filesChanged} file{worktree.filesChanged !== 1 ? "s" : ""}{" "}
      changed{" "}
      <span className="wt-stat-add">+{worktree.insertions}</span>{" / "}
      <span className="wt-stat-del">-{worktree.deletions}</span>
    </div>
  );
}
