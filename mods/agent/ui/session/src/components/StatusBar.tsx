import type { AgentState } from "../hooks/useAgentSession";

interface StatusBarProps {
  state: AgentState;
  elapsedMs: number;
  onToggleCollapse: () => void;
}

const STATE_LABELS: Record<AgentState, string> = {
  idle: "Standby",
  thinking: "Thinking",
  executing: "Working",
  waiting: "Waiting",
};

export function StatusBar({ state, elapsedMs, onToggleCollapse }: StatusBarProps) {
  return (
    <div className="hud-statusbar">
      <StatusDot state={state} />
      <span className={`hud-status-label hud-status-label--${state}`}>
        {STATE_LABELS[state]}
      </span>
      <div className="hud-statusbar-spacer" />
      <ElapsedTimer elapsedMs={elapsedMs} active={state !== "idle"} />
      <button className="hud-toggle-btn" onClick={onToggleCollapse} title="Collapse">
        <CollapseIcon />
      </button>
    </div>
  );
}

function StatusDot({ state }: { state: AgentState }) {
  return <span className={`hud-status-dot hud-status-dot--${state}`} />;
}

function ElapsedTimer({ elapsedMs, active }: { elapsedMs: number; active: boolean }) {
  if (!active) return null;
  return <span className="hud-timer">{formatElapsed(elapsedMs)}</span>;
}

function CollapseIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M2 8L6 4L10 8" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function formatElapsed(ms: number): string {
  const totalSec = Math.floor(ms / 1000);
  const mins = Math.floor(totalSec / 60);
  const secs = totalSec % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}
