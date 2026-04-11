import { signals, Webview } from '@hmcs/sdk';
import { useEffect, useState } from 'react';

export interface WorkerBadgeState {
  runningCount: number;
  hasPendingPermission: boolean;
}

export function useWorkerBadge(): WorkerBadgeState {
  const [personaId, setPersonaId] = useState('');
  const [runningWorkers, setRunningWorkers] = useState<Set<string>>(new Set());
  const [pendingPermissions, setPendingPermissions] = useState<Set<string>>(new Set());

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

  useEffect(() => {
    if (!personaId) return;

    const workerSub = signals.stream<{
      personaId: string;
      workerId: string;
      event: string;
    }>('agent:worker-event', (data) => {
      if (data.personaId !== personaId) return;
      setRunningWorkers((prev) => {
        const next = new Set(prev);
        if (data.event === 'started') {
          next.add(data.workerId);
        } else if (data.event === 'stopped' || data.event === 'error') {
          next.delete(data.workerId);
        }
        return next;
      });
    });

    const permSub = signals.stream<{
      personaId: string;
      workerId: string;
      requestId: string;
      resolved?: boolean;
    }>('agent:permission', (data) => {
      if (data.personaId !== personaId) return;
      if (!data.workerId) return;
      setPendingPermissions((prev) => {
        const next = new Set(prev);
        if (data.resolved) {
          next.delete(data.workerId);
        } else {
          next.add(data.workerId);
        }
        return next;
      });
    });

    return () => {
      workerSub.close();
      permSub.close();
    };
  }, [personaId]);

  return {
    runningCount: runningWorkers.size,
    hasPendingPermission: pendingPermissions.size > 0,
  };
}
