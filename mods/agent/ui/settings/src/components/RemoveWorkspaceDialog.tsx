interface RemoveWorkspaceDialogProps {
  path: string;
  worktreeCount: number;
  onConfirm: () => void;
  onCancel: () => void;
}

export function RemoveWorkspaceDialog({
  path,
  worktreeCount,
  onConfirm,
  onCancel,
}: RemoveWorkspaceDialogProps) {
  return (
    <div className="agent-dialog-overlay">
      <div className="agent-dialog">
        <p className="agent-dialog-title">Remove workspace from list?</p>
        <p className="agent-dialog-path">{path}</p>
        {worktreeCount > 0 && (
          <p className="agent-dialog-warning">
            This will also remove {worktreeCount} associated worktree
            {worktreeCount > 1 ? "s" : ""} from the list. Files on disk are not
            deleted.
          </p>
        )}
        <div className="agent-dialog-actions">
          <button className="settings-close" type="button" onClick={onCancel}>
            Cancel
          </button>
          <button
            className="settings-save agent-dialog-danger"
            type="button"
            onClick={onConfirm}
          >
            Remove from list
          </button>
        </div>
      </div>
    </div>
  );
}
