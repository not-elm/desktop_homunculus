import type { PendingPermission } from "../hooks/useAgentSession";

interface PermissionDialogProps {
  permission: PendingPermission | null;
  onApprove: (requestId: string) => void;
  onDeny: (requestId: string) => void;
}

export function PermissionDialog({
  permission,
  onApprove,
  onDeny,
}: PermissionDialogProps) {
  if (!permission) return null;

  return (
    <div className="hud-dialog">
      <ToolHeader action={permission.action} />
      <TargetBlock target={permission.target} />
      <div className="hud-dialog-actions">
        <button
          className="hud-btn hud-btn--approve"
          onClick={() => onApprove(permission.requestId)}
        >
          Approve
        </button>
        <button
          className="hud-btn hud-btn--deny"
          onClick={() => onDeny(permission.requestId)}
        >
          Deny
        </button>
      </div>
    </div>
  );
}

function ToolHeader({ action }: { action: string }) {
  return (
    <div className="hud-dialog-tool-row">
      <span className="hud-dialog-tool-badge">{action}</span>
    </div>
  );
}

function TargetBlock({ target }: { target: string }) {
  return <div className="hud-dialog-target">{target}</div>;
}
