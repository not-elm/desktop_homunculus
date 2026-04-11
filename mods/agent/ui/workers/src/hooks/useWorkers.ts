import { signals, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useRef, useState } from 'react';

/** A single log entry from a Worker's activity. */
export interface WorkerLogEntry {
  type: string;
  timestamp: number;
  summary?: string;
  text?: string;
  tool?: string;
}

/** The pending permission attached to a Worker, if any. */
export interface WorkerPermission {
  requestId: string;
  tool: string;
  title?: string;
  description?: string;
  input: unknown;
  availableDecisions?: unknown[];
}

/** Aggregated state of a single Worker task. */
export interface WorkerState {
  taskId: string;
  description: string;
  status: 'running' | 'completed' | 'failed' | 'cancelled';
  pendingPermission: WorkerPermission | null;
  log: WorkerLogEntry[];
}

/** Shape of the `agent:worker-event` signal payload. */
interface WorkerEventPayload {
  personaId: string;
  taskId: string;
  event: {
    type: string;
    text?: string;
    tool?: string;
    summary?: string;
    message?: string;
    sessionId?: string;
    requestId?: string;
    input?: unknown;
    title?: string;
    description?: string;
    availableDecisions?: unknown[];
  };
}

/** Shape of the `agent:permission` signal payload (for Workers). */
interface PermissionPayload {
  personaId: string;
  taskId?: string;
  requestId: string;
  action: string;
  target: string;
  availableDecisions?: unknown[];
  resolved?: boolean;
}

interface UseWorkersReturn {
  workers: Map<string, WorkerState>;
  selectedId: string | null;
  setSelectedId: (id: string | null) => void;
  personaId: string | null;
}

/**
 * Subscribe to Worker events and permission signals, aggregating them into
 * per-task state objects.
 */
export function useWorkers(): UseWorkersReturn {
  const [personaId, setPersonaId] = useState<string | null>(null);
  const [workers, setWorkers] = useState<Map<string, WorkerState>>(new Map());
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const removalTimers = useRef<Map<string, ReturnType<typeof setTimeout>>>(new Map());

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const p = await Webview.current()?.linkedPersona();
      if (cancelled) return;
      if (p) setPersonaId(p.id);
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const scheduleRemoval = useCallback((taskId: string) => {
    const existing = removalTimers.current.get(taskId);
    if (existing) clearTimeout(existing);

    const timer = setTimeout(() => {
      removalTimers.current.delete(taskId);
      setWorkers((prev) => {
        const next = new Map(prev);
        next.delete(taskId);
        return next;
      });
      setSelectedId((prev) => (prev === taskId ? null : prev));
    }, 3000);

    removalTimers.current.set(taskId, timer);
  }, []);

  useEffect(() => {
    if (!personaId) return;

    const workerSub = signals.stream<WorkerEventPayload>(
      'agent:worker-event',
      (data) => {
        if (data.personaId !== personaId) return;
        handleWorkerEvent(data);
      },
    );

    const permSub = signals.stream<PermissionPayload>(
      'agent:permission',
      (data) => {
        if (data.personaId !== personaId) return;
        if (!data.taskId) return;
        handlePermission(data);
      },
    );

    return () => {
      workerSub.close();
      permSub.close();
      for (const timer of removalTimers.current.values()) clearTimeout(timer);
      removalTimers.current.clear();
    };
  }, [personaId, scheduleRemoval]);

  function handleWorkerEvent(data: WorkerEventPayload) {
    const { taskId, event } = data;

    setWorkers((prev) => {
      const next = new Map(prev);
      const existing = next.get(taskId);
      const worker = existing ?? createWorker(taskId, data);
      const logEntry = eventToLogEntry(event);

      const updatedWorker: WorkerState = {
        ...worker,
        status: deriveStatus(event, worker.status),
        log: logEntry ? [...worker.log, logEntry] : worker.log,
        pendingPermission:
          event.type === 'permission_request' ? null : worker.pendingPermission,
      };

      next.set(taskId, updatedWorker);
      return next;
    });

    setSelectedId((prev) => prev ?? taskId);

    if (isTerminalEvent(data.event.type)) {
      scheduleRemoval(taskId);
    }
  }

  function handlePermission(data: PermissionPayload) {
    const taskId = data.taskId!;

    if (data.resolved) {
      clearPermission(taskId);
      return;
    }

    setWorkers((prev) => {
      const next = new Map(prev);
      const worker = next.get(taskId);
      if (!worker) return prev;

      next.set(taskId, {
        ...worker,
        pendingPermission: {
          requestId: data.requestId,
          tool: data.action,
          input: data.target,
          availableDecisions: data.availableDecisions,
        },
      });
      return next;
    });
  }

  function clearPermission(taskId: string) {
    setWorkers((prev) => {
      const next = new Map(prev);
      const worker = next.get(taskId);
      if (!worker) return prev;
      next.set(taskId, { ...worker, pendingPermission: null });
      return next;
    });
  }

  return { workers, selectedId, setSelectedId, personaId };
}

function createWorker(taskId: string, data: WorkerEventPayload): WorkerState {
  return {
    taskId,
    description: taskId,
    status: 'running',
    pendingPermission: null,
    log: [],
  };
}

function eventToLogEntry(
  event: WorkerEventPayload['event'],
): WorkerLogEntry | null {
  const timestamp = Date.now();

  switch (event.type) {
    case 'assistant_message':
      return { type: 'assistant_message', timestamp, text: event.text };
    case 'tool_use':
      return {
        type: 'tool_use',
        timestamp,
        tool: event.tool,
        summary: event.summary,
      };
    case 'permission_request':
      return {
        type: 'permission',
        timestamp,
        tool: event.tool,
        summary: `Permission: ${event.tool}`,
      };
    case 'completed':
      return { type: 'completed', timestamp, summary: 'Task completed' };
    case 'error':
      return {
        type: 'error',
        timestamp,
        text: event.message,
        summary: event.message,
      };
    default:
      return null;
  }
}

function deriveStatus(
  event: WorkerEventPayload['event'],
  current: WorkerState['status'],
): WorkerState['status'] {
  if (event.type === 'completed') return 'completed';
  if (event.type === 'error') return 'failed';
  return current;
}

function isTerminalEvent(type: string): boolean {
  return type === 'completed' || type === 'error';
}
