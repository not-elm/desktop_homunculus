import type { PendingPermission } from "../hooks/useAgentSession";

interface PermissionDialogProps {
  permission: PendingPermission | null;
  approvalHints: string[];
  denyHints: string[];
  onApprove: (requestId: string) => void;
  onDeny: (requestId: string) => void;
}

export function PermissionDialog({
  permission,
  approvalHints,
  denyHints,
  onApprove,
  onDeny,
}: PermissionDialogProps) {
  if (!permission) return null;

  return (
    <div className="hud-dialog">
      <ToolHeader action={permission.action} />
      <TargetBlock target={permission.target} />
      {(approvalHints.length > 0 || denyHints.length > 0) && (
        <VoiceHints approvalHints={approvalHints} denyHints={denyHints} />
      )}
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

function VoiceHints({
  approvalHints,
  denyHints,
}: {
  approvalHints: string[];
  denyHints: string[];
}) {
  return (
    <div className="hud-dialog-hints">
      {approvalHints.length > 0 && (
        <div>
          <span className="hud-dialog-hint-approve">Approve: </span>
          {approvalHints.join(", ")}
        </div>
      )}
      {denyHints.length > 0 && (
        <div>
          <span className="hud-dialog-hint-deny">Deny: </span>
          {denyHints.join(", ")}
        </div>
      )}
    </div>
  );
}
