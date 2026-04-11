import { useEffect, useRef } from 'react';
import type { WorkerLogEntry, WorkerState } from '../hooks/useWorkers';
import { PermissionPanel } from './PermissionPanel';

interface WorkerLogProps {
  worker: WorkerState | undefined;
}

export function WorkerLog({ worker }: WorkerLogProps) {
  if (!worker) {
    return <Placeholder />;
  }

  return (
    <div className="flex flex-1 flex-col overflow-hidden">
      <LogHeader worker={worker} />
      <LogBody worker={worker} />
    </div>
  );
}

function Placeholder() {
  return (
    <div className="flex flex-1 items-center justify-center">
      <span className="text-[var(--hud-font-size-xs)] text-[var(--hud-text-muted)]">
        Select a Worker
      </span>
    </div>
  );
}

function LogHeader({ worker }: { worker: WorkerState }) {
  return (
    <div className="flex items-center gap-2 border-b border-[var(--hud-border-decorative)] px-3 py-1.5">
      <span className="truncate text-[var(--hud-font-size-sm)] font-medium text-[var(--hud-text-primary)]">
        {worker.description}
      </span>
      <StatusLabel status={worker.status} />
    </div>
  );
}

function StatusLabel({ status }: { status: WorkerState['status'] }) {
  const { label, color } = statusMeta(status);
  return (
    <span
      className="shrink-0 rounded px-1.5 py-0.5 text-[10px] font-medium"
      style={{ backgroundColor: `${color}22`, color }}
    >
      {label}
    </span>
  );
}

function statusMeta(status: WorkerState['status']): { label: string; color: string } {
  switch (status) {
    case 'running':
      return { label: 'RUNNING', color: 'oklch(0.72 0.14 192)' };
    case 'completed':
      return { label: 'DONE', color: 'oklch(0.65 0.18 145)' };
    case 'failed':
      return { label: 'FAILED', color: 'oklch(0.65 0.2 20)' };
    case 'cancelled':
      return { label: 'CANCELLED', color: 'oklch(0.55 0.02 250)' };
  }
}

function LogBody({ worker }: { worker: WorkerState }) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  return (
    <div className="flex-1 overflow-y-auto px-2 py-1.5">
      {worker.log.map((entry) => (
        <LogEntryRow key={`${entry.type}-${entry.timestamp}`} entry={entry} />
      ))}
      {worker.pendingPermission && <PermissionPanel permission={worker.pendingPermission} />}
      {isThinking(worker) && <ThinkingIndicator />}
      <div ref={bottomRef} />
    </div>
  );
}

function isThinking(worker: WorkerState): boolean {
  return worker.status === 'running' && worker.pendingPermission === null;
}

function LogEntryRow({ entry }: { entry: WorkerLogEntry }) {
  return (
    <div className="flex items-start gap-2 py-0.5">
      <span className="mt-0.5 shrink-0 text-[11px]">
        <LogIcon type={entry.type} />
      </span>
      <span className="min-w-0 flex-1 text-[var(--hud-font-size-xs)] leading-[var(--hud-line-height-normal)] text-[var(--hud-text-secondary)]">
        {entry.summary ?? entry.text ?? entry.type}
      </span>
      <span className="shrink-0 text-[10px] text-[var(--hud-text-muted)]">
        {formatTime(entry.timestamp)}
      </span>
    </div>
  );
}

function LogIcon({ type }: { type: string }) {
  switch (type) {
    case 'tool_use':
      return <span style={{ color: 'oklch(0.72 0.14 192)' }}>&#x2699;</span>;
    case 'assistant_message':
      return <span style={{ color: 'oklch(0.72 0.14 192)' }}>&#x25C7;</span>;
    case 'permission':
      return <span style={{ color: 'oklch(0.65 0.2 20)' }}>&#x26A0;</span>;
    case 'completed':
      return <span style={{ color: 'oklch(0.65 0.18 145)' }}>&#x2713;</span>;
    case 'error':
      return <span style={{ color: 'oklch(0.65 0.2 20)' }}>&#x2717;</span>;
    default:
      return <span style={{ color: 'oklch(0.55 0.02 250)' }}>&#x2022;</span>;
  }
}

function ThinkingIndicator() {
  return (
    <div className="flex items-center gap-1.5 py-1">
      <span className="inline-block size-1.5 animate-pulse rounded-full bg-[oklch(0.72_0.14_192)]" />
      <span className="animate-pulse text-[var(--hud-font-size-xs)] text-[var(--hud-text-muted)]">
        Thinking...
      </span>
    </div>
  );
}

function formatTime(timestamp: number): string {
  const d = new Date(timestamp);
  const h = String(d.getHours()).padStart(2, '0');
  const m = String(d.getMinutes()).padStart(2, '0');
  const s = String(d.getSeconds()).padStart(2, '0');
  return `${h}:${m}:${s}`;
}
