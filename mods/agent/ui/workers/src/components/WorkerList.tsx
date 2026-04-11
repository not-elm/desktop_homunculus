import type { WorkerState } from '../hooks/useWorkers';

interface WorkerListProps {
  workers: Map<string, WorkerState>;
  selectedId: string | null;
  onSelect: (taskId: string) => void;
}

export function WorkerList({ workers, selectedId, onSelect }: WorkerListProps) {
  if (workers.size === 0) {
    return <EmptyState />;
  }

  return (
    <div className="flex flex-col gap-0.5 overflow-y-auto p-1.5">
      {[...workers.values()].map((worker) => (
        <WorkerRow
          key={worker.taskId}
          worker={worker}
          isSelected={worker.taskId === selectedId}
          onSelect={onSelect}
        />
      ))}
    </div>
  );
}

function EmptyState() {
  return (
    <div className="flex flex-1 items-center justify-center px-3 py-6">
      <span className="text-[var(--hud-text-muted)] text-[var(--hud-font-size-xs)]">
        No Workers running
      </span>
    </div>
  );
}

interface WorkerRowProps {
  worker: WorkerState;
  isSelected: boolean;
  onSelect: (taskId: string) => void;
}

function WorkerRow({ worker, isSelected, onSelect }: WorkerRowProps) {
  const hasPendingPermission = worker.pendingPermission !== null;

  return (
    <button
      type="button"
      onClick={() => onSelect(worker.taskId)}
      className={`
        relative flex w-full cursor-pointer items-center gap-2 rounded-md px-2.5 py-2 text-left
        transition-all duration-150
        border
        ${isSelected
          ? 'border-l-2 border-l-[oklch(0.72_0.14_192)] border-t-[var(--hud-border-decorative)] border-r-[var(--hud-border-decorative)] border-b-[var(--hud-border-decorative)] bg-[oklch(0.18_0.02_250)]'
          : 'border-transparent hover:border-[var(--hud-border-decorative)] hover:bg-[oklch(0.15_0.015_250)]'
        }
        ${hasPendingPermission ? 'animate-pulse-permission' : ''}
      `}
    >
      <StatusDot status={worker.status} />
      <div className="flex min-w-0 flex-1 flex-col gap-0.5">
        <span className="truncate text-[var(--hud-font-size-sm)] text-[var(--hud-text-primary)]">
          {worker.description}
        </span>
        <span className="text-[var(--hud-font-size-xs)] text-[var(--hud-text-muted)]">
          {worker.log.length} events
        </span>
      </div>
      {hasPendingPermission && <PermissionBadge />}
    </button>
  );
}

function StatusDot({ status }: { status: WorkerState['status'] }) {
  const color = statusColor(status);
  return (
    <span
      className="inline-block size-2 shrink-0 rounded-full"
      style={{ backgroundColor: color }}
    />
  );
}

function statusColor(status: WorkerState['status']): string {
  switch (status) {
    case 'running':
      return 'oklch(0.72 0.14 192)';
    case 'completed':
      return 'oklch(0.65 0.18 145)';
    case 'failed':
      return 'oklch(0.65 0.2 20)';
    case 'cancelled':
      return 'oklch(0.55 0.02 250)';
  }
}

function PermissionBadge() {
  return (
    <span className="flex size-5 shrink-0 items-center justify-center rounded-full bg-[oklch(0.65_0.2_20)] text-[10px] font-bold text-white">
      !
    </span>
  );
}
