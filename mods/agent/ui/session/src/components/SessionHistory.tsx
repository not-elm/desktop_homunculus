import { useState, useEffect } from "react";
import { rpc } from "@hmcs/sdk/rpc";

interface SessionMeta {
  uuid: string;
  startedAt: number;
  preview: string | null;
}

interface SessionHistoryProps {
  workspacePath: string;
  personaId: string;
  branchName: string | null;
  onSelectSession: (uuid: string) => void;
}

export function SessionHistory({ workspacePath, personaId, branchName, onSelectSession }: SessionHistoryProps) {
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
        }) as { success: boolean; sessions?: SessionMeta[] };
        if (!cancelled && result.success && result.sessions) {
          setSessions(result.sessions);
        }
      } catch {
        // silently fail
      }
      if (!cancelled) setLoading(false);
    })();
    return () => { cancelled = true; };
  }, [workspacePath, personaId, branchName]);

  if (loading) return null;

  const groups = groupByDate(sessions);

  return (
    <div className="hud-log" style={{ flex: 1, overflowY: "auto" }}>
      {sessions.length === 0 ? (
        <div className="hud-log-empty">No previous sessions</div>
      ) : (
        groups.map(([label, items]) => (
          <div key={label}>
            <div style={{
              color: "#6c7a89",
              fontSize: "10px",
              fontWeight: "bold",
              textTransform: "uppercase",
              letterSpacing: "1px",
              margin: "12px 0 4px",
              padding: "0 8px",
            }}>
              {label}
            </div>
            {items.map((s) => (
              <button
                key={s.uuid}
                type="button"
                onClick={() => onSelectSession(s.uuid)}
                style={{
                  display: "flex",
                  justifyContent: "space-between",
                  width: "100%",
                  background: "rgba(255,255,255,0.05)",
                  border: "1px solid rgba(255,255,255,0.1)",
                  borderRadius: "6px",
                  padding: "8px 10px",
                  marginBottom: "6px",
                  cursor: "pointer",
                  textAlign: "left",
                  color: "#e0e0e0",
                  fontSize: "12px",
                }}
              >
                <span style={{ flex: 1, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                  {s.preview ?? "Empty session"}
                </span>
                <span style={{ color: "#666", fontSize: "10px", marginLeft: "8px", flexShrink: 0 }}>
                  {formatTime(s.startedAt)}
                </span>
              </button>
            ))}
          </div>
        ))
      )}
    </div>
  );
}

function groupByDate(sessions: SessionMeta[]): [string, SessionMeta[]][] {
  const groups = new Map<string, SessionMeta[]>();
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime();
  const yesterday = today - 86400000;

  for (const s of sessions) {
    const label = dateLabel(s.startedAt, today, yesterday);
    if (!groups.has(label)) groups.set(label, []);
    groups.get(label)!.push(s);
  }

  return [...groups.entries()];
}

function dateLabel(startedAt: number, today: number, yesterday: number): string {
  if (startedAt >= today) return "Today";
  if (startedAt >= yesterday) return "Yesterday";
  return new Date(startedAt).toLocaleDateString("en-US", { month: "short", day: "numeric" });
}

function formatTime(ts: number): string {
  return new Date(ts).toLocaleTimeString("en-US", { hour: "2-digit", minute: "2-digit", hour12: false });
}
