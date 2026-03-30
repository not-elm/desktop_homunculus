import { useState, useEffect } from "react";
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
  const [expanded, setExpanded] = useState(true);
  const [mounted, setMounted] = useState(false);
  const [view, setView] = useState<View>("session");
  const session = useAgentSession();
  const settingsHook = useAgentSettings();
  const isActive = session.state !== "idle";
  const showBack = view === "settings";

  useEffect(() => {
    requestAnimationFrame(() => setMounted(true));
  }, []);

  return (
    <div className={`hud-container${mounted ? " hud-container--ready" : ""}`} data-expanded={expanded}>
      <HudDecorations />
      <PanelHeader
        state={session.state}
        elapsedMs={session.elapsedMs}
        isRecording={session.isRecording}
        isActive={isActive}
        expanded={expanded}
        showBack={showBack}
        hasPending={session.hasPending}
        onToggleSession={isActive ? session.stopSession : session.startSession}
        onInterrupt={session.interruptSession}
        onToggleView={() => setView(view === "session" ? "settings" : "session")}
        onBack={() => setView("session")}
        onToggleExpand={() => setExpanded(!expanded)}
        onClose={session.closePanel}
      />
      <div className="hud-body">
        <div className="hud-body-inner">
          {settingsHook.loading
            ? <div className="hud-inline-settings" style={{ visibility: "hidden" }} />
            : <InlineSettingsBar
                settings={settingsHook.settings}
                onSettingsChange={settingsHook.setAndSaveSettings}
                apiKey={settingsHook.apiKey}
              />
          }
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
  expanded: boolean;
  showBack: boolean;
  hasPending: boolean;
  onToggleSession: () => void;
  onInterrupt: () => void;
  onToggleView: () => void;
  onBack: () => void;
  onToggleExpand: () => void;
  onClose: () => void;
}

function PanelHeader({
  state, elapsedMs, isRecording, isActive, expanded, showBack, hasPending,
  onToggleSession, onInterrupt, onToggleView, onBack, onToggleExpand, onClose,
}: PanelHeaderProps) {
  const interruptible = isActive && (state === "thinking" || state === "executing");

  return (
    <div className="hud-header">
      {showBack && expanded && (
        <button className="hud-icon-btn" onClick={onBack} title="Back">
          <BackArrow />
        </button>
      )}

      {isRecording ? <RecordingIndicator /> : <span className={`hud-status-dot hud-status-dot--${state}`} />}
      <span className={`hud-status-label hud-status-label--${isRecording ? "listening" : state}`}>
        {isRecording ? "Listening..." : stateLabel(state)}
      </span>
      {isActive && <span className="hud-timer">{formatElapsed(elapsedMs)}</span>}
      {!expanded && hasPending && <span className="hud-notification-dot" />}

      <div className="hud-header-spacer" />

      {expanded && interruptible && (
        <button className="hud-interrupt-btn" onClick={onInterrupt} title="Interrupt">
          <InterruptIcon />
        </button>
      )}
      <button
        className={`hud-session-toggle${isActive ? " hud-session-toggle--active" : ""}`}
        onClick={onToggleSession}
        title={isActive ? "Stop Session" : "Start Session"}
      >
        {isActive ? <StopSquare /> : <PlayTriangle />}
      </button>
      {expanded && (
        <button className="hud-icon-btn" onClick={onToggleView} title="Settings">
          <GearIcon />
        </button>
      )}
      <button className="hud-icon-btn" onClick={onToggleExpand} title={expanded ? "Minimize" : "Expand"}>
        <ExpandCollapseChevron expanded={expanded} />
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

function MicIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 12 12" fill="none">
      <rect x="4" y="1" width="4" height="6" rx="2" fill="oklch(0.8 0.18 30)" />
      <path d="M2.5 5.5V6a3.5 3.5 0 0 0 7 0V5.5" stroke="oklch(0.8 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
      <path d="M6 9.5V11" stroke="oklch(0.8 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
}

function ExpandCollapseChevron({ expanded }: { expanded: boolean }) {
  return (
    <svg
      width="12" height="12" viewBox="0 0 12 12" fill="none"
      style={{ transition: "transform 250ms ease", transform: expanded ? "rotate(0)" : "rotate(180deg)" }}
    >
      <path d="M3 8L6 5L9 8" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round" />
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
