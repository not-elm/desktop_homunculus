import { useState, useEffect } from "react";
import { rpc } from "@hmcs/sdk/rpc";
import { ActivityLog } from "./ActivityLog";
import { TextInput } from "./TextInput";
import type { LogEntry } from "../hooks/useAgentSession";

interface PastSessionViewProps {
  workspacePath: string;
  personaId: string;
  branchName: string | null;
  uuid: string;
  onBack: () => void;
  onSendMessage: (text: string, contextUuid: string) => Promise<void>;
}

export function PastSessionView({
  workspacePath,
  personaId,
  branchName,
  uuid,
  onBack,
  onSendMessage,
}: PastSessionViewProps) {
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [sessionDate, setSessionDate] = useState("");

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
          method: "get-session-logs",
          body: { workspacePath, personaId, branchName, uuid },
        }) as { entries?: Array<{ type: string; message: string; timestamp: number; source?: string }> };
        if (!cancelled && result.entries) {
          const mapped = mapEntries(result.entries);
          setEntries(mapped);
          if (mapped.length > 0) {
            setSessionDate(formatDateTime(mapped[0].timestamp));
          }
        }
      } catch {
        // silently fail
      }
      if (!cancelled) setLoading(false);
    })();
    return () => { cancelled = true; };
  }, [workspacePath, personaId, branchName, uuid]);

  if (loading) return null;

  return (
    <div className="uv-session">
      <PastSessionHeader date={sessionDate} onBack={onBack} />
      <ActivityLog entries={entries} />
      <TextInput
        onSend={(text) => onSendMessage(text, uuid)}
        isInterruptible={false}
        onInterrupt={() => {}}
      />
    </div>
  );
}

function PastSessionHeader({ date, onBack }: { date: string; onBack: () => void }) {
  return (
    <div style={{
      display: "flex",
      alignItems: "center",
      gap: "8px",
      padding: "8px 12px",
      borderBottom: "1px solid rgba(255,255,255,0.1)",
    }}>
      <button
        type="button"
        onClick={onBack}
        style={{
          background: "none",
          border: "none",
          color: "#7c8da6",
          cursor: "pointer",
          fontSize: "14px",
          padding: "4px",
        }}
      >
        &larr;
      </button>
      <span style={{ color: "#aaa", fontSize: "11px" }}>{date}</span>
      <span style={{ color: "#555", fontSize: "10px", fontStyle: "italic" }}>read-only</span>
    </div>
  );
}

function mapEntries(
  raw: Array<{ type: string; message: string; timestamp: number; source?: string }>,
): LogEntry[] {
  return raw.map((e, i) => ({
    id: `past-${i}`,
    type: e.type as LogEntry["type"],
    message: e.message,
    timestamp: e.timestamp,
    source: e.source as LogEntry["source"],
  }));
}

function formatDateTime(ts: number): string {
  return new Date(ts).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });
}
