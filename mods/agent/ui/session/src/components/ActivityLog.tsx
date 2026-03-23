import { useEffect, useRef } from "react";
import type { LogEntry, LogType } from "../hooks/useAgentSession";

interface ActivityLogProps {
  entries: LogEntry[];
}

export function ActivityLog({ entries }: ActivityLogProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [entries]);

  return (
    <div className="hud-log">
      {entries.length === 0 ? (
        <div className="hud-log-empty">Awaiting activity</div>
      ) : (
        entries.map((entry) => <LogRow key={entry.id} entry={entry} />)
      )}
      <div ref={bottomRef} />
    </div>
  );
}

function LogRow({ entry }: { entry: LogEntry }) {
  return (
    <div className="hud-log-entry">
      <span className="hud-log-icon">
        <LogIcon type={entry.type} />
      </span>
      <span className={`hud-log-text hud-log-text--${entry.type === "assistant" ? "assistant" : entry.type === "done" ? "done" : entry.type === "error" ? "error" : ""}`}>
        {entry.message}
      </span>
      <span className="hud-log-ts">{formatTime(entry.timestamp)}</span>
    </div>
  );
}

function LogIcon({ type }: { type: LogType }) {
  switch (type) {
    case "read":
      return <ReadIcon />;
    case "edit":
      return <EditIcon />;
    case "run":
      return <RunIcon />;
    case "tool":
      return <ToolIcon />;
    case "assistant":
      return <DiamondIcon />;
    case "done":
      return <DoneIcon />;
    case "error":
      return <ErrorIcon />;
    case "warning":
      return <WarningIcon />;
    default:
      return <DotIcon />;
  }
}

function ReadIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M1 2h8M1 5h5M1 8h3" stroke="oklch(0.72 0.18 192)" strokeWidth="1.2" strokeLinecap="round" />
    </svg>
  );
}

function EditIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M7 1L9 3L3 9H1V7L7 1Z" stroke="oklch(0.78 0.16 75)" strokeWidth="1.1" strokeLinejoin="round" />
    </svg>
  );
}

function RunIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M2 1.5L8 5L2 8.5V1.5Z" fill="oklch(0.78 0.16 75)" />
    </svg>
  );
}

function ToolIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <circle cx="5" cy="5" r="3.5" stroke="oklch(0.72 0.06 250 / 0.6)" strokeWidth="1.1" />
      <path d="M5 3v2.5M5 7v.3" stroke="oklch(0.72 0.06 250 / 0.6)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
}

function DiamondIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M5 1L9 5L5 9L1 5L5 1Z" stroke="oklch(0.72 0.18 192)" strokeWidth="1.1" />
    </svg>
  );
}

function DoneIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M1.5 5L4 7.5L8.5 2.5" stroke="oklch(0.65 0.18 145)" strokeWidth="1.3" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function ErrorIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M5 1L9.5 9H.5L5 1Z" stroke="oklch(0.65 0.2 20)" strokeWidth="1.1" strokeLinejoin="round" />
      <path d="M5 4.5V6.5M5 7.5V8" stroke="oklch(0.65 0.2 20)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
}

function WarningIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M5 2L8.5 8H1.5L5 2Z" stroke="oklch(0.78 0.16 75)" strokeWidth="1.1" strokeLinejoin="round" />
      <path d="M5 4.5V6M5 7V7.5" stroke="oklch(0.78 0.16 75)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
}

function DotIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <circle cx="5" cy="5" r="2" fill="oklch(0.55 0.02 250 / 0.5)" />
    </svg>
  );
}

function formatTime(timestamp: number): string {
  const d = new Date(timestamp);
  const h = String(d.getHours()).padStart(2, "0");
  const m = String(d.getMinutes()).padStart(2, "0");
  const s = String(d.getSeconds()).padStart(2, "0");
  return `${h}:${m}:${s}`;
}
