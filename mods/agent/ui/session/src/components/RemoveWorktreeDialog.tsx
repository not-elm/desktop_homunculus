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
    <div className="agent-dialog-overlay">
      <div className="agent-dialog">
        <p className="agent-dialog-title">Remove worktree "{worktree.name}"?</p>
        <ChangeSummary worktree={worktree} />
        {worktree.hasUncommittedChanges && (
          <p className="agent-dialog-warning">
            Cannot remove: worktree has uncommitted changes. Commit or stash
            them first.
          </p>
        )}
        {error && <p className="agent-dialog-error">{error}</p>}
        <div className="agent-dialog-actions">
          <button className="settings-close" type="button" onClick={onCancel}>
            Cancel
          </button>
          {worktree.canMerge && (
            <button
              className="settings-save"
              type="button"
              disabled={actionsDisabled}
              onClick={() => handleAction("merge")}
            >
              Merge & Remove
            </button>
          )}
          <button
            className="settings-save agent-dialog-danger"
            type="button"
            disabled={actionsDisabled}
            onClick={() => handleAction("remove")}
          >
            Remove
          </button>
        </div>
      </div>
    </div>
  );
}

function ChangeSummary({ worktree }: { worktree: WorktreeDetails }) {
  if (worktree.commits === 0 && worktree.filesChanged === 0) {
    return <p className="agent-dialog-detail">No changes from base branch.</p>;
  }

  return (
    <p className="agent-dialog-detail">
      {worktree.commits} commit{worktree.commits !== 1 ? "s" : ""},{" "}
      {worktree.filesChanged} file{worktree.filesChanged !== 1 ? "s" : ""}{" "}
      changed{" "}
      <span className="agent-dialog-insertions">
        +{worktree.insertions}
      </span>{" "}
      <span className="agent-dialog-deletions">-{worktree.deletions}</span>
    </p>
  );
}
