import { useState, useEffect } from "react";
import { audio, Webview, webviewSource } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { useAgentSession } from "./hooks/useAgentSession";
import { useAgentSettings } from "./hooks/useAgentSettings";
import { ActivityLog } from "./components/ActivityLog";
import { PermissionDialog } from "./components/PermissionDialog";
import { QuestionDialog } from "./components/QuestionDialog";
import { TextInput } from "./components/TextInput";
import { InlineSettingsBar } from "./components/InlineSettingsBar";
import type { AgentState } from "./hooks/useAgentSession";

export function App() {
  const [expanded, setExpanded] = useState(true);
  const [mounted, setMounted] = useState(false);
  const session = useAgentSession({ autoStart: true });
  const settingsHook = useAgentSettings();
  const isActive = session.state !== "idle";

  useEffect(() => {
    requestAnimationFrame(() => setMounted(true));
  }, []);

  useEffect(() => {
    if (!session.characterId) return;

    function reportFocus() {
      const el = document.activeElement;
      const focused = el instanceof HTMLElement && el.matches('textarea, input, [contenteditable="true"]');
      rpc.call({
        modName: "@hmcs/agent",
        method: "set-text-focus",
        body: { characterId: session.characterId, focused },
      }).catch(() => {
        // fail-open; next focus change will resync state
      });
    }

    reportFocus();

    document.addEventListener("focusin", reportFocus);
    document.addEventListener("focusout", reportFocus);
    return () => {
      document.removeEventListener("focusin", reportFocus);
      document.removeEventListener("focusout", reportFocus);
    };
  }, [session.characterId]);

  async function openSettingsWindow() {
    const vrm = await Webview.current()?.linkedVrm();
    await Webview.open({
      source: webviewSource.local("agent:settings-ui"),
      size: [1.3333, 1.0],
      viewportSize: [1200, 900],
      linkedVrm: vrm?.entity,
      offset: [-0.5, -0.3, 12.0],
    });
    await audio.se.play("se:open");
  }

  return (
    <div className={`hud-container${mounted ? " hud-container--ready" : ""}`} data-expanded={expanded}>
      {!expanded ? (
        <CollapsedIcon
          state={session.state}
          hasPending={session.hasPending}
          onClick={() => setExpanded(true)}
        />
      ) : (
        <>
          <HudDecorations />
          <PanelHeader
            state={session.state}
            elapsedMs={session.elapsedMs}
            isRecording={session.isRecording}
            isActive={isActive}
            expanded={expanded}
            hasPending={session.hasPending}
            worktreeInfo={session.worktreeInfo}
            onToggleSession={isActive ? session.stopSession : session.startSession}
            onInterrupt={session.interruptSession}
            onOpenSettings={openSettingsWindow}
            onToggleExpand={() => setExpanded(false)}
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
                    worktreeActive={session.worktreeInfo !== null}
                  />
              }
              <div className="hud-view-slider" data-view="session">
                <div className="hud-view-slide">
                  <SessionContent session={session} />
                </div>
              </div>
            </div>
          </div>
        </>
      )}
    </div>
  );
}

function SessionContent({ session }: { session: ReturnType<typeof useAgentSession> }) {
  const isActive = session.state !== "idle";
  const hasDialog = session.permission !== null || session.question !== null;

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
      {!hasDialog && (
        <TextInput onSend={session.sendMessage} />
      )}
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
  hasPending: boolean;
  worktreeInfo: { name: string; branch: string } | null;
  onToggleSession: () => void;
  onInterrupt: () => void;
  onOpenSettings: () => void;
  onToggleExpand: () => void;
  onClose: () => void;
}

function PanelHeader({
  state, elapsedMs, isRecording, isActive, expanded, hasPending, worktreeInfo,
  onToggleSession, onInterrupt, onOpenSettings, onToggleExpand, onClose,
}: PanelHeaderProps) {
  const interruptible = isActive && (state === "thinking" || state === "executing");

  return (
    <div className="hud-header">
      {isRecording ? <RecordingIndicator /> : <span className={`hud-status-dot hud-status-dot--${state}`} />}
      <span className={`hud-status-label hud-status-label--${isRecording ? "listening" : state}`}>
        {isRecording ? "Listening..." : stateLabel(state)}
      </span>
      {isActive && <span className="hud-timer">{formatElapsed(elapsedMs)}</span>}
      {worktreeInfo && (
        <span className="agent-badge agent-badge--violet" style={{ fontSize: "0.68rem", fontFamily: "monospace" }}>
          {worktreeInfo.branch}
        </span>
      )}
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
        <button className="hud-icon-btn" onClick={onOpenSettings} title="Settings">
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

/* ━━ Collapsed Icon ━━ */

interface CollapsedIconProps {
  state: AgentState;
  hasPending: boolean;
  onClick: () => void;
}

function CollapsedIcon({ state, hasPending, onClick }: CollapsedIconProps) {
  return (
    <div className="hud-collapsed-icon" onClick={onClick} role="button" tabIndex={0} aria-label="Expand session panel">
      <span className={`hud-collapsed-dot hud-collapsed-dot--${state}`} />
      {hasPending && <span className="hud-collapsed-badge" />}
    </div>
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
