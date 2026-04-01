import type { AgentState } from "../hooks/useAgentSession";

interface StatusBarProps {
  state: AgentState;
  elapsedMs: number;
  isRecording?: boolean;
  onInterrupt?: () => void;
  onToggleCollapse: () => void;
}

const STATE_LABELS: Record<AgentState, string> = {
  idle: "Standby",
  thinking: "Thinking",
  executing: "Working",
  waiting: "Waiting",
  listening: "Listening",
};

export function StatusBar({ state, elapsedMs, isRecording, onInterrupt, onToggleCollapse }: StatusBarProps) {
  const interruptible = state === "thinking" || state === "executing";

  return (
    <div className="hud-statusbar">
      <StatusIndicator state={state} isRecording={isRecording} />
      <StatusLabel state={state} isRecording={isRecording} />
      {interruptible && onInterrupt && <InterruptButton onClick={onInterrupt} />}
      <div className="hud-statusbar-spacer" />
      <ElapsedTimer elapsedMs={elapsedMs} active={state !== "idle"} />
      <button className="hud-toggle-btn" onClick={onToggleCollapse} title="Collapse">
        <CollapseIcon />
      </button>
    </div>
  );
}

function StatusIndicator({ state, isRecording }: { state: AgentState; isRecording?: boolean }) {
  if (isRecording) return <RecordingIndicator />;
  return <span className={`hud-status-dot hud-status-dot--${state}`} />;
}

function RecordingIndicator() {
  return (
    <span className="hud-recording-indicator">
      <MicIcon />
      <StaggeredDots />
    </span>
  );
}

function InterruptButton({ onClick }: { onClick: () => void }) {
  return (
    <button className="hud-interrupt-btn" onClick={onClick} title="Interrupt">
      <StopIcon />
    </button>
  );
}

function StopIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="2" y="2" width="6" height="6" rx="1" fill="currentColor" />
    </svg>
  );
}

function StatusLabel({ state, isRecording }: { state: AgentState; isRecording?: boolean }) {
  if (isRecording) {
    return <span className="hud-status-label hud-status-label--listening">Listening...</span>;
  }
  return (
    <span className={`hud-status-label hud-status-label--${state}`}>
      {STATE_LABELS[state]}
    </span>
  );
}

function StaggeredDots() {
  return (
    <span className="hud-staggered-dots">
      <span className="hud-staggered-dot" style={{ animationDelay: "0s" }} />
      <span className="hud-staggered-dot" style={{ animationDelay: "0.2s" }} />
      <span className="hud-staggered-dot" style={{ animationDelay: "0.4s" }} />
    </span>
  );
}

function MicIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="4" y="1" width="4" height="6" rx="2" fill="oklch(0.75 0.18 30)" />
      <path d="M2.5 5.5V6a3.5 3.5 0 0 0 7 0V5.5" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
      <path d="M6 9.5V11" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
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
