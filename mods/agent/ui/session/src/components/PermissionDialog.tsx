import type { PendingPermission } from "../hooks/useAgentSession";

interface PermissionDialogProps {
  permission: PendingPermission | null;
  onApprove: (requestId: string, decision?: string) => void;
  onDeny: (requestId: string, decision?: string) => void;
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
        <DecisionButtons
          permission={permission}
          onApprove={onApprove}
          onDeny={onDeny}
        />
      </div>
    </div>
  );
}

function DecisionButtons({
  permission,
  onApprove,
  onDeny,
}: {
  permission: PendingPermission;
  onApprove: (requestId: string, decision?: string) => void;
  onDeny: (requestId: string, decision?: string) => void;
}) {
  const { availableDecisions } = permission;

  if (!availableDecisions || availableDecisions.length === 0) {
    return <FallbackButtons requestId={permission.requestId} onApprove={onApprove} onDeny={onDeny} />;
  }

  return (
    <>
      {availableDecisions.map((decision) => (
        <DynamicDecisionButton
          key={decision}
          decision={decision}
          requestId={permission.requestId}
          onApprove={onApprove}
          onDeny={onDeny}
        />
      ))}
    </>
  );
}

function FallbackButtons({
  requestId,
  onApprove,
  onDeny,
}: {
  requestId: string;
  onApprove: (requestId: string, decision?: string) => void;
  onDeny: (requestId: string, decision?: string) => void;
}) {
  return (
    <>
      <button
        className="hud-btn hud-btn--approve"
        onClick={() => onApprove(requestId)}
      >
        Approve
      </button>
      <button
        className="hud-btn hud-btn--deny"
        onClick={() => onDeny(requestId)}
      >
        Deny
      </button>
    </>
  );
}

/** Maps a decision string to its display label and CSS class. */
function decisionMeta(decision: string): { label: string; className: string; isApproval: boolean } {
  switch (decision) {
    case "accept":
      return { label: "Approve", className: "hud-btn hud-btn--approve", isApproval: true };
    case "acceptForSession":
      return { label: "Allow for Session", className: "hud-btn hud-btn--approve-session", isApproval: true };
    case "decline":
      return { label: "Deny", className: "hud-btn hud-btn--deny", isApproval: false };
    case "cancel":
      return { label: "Cancel", className: "hud-btn hud-btn--cancel", isApproval: false };
    default:
      return { label: decision, className: "hud-btn", isApproval: false };
  }
}

function DynamicDecisionButton({
  decision,
  requestId,
  onApprove,
  onDeny,
}: {
  decision: string;
  requestId: string;
  onApprove: (requestId: string, decision?: string) => void;
  onDeny: (requestId: string, decision?: string) => void;
}) {
  const { label, className, isApproval } = decisionMeta(decision);
  const handler = isApproval ? onApprove : onDeny;

  return (
    <button
      className={className}
      onClick={() => handler(requestId, decision)}
    >
      {label}
    </button>
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
