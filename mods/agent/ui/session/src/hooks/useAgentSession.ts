import { useCallback, useEffect, useRef, useState } from "react";
import { audio, signals, Webview } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";

export type AgentState = "idle" | "thinking" | "executing" | "waiting" | "listening";

export type LogType = "read" | "edit" | "run" | "tool" | "assistant" | "done" | "error" | "warning" | "user" | "interrupt";

/** A decision can be a simple string ("accept", "decline") or a tagged-union object from the AppServer. */
export type Decision = string | Record<string, unknown>;

export interface LogEntry {
  id: string;
  type: LogType;
  message: string;
  source?: "voice" | "text";
  timestamp: number;
}

export interface PendingPermission {
  requestId: string;
  action: string;
  target: string;
  /** Available decision options from AppServer. Undefined for Claude executor. */
  availableDecisions?: Decision[];
}

export interface PendingQuestion {
  requestId: string;
  questions: unknown;
}

export function useAgentSession() {
  const [characterId, setCharacterId] = useState("");

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const vrm = await Webview.current()?.linkedVrm();
      if (cancelled) return;
      const name = vrm ? await vrm.name() : "";
      if (!cancelled) setCharacterId(name);
    })();
    return () => { cancelled = true; };
  }, []);

  const [state, setState] = useState<AgentState>("idle");
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [permission, setPermission] = useState<PendingPermission | null>(null);
  const [question, setQuestion] = useState<PendingQuestion | null>(null);
  const [elapsedMs, setElapsedMs] = useState(0);
  const [isRecording, setIsRecording] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
    if (!characterId) return;
    let cancelled = false;
    checkInitialStatus(characterId).then((active) => {
      if (!cancelled && active) {
        setState((prev) => (prev === "idle" ? "listening" : prev));
        startTimeRef.current ??= Date.now();
      }
    });
    return () => { cancelled = true; };
  }, [characterId]);

  useEffect(() => {
    if (!characterId) return;
    const sources = [
      subscribeToStatus(characterId, (newState) => {
        setState(newState);
        if (newState !== "idle") {
          startTimeRef.current ??= Date.now();
        } else {
          startTimeRef.current = null;
          setElapsedMs(0);
        }
      }),
      subscribeToLog(characterId, (entry) => {
        setEntries((prev) => [...prev.slice(-99), entry]);
      }),
      subscribeToPermission(characterId, (perm) => {
        setPermission(perm);
        setState("waiting");
      }),
      subscribeToQuestion(characterId, (q) => {
        setQuestion(q);
        setState("waiting");
      }),
      subscribeToRecording(characterId, setIsRecording),
    ];
    return () => sources.forEach((s) => s.close());
  }, [characterId]);

  useEffect(() => {
    if (state === "idle") return;
    const interval = setInterval(() => {
      if (startTimeRef.current !== null) {
        setElapsedMs(Date.now() - startTimeRef.current);
      }
    }, 500);
    return () => clearInterval(interval);
  }, [state]);

  const approvePermission = useCallback(async (requestId: string, decision?: Decision) => {
    await callRpc("approve-permission", { requestId, approved: true, decision: decision ?? "accept" });
    setPermission(null);
  }, []);

  const denyPermission = useCallback(async (requestId: string, decision?: Decision) => {
    await callRpc("approve-permission", { requestId, approved: false, decision: decision ?? "decline" });
    setPermission(null);
  }, []);

  const answerQuestion = useCallback(async (requestId: string, answers: Record<string, string>) => {
    await callRpc("answer-question", { requestId, answers });
    setQuestion(null);
  }, []);

  const interruptSession = useCallback(async () => {
    if (characterId) await callRpc("interrupt-session", { characterId });
  }, [characterId]);

  const startSession = useCallback(async () => {
    if (!characterId) return;
    setError(null);
    setEntries([]);
    try {
      await callRpc("start-session", { characterId });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }, [characterId]);

  const stopSession = useCallback(async () => {
    if (!characterId) return;
    setError(null);
    try {
      await callRpc("stop-session", { characterId });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }, [characterId]);

  const sendMessage = useCallback(async (text: string) => {
    if (!characterId || !text.trim()) return;
    await callRpc("send-message", { characterId, text: text.trim() });
  }, [characterId]);

  const closePanel = useCallback(async () => {
    if (characterId && state !== "idle") {
      await callRpc("stop-session", { characterId }).catch(() => {});
    }
    audio.se.play("se:close").catch(() => {});
    await Webview.current()?.close();
  }, [characterId, state]);

  return {
    state,
    elapsedMs,
    entries,
    permission,
    question,
    hasPending: permission !== null || question !== null,
    isRecording,
    error,
    approvePermission,
    denyPermission,
    answerQuestion,
    interruptSession,
    startSession,
    stopSession,
    sendMessage,
    closePanel,
  };
}

async function checkInitialStatus(characterId: string): Promise<boolean> {
  try {
    const result = await callRpc("status", {});
    if (result && typeof result === "object") {
      return characterId in (result as Record<string, string>);
    }
  } catch { /* ignore */ }
  return false;
}

function callRpc(method: string, body: Record<string, unknown>) {
  return rpc.call({ modName: "@hmcs/agent", method, body });
}

function subscribeToStatus(id: string, onState: (state: AgentState) => void) {
  return signals.stream<{ characterId: string; state: string }>(
    "agent:status",
    (p) => { if (p.characterId === id) onState(p.state as AgentState); },
  );
}

function subscribeToLog(id: string, onEntry: (entry: LogEntry) => void) {
  return signals.stream<{ characterId: string; type: string; message: string; source?: "voice" | "text"; timestamp: number }>(
    "agent:log",
    (p) => {
      if (p.characterId === id) {
        onEntry({ id: crypto.randomUUID(), type: p.type as LogType, message: p.message, source: p.source, timestamp: p.timestamp });
      }
    },
  );
}

function subscribeToPermission(id: string, onPermission: (perm: PendingPermission) => void) {
  return signals.stream<{ characterId: string; requestId: string; action: string; target: string; availableDecisions?: Decision[] }>(
    "agent:permission",
    (p) => {
      if (p.characterId === id) {
        onPermission({
          requestId: p.requestId,
          action: p.action,
          target: p.target,
          availableDecisions: p.availableDecisions,
        });
      }
    },
  );
}

function subscribeToQuestion(id: string, onQuestion: (q: PendingQuestion) => void) {
  return signals.stream<{ characterId: string; requestId: string; questions: unknown }>(
    "agent:question",
    (p) => { if (p.characterId === id) onQuestion({ requestId: p.requestId, questions: p.questions }); },
  );
}

function subscribeToRecording(id: string, onRecording: (recording: boolean) => void) {
  return signals.stream<{ characterId: string; recording: boolean }>(
    "agent:recording",
    (p) => { if (p.characterId === id) onRecording(p.recording); },
  );
}
