import { useState } from "react";
import { useAgentSession } from "./hooks/useAgentSession";
import { StatusBar } from "./components/StatusBar";
import { ActivityLog } from "./components/ActivityLog";
import { PermissionDialog } from "./components/PermissionDialog";
import { QuestionDialog } from "./components/QuestionDialog";
import type { AgentState } from "./hooks/useAgentSession";

export function App() {
  const [expanded, setExpanded] = useState(false);
  const session = useAgentSession();

  if (!expanded) {
    return (
      <CollapsedPill
        state={session.state}
        elapsedMs={session.elapsedMs}
        hasPending={session.hasPending}
        isRecording={session.isRecording}
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
  isRecording?: boolean;
  onExpand: () => void;
}

function CollapsedPill({ state, elapsedMs, hasPending, isRecording, onExpand }: CollapsedPillProps) {
  return (
    <div className="hud-pill" onClick={onExpand}>
      <PillIndicator state={state} isRecording={isRecording} />
      <span className={`hud-status-label hud-status-label--${isRecording ? "listening" : state}`}>
        {isRecording ? "Listening..." : stateLabel(state)}
      </span>
      {state !== "idle" && (
        <span className="hud-timer">{formatElapsed(elapsedMs)}</span>
      )}
      {hasPending && <span className="hud-notification-dot" />}
      <ExpandIcon />
    </div>
  );
}

function PillIndicator({ state, isRecording }: { state: AgentState; isRecording?: boolean }) {
  if (isRecording) return <PillMicIcon />;
  return <span className={`hud-status-dot hud-status-dot--${state}`} />;
}

function PillMicIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 12 12" fill="none" style={{ flexShrink: 0 }}>
      <rect x="4" y="1" width="4" height="6" rx="2" fill="oklch(0.75 0.18 30)" />
      <path d="M2.5 5.5V6a3.5 3.5 0 0 0 7 0V5.5" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
      <path d="M6 9.5V11" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
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
        isRecording={session.isRecording}
        onInterrupt={session.interruptSession}
        onToggleCollapse={onCollapse}
      />
      <ActivityLog entries={session.entries} />
      <PermissionDialog
        permission={session.permission}
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
    case "listening": return "Listening...";
  }
}

function formatElapsed(ms: number): string {
  const totalSec = Math.floor(ms / 1000);
  const mins = Math.floor(totalSec / 60);
  const secs = totalSec % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}
