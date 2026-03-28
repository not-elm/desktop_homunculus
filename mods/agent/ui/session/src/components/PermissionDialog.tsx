import type { Decision, PendingPermission } from "../hooks/useAgentSession";

interface PermissionDialogProps {
  permission: PendingPermission | null;
  onApprove: (requestId: string, decision?: Decision) => void;
  onDeny: (requestId: string, decision?: Decision) => void;
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
  onApprove: (requestId: string, decision?: Decision) => void;
  onDeny: (requestId: string, decision?: Decision) => void;
}) {
  const { availableDecisions } = permission;

  if (!availableDecisions || availableDecisions.length === 0) {
    return <FallbackButtons requestId={permission.requestId} onApprove={onApprove} onDeny={onDeny} />;
  }

  return (
    <>
      {availableDecisions.map((decision) => (
        <DynamicDecisionButton
          key={decisionKey(decision)}
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
  onApprove: (requestId: string, decision?: Decision) => void;
  onDeny: (requestId: string, decision?: Decision) => void;
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

/** Generate a stable string key for a decision (for React keys). */
function decisionKey(decision: Decision): string {
  if (typeof decision === "string") return decision;
  return JSON.stringify(decision);
}

/** Extract the tag key from a tagged-union decision object, or null for strings. */
function decisionTag(decision: Decision): string | null {
  if (typeof decision === "string") return null;
  const keys = Object.keys(decision);
  return keys.length > 0 ? keys[0] : null;
}

/** Maps a decision to its display label, CSS class, and approval semantics. */
function decisionMeta(decision: Decision): { label: string; className: string; isApproval: boolean } {
  if (typeof decision === "string") {
    return stringDecisionMeta(decision);
  }
  return objectDecisionMeta(decision);
}

function stringDecisionMeta(decision: string): { label: string; className: string; isApproval: boolean } {
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

function objectDecisionMeta(decision: Record<string, unknown>): { label: string; className: string; isApproval: boolean } {
  const tag = decisionTag(decision);

  switch (tag) {
    case "acceptWithExecpolicyAmendment":
      return { label: "Always Allow", className: "hud-btn hud-btn--policy", isApproval: true };
    case "applyNetworkPolicyAmendment": {
      const inner = decision[tag] as { network_policy_amendment?: { host?: string; action?: string } } | undefined;
      const host = inner?.network_policy_amendment?.host ?? "unknown";
      const action = inner?.network_policy_amendment?.action ?? "allow";
      const label = action === "deny" ? `Block ${host}` : `Allow ${host}`;
      return { label, className: "hud-btn hud-btn--policy", isApproval: action !== "deny" };
    }
    default:
      return { label: tag ?? "Unknown", className: "hud-btn", isApproval: false };
  }
}

function DynamicDecisionButton({
  decision,
  requestId,
  onApprove,
  onDeny,
}: {
  decision: Decision;
  requestId: string;
  onApprove: (requestId: string, decision?: Decision) => void;
  onDeny: (requestId: string, decision?: Decision) => void;
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
