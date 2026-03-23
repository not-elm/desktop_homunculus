import { useState } from "react";
import { useAgentSession } from "./hooks/useAgentSession";
import { StatusBar } from "./components/StatusBar";
import { ActivityLog } from "./components/ActivityLog";
import { PermissionDialog } from "./components/PermissionDialog";
import { QuestionDialog } from "./components/QuestionDialog";
import type { AgentState } from "./hooks/useAgentSession";

const APPROVAL_HINTS = ["はい", "yes", "ok", "allow"];
const DENY_HINTS = ["いいえ", "no", "deny", "cancel"];

export function App() {
  const [expanded, setExpanded] = useState(false);
  const session = useAgentSession();

  if (!expanded) {
    return (
      <CollapsedPill
        state={session.state}
        elapsedMs={session.elapsedMs}
        hasPending={session.hasPending}
        onExpand={() => setExpanded(true)}
      />
    );
  }

  return (
    <ExpandedPanel
      session={session}
      onCollapse={() => setExpanded(false)}
    />
  );
}

interface CollapsedPillProps {
  state: AgentState;
  elapsedMs: number;
  hasPending: boolean;
  onExpand: () => void;
}

function CollapsedPill({ state, elapsedMs, hasPending, onExpand }: CollapsedPillProps) {
  return (
    <div className="hud-pill" onClick={onExpand}>
      <span className={`hud-status-dot hud-status-dot--${state}`} />
      <span className={`hud-status-label hud-status-label--${state}`}>
        {stateLabel(state)}
      </span>
      {state !== "idle" && (
        <span className="hud-timer">{formatElapsed(elapsedMs)}</span>
      )}
      {hasPending && <span className="hud-notification-dot" />}
      <ExpandIcon />
    </div>
  );
}

function ExpandIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none" style={{ flexShrink: 0 }}>
      <path d="M2 3L5 6.5L8 3" stroke="currentColor" strokeWidth="1.3" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

interface ExpandedPanelProps {
  session: ReturnType<typeof useAgentSession>;
  onCollapse: () => void;
}

function ExpandedPanel({ session, onCollapse }: ExpandedPanelProps) {
  return (
    <div className="hud-panel">
      <HudDecorations />
      <StatusBar
        state={session.state}
        elapsedMs={session.elapsedMs}
        onToggleCollapse={onCollapse}
      />
      <ActivityLog entries={session.entries} />
      <PermissionDialog
        permission={session.permission}
        approvalHints={APPROVAL_HINTS}
        denyHints={DENY_HINTS}
        onApprove={session.approvePermission}
        onDeny={session.denyPermission}
      />
      <QuestionDialog
        question={session.question}
        onAnswer={session.answerQuestion}
      />
    </div>
  );
}

function HudDecorations() {
  return (
    <>
      <div className="hud-highlight" />
      <div className="hud-scanline" />
      <span className="hud-corner hud-corner--tl" />
      <span className="hud-corner hud-corner--tr" />
      <span className="hud-corner hud-corner--bl" />
      <span className="hud-corner hud-corner--br" />
    </>
  );
}

function stateLabel(state: AgentState): string {
  switch (state) {
    case "idle": return "Standby";
    case "thinking": return "Thinking";
    case "executing": return "Working";
    case "waiting": return "Waiting";
  }
}

function formatElapsed(ms: number): string {
  const totalSec = Math.floor(ms / 1000);
  const mins = Math.floor(totalSec / 60);
  const secs = totalSec % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}
