import { rpc } from '@hmcs/sdk/rpc';
import type { WorkerPermission } from '../hooks/useWorkers';

interface PermissionPanelProps {
  permission: WorkerPermission;
}

export function PermissionPanel({ permission }: PermissionPanelProps) {
  return (
    <div className="mx-2 my-1.5 rounded-md border border-[oklch(0.65_0.2_20/0.4)] bg-[oklch(0.14_0.02_20/0.3)] p-3">
      <ToolBadge tool={permission.tool} />
      <InputPreview input={permission.input} />
      {permission.description && (
        <p className="mt-1 text-[var(--hud-font-size-xs)] text-[var(--hud-text-secondary)]">
          {permission.description}
        </p>
      )}
      <ActionButtons requestId={permission.requestId} />
    </div>
  );
}

function ToolBadge({ tool }: { tool: string }) {
  return (
    <div className="mb-1.5 flex items-center gap-1.5">
      <span className="text-[10px] text-[oklch(0.65_0.2_20)]">&#x26A0;</span>
      <span className="rounded bg-[oklch(0.2_0.02_250)] px-1.5 py-0.5 text-[var(--hud-font-size-xs)] font-medium text-[var(--hud-text-primary)]">
        {tool}
      </span>
    </div>
  );
}

function InputPreview({ input }: { input: unknown }) {
  const text = typeof input === 'string' ? input : JSON.stringify(input, null, 2);
  return (
    <pre className="max-h-20 overflow-auto rounded bg-[oklch(0.1_0.015_250)] p-1.5 text-[var(--hud-font-size-xs)] leading-[var(--hud-line-height-normal)] text-[var(--hud-text-tertiary)]">
      {text}
    </pre>
  );
}

function ActionButtons({ requestId }: { requestId: string }) {
  async function handleApprove() {
    await callAgentRpc('approve-permission', {
      requestId,
      approved: true,
      decision: 'accept',
    });
  }

  async function handleDeny() {
    await callAgentRpc('approve-permission', {
      requestId,
      approved: false,
      decision: 'decline',
    });
  }

  return (
    <div className="mt-2 flex gap-2">
      <button
        type="button"
        onClick={handleApprove}
        className="cursor-pointer rounded border border-[oklch(0.65_0.18_145/0.4)] bg-[oklch(0.65_0.18_145/0.15)] px-3 py-1 text-[var(--hud-font-size-xs)] font-medium text-[oklch(0.78_0.14_155)] transition-colors hover:bg-[oklch(0.65_0.18_145/0.3)]"
      >
        Allow
      </button>
      <button
        type="button"
        onClick={handleDeny}
        className="cursor-pointer rounded border border-[oklch(0.65_0.2_20/0.4)] bg-[oklch(0.65_0.2_20/0.15)] px-3 py-1 text-[var(--hud-font-size-xs)] font-medium text-[oklch(0.78_0.14_15)] transition-colors hover:bg-[oklch(0.65_0.2_20/0.3)]"
      >
        Deny
      </button>
    </div>
  );
}

function callAgentRpc(method: string, body: Record<string, unknown>) {
  return rpc.call({ modName: '@hmcs/agent', method, body });
}
