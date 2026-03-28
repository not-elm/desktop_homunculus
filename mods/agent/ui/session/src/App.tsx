import { useState } from "react";
import { useAgentSession } from "./hooks/useAgentSession";
import { useAgentSettings } from "./hooks/useAgentSettings";
import { ActivityLog } from "./components/ActivityLog";
import { PermissionDialog } from "./components/PermissionDialog";
import { QuestionDialog } from "./components/QuestionDialog";
import { InlineSettingsBar } from "./components/InlineSettingsBar";
import { SettingsView } from "./components/SettingsView";
import type { AgentState } from "./hooks/useAgentSession";

type View = "session" | "settings";

export function App() {
  const [collapsed, setCollapsed] = useState(false);
  const [view, setView] = useState<View>("session");
  const session = useAgentSession();
  const settingsHook = useAgentSettings();

  if (collapsed) {
    return (
      <CollapsedPill
        state={session.state}
        elapsedMs={session.elapsedMs}
        hasPending={session.hasPending}
        isRecording={session.isRecording}
        onExpand={() => setCollapsed(false)}
      />
    );
  }

  return (
    <AgentPanel
      session={session}
      settingsHook={settingsHook}
      view={view}
      onViewChange={setView}
      onCollapse={() => setCollapsed(true)}
    />
  );
}

/* ━━ Collapsed Pill ━━ */

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
      {state !== "idle" && <span className="hud-timer">{formatElapsed(elapsedMs)}</span>}
      {hasPending && <span className="hud-notification-dot" />}
      <ExpandArrow />
    </div>
  );
}

/* ━━ Agent Panel (Expanded) ━━ */

interface AgentPanelProps {
  session: ReturnType<typeof useAgentSession>;
  settingsHook: ReturnType<typeof useAgentSettings>;
  view: View;
  onViewChange: (v: View) => void;
  onCollapse: () => void;
}

function AgentPanel({ session, settingsHook, view, onViewChange, onCollapse }: AgentPanelProps) {
  const isActive = session.state !== "idle";
  const showBack = view === "settings";

  function toggleView() {
    onViewChange(view === "session" ? "settings" : "session");
  }

  return (
    <div className="hud-panel">
      <HudDecorations />
      <PanelHeader
        state={session.state}
        elapsedMs={session.elapsedMs}
        isRecording={session.isRecording}
        isActive={isActive}
        showBack={showBack}
        onToggleSession={isActive ? session.stopSession : session.startSession}
        onInterrupt={session.interruptSession}
        onToggleView={toggleView}
        onBack={() => onViewChange("session")}
        onCollapse={onCollapse}
        onClose={session.closePanel}
      />
      <InlineSettingsBar
        settings={settingsHook.settings}
        onSettingsChange={settingsHook.setSettings}
        apiKey={settingsHook.apiKey}
      />
      <div className="hud-view-slider" data-view={view}>
        <div className="hud-view-slide">
          <SessionContent session={session} />
        </div>
        <div className="hud-view-slide">
          <SettingsView
            settings={settingsHook.settings}
            onSettingsChange={settingsHook.setSettings}
            saving={settingsHook.saving}
            onSave={settingsHook.saveSettings}
            apiKey={settingsHook.apiKey}
            onApiKeyChange={settingsHook.setApiKey}
            savingApiKey={settingsHook.savingApiKey}
            onApiKeySave={settingsHook.saveApiKey}
          />
        </div>
      </div>
    </div>
  );
}

function SessionContent({ session }: { session: ReturnType<typeof useAgentSession> }) {
  return (
    <>
      {session.error && <div className="hud-error">{session.error}</div>}
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
    </>
  );
}

/* ━━ Panel Header ━━ */

interface PanelHeaderProps {
  state: AgentState;
  elapsedMs: number;
  isRecording?: boolean;
  isActive: boolean;
  showBack: boolean;
  onToggleSession: () => void;
  onInterrupt: () => void;
  onToggleView: () => void;
  onBack: () => void;
  onCollapse: () => void;
  onClose: () => void;
}

function PanelHeader({
  state, elapsedMs, isRecording, isActive, showBack,
  onToggleSession, onInterrupt, onToggleView, onBack, onCollapse, onClose,
}: PanelHeaderProps) {
  const interruptible = isActive && (state === "thinking" || state === "executing");

  return (
    <div className="hud-header">
      <button
        className={`hud-session-toggle${isActive ? " hud-session-toggle--active" : ""}`}
        onClick={onToggleSession}
        title={isActive ? "Stop Session" : "Start Session"}
      >
        {isActive ? <StopSquare /> : <PlayTriangle />}
      </button>

      {showBack && (
        <button className="hud-icon-btn" onClick={onBack} title="Back">
          <BackArrow />
        </button>
      )}

      {isRecording ? <RecordingIndicator /> : <span className={`hud-status-dot hud-status-dot--${state}`} />}
      <span className={`hud-status-label hud-status-label--${isRecording ? "listening" : state}`}>
        {isRecording ? "Listening..." : stateLabel(state)}
      </span>
      {isActive && <span className="hud-timer">{formatElapsed(elapsedMs)}</span>}

      <div className="hud-header-spacer" />

      {interruptible && (
        <button className="hud-interrupt-btn" onClick={onInterrupt} title="Interrupt">
          <InterruptIcon />
        </button>
      )}
      <button className="hud-icon-btn" onClick={onToggleView} title="Settings">
        <GearIcon />
      </button>
      <button className="hud-icon-btn" onClick={onCollapse} title="Minimize">
        <CollapseChevron />
      </button>
      <button className="hud-icon-btn hud-icon-btn--close" onClick={onClose} title="Close">
        <CloseIcon />
      </button>
    </div>
  );
}

/* ━━ Decorations ━━ */

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

function RecordingIndicator() {
  return (
    <span className="hud-recording-indicator">
      <MicIcon />
      <span className="hud-staggered-dots">
        <span className="hud-staggered-dot" style={{ animationDelay: "0s" }} />
        <span className="hud-staggered-dot" style={{ animationDelay: "0.2s" }} />
        <span className="hud-staggered-dot" style={{ animationDelay: "0.4s" }} />
      </span>
    </span>
  );
}

/* ━━ Icons ━━ */

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

function MicIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <rect x="4" y="1" width="4" height="6" rx="2" fill="oklch(0.75 0.18 30)" />
      <path d="M2.5 5.5V6a3.5 3.5 0 0 0 7 0V5.5" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
      <path d="M6 9.5V11" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
}

function ExpandArrow() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none" style={{ flexShrink: 0 }}>
      <path d="M2 3L5 6.5L8 3" stroke="currentColor" strokeWidth="1.3" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function PlayTriangle() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <path d="M3 2L10 6L3 10V2Z" fill="currentColor" />
    </svg>
  );
}

function StopSquare() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <rect x="2.5" y="2.5" width="7" height="7" rx="1" fill="currentColor" />
    </svg>
  );
}

function InterruptIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <rect x="2" y="2" width="6" height="6" rx="1" fill="currentColor" />
    </svg>
  );
}

function BackArrow() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <path d="M7 3L4 6L7 9" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function CollapseChevron() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <path d="M3 8L6 5L9 8" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function CloseIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <path d="M3 3L9 9M9 3L3 9" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" />
    </svg>
  );
}

function GearIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <circle cx="6" cy="6" r="2" stroke="currentColor" strokeWidth="1.2" />
      <path d="M6 1v1.5M6 9.5V11M1 6h1.5M9.5 6H11M2.4 2.4l1.1 1.1M8.5 8.5l1.1 1.1M9.6 2.4L8.5 3.5M3.5 8.5L2.4 9.6" stroke="currentColor" strokeWidth="1" strokeLinecap="round" />
    </svg>
  );
}

/* ━━ Helpers ━━ */

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
