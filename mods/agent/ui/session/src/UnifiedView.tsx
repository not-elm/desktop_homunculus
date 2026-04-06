import { useState, useEffect, useCallback, useRef } from "react";
import { audio, dialog, Webview } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hmcs/ui";
import { useAgentSession } from "./hooks/useAgentSession";
import { useWebviewMode } from "./hooks/useWebviewMode";
import { useSettingsDraft } from "./settings/hooks/useSettingsDraft";
import type { WorkspaceSelection, AgentSettings } from "./settings/hooks/useSettingsDraft";
import { Sidebar } from "./settings/components/Sidebar";
import { SettingsFormView } from "./settings/components/SettingsFormView";
import { ActivityLog } from "./components/ActivityLog";
import { PermissionDialog } from "./components/PermissionDialog";
import { QuestionDialog } from "./components/QuestionDialog";
import { TextInput } from "./components/TextInput";
import type { SettingsCategory, BodyContent } from "./settings/types";
import type { AgentState } from "./hooks/useAgentSession";

const EXECUTOR_OPTIONS = [{ value: "codex", label: "Codex" }];

export function UnifiedView() {
  const draft = useSettingsDraft();
  const session = useAgentSession();
  const isActive = session.state !== "idle";

  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [sidebarWidth, setSidebarWidth] = useState(240);
  const [bodyContent, setBodyContent] = useState<BodyContent>({ kind: "sessionLog" });
  const [activeCategory, setActiveCategory] = useState<SettingsCategory | null>(null);
  const [prevActive, setPrevActive] = useState(false);
  const dragRef = useRef<{ startX: number; startWidth: number } | null>(null);

  const paths = draft.settings.workspaces.paths;
  const selection = draft.settings.workspaces.selection;
  const workspacePath = paths[selection.workspaceIndex] ?? null;

  // Geometry management
  const geometryMode = sidebarOpen ? "expanded" : "collapsed";
  useWebviewMode(draft.loading ? null : geometryMode);

  // Auto-collapse sidebar on session start, auto-expand on session stop
  useEffect(() => {
    if (isActive && !prevActive) {
      setSidebarOpen(false);
      setBodyContent({ kind: "sessionLog" });
      setActiveCategory(null);
    }
    if (!isActive && prevActive) {
      setSidebarOpen(true);
    }
    setPrevActive(isActive);
  }, [isActive, prevActive]);

  // Focus reporting for PTT
  useEffect(() => {
    if (!session.characterId) return;

    function reportFocus() {
      const el = document.activeElement;
      const focused =
        el instanceof HTMLElement &&
        el.matches('textarea, input, [contenteditable="true"]');
      if (focused) {
        document.documentElement.setAttribute("data-input-focus", "true");
      } else {
        document.documentElement.removeAttribute("data-input-focus");
      }
      rpc
        .call({
          modName: "@hmcs/agent",
          method: "set-text-focus",
          body: { characterId: session.characterId, focused },
        })
        .catch(() => {});
    }

    reportFocus();
    document.addEventListener("focusin", reportFocus);
    document.addEventListener("focusout", reportFocus);
    return () => {
      document.removeEventListener("focusin", reportFocus);
      document.removeEventListener("focusout", reportFocus);
      document.documentElement.removeAttribute("data-input-focus");
    };
  }, [session.characterId]);

  // Empty state when no workspaces
  useEffect(() => {
    if (!draft.loading && paths.length === 0) {
      setBodyContent({ kind: "empty" });
    }
  }, [draft.loading, paths.length]);

  function handleSidebarToggle() {
    setSidebarOpen((prev) => !prev);
  }

  function handleSelectionChange(newSelection: WorkspaceSelection) {
    if (isActive && selectionDiffers(selection, newSelection)) {
      const confirmed = window.confirm(
        "Stop current session and switch worktree?",
      );
      if (!confirmed) return;
      session.stopSession();
    }
    updateSelection(newSelection);
    setActiveCategory(null);
    setBodyContent({ kind: "sessionLog" });
  }

  function updateSelection(newSelection: WorkspaceSelection) {
    draft.setSettings({
      ...draft.settings,
      workspaces: { ...draft.settings.workspaces, selection: newSelection },
    });
  }

  function handleCategorySelect(category: SettingsCategory) {
    setActiveCategory(category);
    setBodyContent({ kind: "settingsForm", category });
  }

  function handleBack() {
    setActiveCategory(null);
    setBodyContent({ kind: "sessionLog" });
  }

  function handleAddWorkspace(path: string) {
    const newPaths = [...paths, path];
    const newIndex = newPaths.length - 1;
    draft.setSettings({
      ...draft.settings,
      workspaces: {
        paths: newPaths,
        selection: { workspaceIndex: newIndex, worktreeName: null },
      },
    });
    setActiveCategory(null);
    setBodyContent({ kind: "sessionLog" });
  }

  const handleAddWorkspaceFromPanel = useCallback(async () => {
    try {
      const path = await dialog.pickFolder({
        title: "Select workspace directory",
      });
      if (!path) return;
      handleAddWorkspace(path);
    } catch (e) {
      console.error("pickFolder failed:", e);
    }
  }, [paths, draft]);

  function handleRemoveWorkspace(index: number) {
    const newPaths = paths.filter((_, i) => i !== index);
    const sel = selection;
    const newSelection =
      sel.workspaceIndex >= newPaths.length
        ? { workspaceIndex: Math.max(0, newPaths.length - 1), worktreeName: null }
        : sel.workspaceIndex > index
          ? { ...sel, workspaceIndex: sel.workspaceIndex - 1 }
          : sel;
    draft.setSettings({
      ...draft.settings,
      workspaces: { paths: newPaths, selection: newSelection },
    });
    if (newPaths.length === 0) {
      setBodyContent({ kind: "empty" });
      setActiveCategory(null);
    }
  }

  function handleExecutorChange(value: string) {
    const newSettings = {
      ...draft.settings,
      executor: value as AgentSettings["executor"],
    };
    void draft.autoSave(newSettings);
  }

  async function handleClose() {
    await audio.se.play("se:close");
    await Webview.current()?.close();
  }

  function handleResizeStart(e: React.MouseEvent) {
    e.preventDefault();
    dragRef.current = { startX: e.clientX, startWidth: sidebarWidth };
    document.body.style.userSelect = "none";

    function onMouseMove(ev: MouseEvent) {
      if (!dragRef.current) return;
      const delta = ev.clientX - dragRef.current.startX;
      const newWidth = Math.max(160, Math.min(320, dragRef.current.startWidth + delta));
      setSidebarWidth(newWidth);
    }

    function onMouseUp() {
      dragRef.current = null;
      document.body.style.userSelect = "";
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
    }

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
  }

  if (draft.loading) return null;

  return (
    <div
      className="stg-chrome"
      data-sidebar={sidebarOpen ? "open" : "closed"}
      style={{ width: sidebarOpen ? 640 : 640 - sidebarWidth }}
    >
      <UnifiedHeader
        state={session.state}
        isActive={isActive}
        elapsedMs={session.elapsedMs}
        isRecording={session.isRecording}
        worktreeInfo={session.worktreeInfo}
        executor={draft.settings.executor}
        onExecutorChange={handleExecutorChange}
        onToggleSidebar={handleSidebarToggle}
        onToggleSession={isActive ? session.stopSession : session.startSession}
        onInterrupt={session.interruptSession}
        onClose={handleClose}
      />
      <div className="uv-body">
        <div
          className="uv-sidebar"
          inert={!sidebarOpen || undefined}
          style={sidebarOpen ? { width: sidebarWidth } : undefined}
        >
          <Sidebar
            paths={paths}
            selection={selection}
            onSelectionChange={handleSelectionChange}
            onAddWorkspace={handleAddWorkspace}
            onRemoveWorkspace={handleRemoveWorkspace}
            activeCategory={activeCategory}
            onCategorySelect={handleCategorySelect}
            refreshKey={0}
          />
        </div>
        <div className="uv-resize-handle" onMouseDown={handleResizeStart} />
        <div className="uv-main">
          <BodyPanel
            content={bodyContent}
            session={session}
            draft={draft}
            isActive={isActive}
            onBack={handleBack}
            onAddWorkspace={handleAddWorkspaceFromPanel}
          />
          {draft.isDirty && (
            <div className="stg-save-bar">
              <button
                className="stg-action-btn stg-action-btn--primary"
                type="button"
                onClick={draft.saveSettings}
                disabled={draft.saving}
              >
                {draft.saving ? "Saving..." : "Save Changes"}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function BodyPanel({
  content,
  session,
  draft,
  isActive,
  onBack,
  onAddWorkspace,
}: {
  content: BodyContent;
  session: ReturnType<typeof useAgentSession>;
  draft: ReturnType<typeof useSettingsDraft>;
  isActive: boolean;
  onBack: () => void;
  onAddWorkspace: () => void;
}) {
  if (content.kind === "empty") {
    return (
      <div className="stg-empty">
        <span className="stg-empty-text">No workspace configured</span>
        <button
          className="stg-action-btn stg-action-btn--primary"
          type="button"
          onClick={onAddWorkspace}
        >
          + Add Workspace
        </button>
      </div>
    );
  }

  if (content.kind === "settingsForm") {
    return (
      <div className="uv-form-wrapper">
        <div className="uv-form-subheader">
          <button className="uv-back-btn" type="button" onClick={onBack}>
            ←
          </button>
          <span className="uv-form-title">
            {content.category === "phrases" ? "Phrases" : "Permissions"}
          </span>
        </div>
        <div className="uv-form-scroll">
          <SettingsFormView
            category={content.category}
            settings={draft.settings}
            onSettingsChange={draft.setSettings}
          />
        </div>
      </div>
    );
  }

  // sessionLog
  const hasDialog = session.permission !== null || session.question !== null;
  return (
    <div className="uv-session">
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
      {!hasDialog && <TextInput onSend={session.sendMessage} />}
    </div>
  );
}

interface UnifiedHeaderProps {
  state: AgentState;
  isActive: boolean;
  elapsedMs: number;
  isRecording?: boolean;
  worktreeInfo: { name: string; branch: string } | null;
  executor: string;
  onExecutorChange: (value: string) => void;
  onToggleSidebar: () => void;
  onToggleSession: () => void;
  onInterrupt: () => void;
  onClose: () => void;
}

function UnifiedHeader({
  state,
  isActive,
  elapsedMs,
  isRecording,
  worktreeInfo,
  executor,
  onExecutorChange,
  onToggleSidebar,
  onToggleSession,
  onInterrupt,
  onClose,
}: UnifiedHeaderProps) {
  const interruptible = isActive && (state === "thinking" || state === "executing");

  return (
    <div className="uv-header">
      <button
        className="uv-hamburger"
        type="button"
        onClick={onToggleSidebar}
        title="Toggle sidebar"
      >
        <HamburgerIcon />
      </button>
      <span className="uv-title">Agent</span>
      {isRecording ? (
        <RecordingIndicator />
      ) : (
        <span className={`hud-status-dot hud-status-dot--${state}`} />
      )}
      <span
        className={`uv-status-label uv-status-label--${isRecording ? "listening" : state}`}
      >
        {isRecording ? "Listening..." : stateLabel(state)}
      </span>
      {isActive && (
        <span className="uv-timer">{formatElapsed(elapsedMs)}</span>
      )}
      {worktreeInfo && (
        <span className="agent-badge agent-badge--violet uv-worktree-badge">
          {worktreeInfo.branch}
        </span>
      )}
      <div className="uv-header-spacer" />
      <div className="uv-executor-select">
        <Select value={executor} onValueChange={onExecutorChange}>
          <SelectTrigger className="uv-executor-trigger">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {EXECUTOR_OPTIONS.map((o) => (
              <SelectItem key={o.value} value={o.value}>
                {o.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      {interruptible && (
        <button
          className="hud-interrupt-btn"
          onClick={onInterrupt}
          title="Interrupt"
        >
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
      <button
        className="hud-icon-btn hud-icon-btn--close"
        onClick={onClose}
        title="Close"
      >
        <CloseIcon />
      </button>
    </div>
  );
}

function selectionDiffers(
  a: WorkspaceSelection,
  b: WorkspaceSelection,
): boolean {
  return (
    a.workspaceIndex !== b.workspaceIndex ||
    a.worktreeName !== b.worktreeName
  );
}

function stateLabel(state: AgentState): string {
  switch (state) {
    case "idle":
      return "Standby";
    case "thinking":
      return "Thinking";
    case "executing":
      return "Working";
    case "waiting":
      return "Waiting";
    case "listening":
      return "Listening...";
  }
}

function formatElapsed(ms: number): string {
  const totalSec = Math.floor(ms / 1000);
  const mins = Math.floor(totalSec / 60);
  const secs = totalSec % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}

function HamburgerIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
      <path
        d="M2 4h10M2 7h10M2 10h10"
        stroke="currentColor"
        strokeWidth="1.4"
        strokeLinecap="round"
      />
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
      <path
        d="M3 3L9 9M9 3L3 9"
        stroke="currentColor"
        strokeWidth="1.4"
        strokeLinecap="round"
      />
    </svg>
  );
}

function RecordingIndicator() {
  return (
    <span className="hud-recording-indicator">
      <svg width="14" height="14" viewBox="0 0 12 12" fill="none">
        <rect x="4" y="1" width="4" height="6" rx="2" fill="oklch(0.8 0.18 30)" />
        <path
          d="M2.5 5.5V6a3.5 3.5 0 0 0 7 0V5.5"
          stroke="oklch(0.8 0.18 30)"
          strokeWidth="1.1"
          strokeLinecap="round"
        />
        <path
          d="M6 9.5V11"
          stroke="oklch(0.8 0.18 30)"
          strokeWidth="1.1"
          strokeLinecap="round"
        />
      </svg>
      <span className="hud-staggered-dots">
        <span className="hud-staggered-dot" style={{ animationDelay: "0s" }} />
        <span
          className="hud-staggered-dot"
          style={{ animationDelay: "0.2s" }}
        />
        <span
          className="hud-staggered-dot"
          style={{ animationDelay: "0.4s" }}
        />
      </span>
    </span>
  );
}
