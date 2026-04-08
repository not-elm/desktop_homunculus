import { useState, useEffect } from "react";
import { rpc } from "@hmcs/sdk/rpc";

export interface SessionMeta {
  uuid: string;
  startedAt: number;
  preview: string | null;
}

/** Fetch session history for a persona on a specific branch. */
export function useSessionList(
  workspacePath: string,
  personaId: string,
  branchName: string | null,
): { sessions: SessionMeta[]; loading: boolean } {
  const [sessions, setSessions] = useState<SessionMeta[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!branchName) {
      setLoading(false);
      return;
    }
    let cancelled = false;
    (async () => {
      try {
        const result = await rpc.call({
          modName: "@hmcs/agent",
          method: "list-sessions",
          body: { workspacePath, personaId, branchName },
        }) as { sessions?: SessionMeta[] };
        if (!cancelled && result.sessions) {
          setSessions(result.sessions);
        }
      } catch {
        // silently fail
      }
      if (!cancelled) setLoading(false);
    })();
    return () => { cancelled = true; };
  }, [workspacePath, personaId, branchName]);

  return { sessions, loading };
}
