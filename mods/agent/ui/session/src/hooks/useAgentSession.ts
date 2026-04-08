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
  /** Available decision options from AppServer. Undefined for Claude runtime. */
  availableDecisions?: Decision[];
}

export interface PendingQuestion {
  requestId: string;
  questions: unknown;
}

export function useAgentSession() {
  const [personaId, setPersonaId] = useState("");

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const p = await Webview.current()?.linkedPersona();
      if (cancelled) return;
      const id = p ? p.id : "";
      if (!cancelled) setPersonaId(id);
    })();
    return () => { cancelled = true; };
  }, []);

  const [state, setState] = useState<AgentState>("idle");
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [permission, setPermission] = useState<PendingPermission | null>(null);
  const [question, setQuestion] = useState<PendingQuestion | null>(null);
  const [elapsedMs, setElapsedMs] = useState(0);
  const [isRecording, setIsRecording] = useState(false);
  const [worktreeInfo, setWorktreeInfo] = useState<{ name: string; branch: string } | null>(null);
  const [error, setError] = useState<string | null>(null);
  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
    if (!personaId) return;
    let cancelled = false;
    checkInitialStatus(personaId).then((active) => {
      if (!cancelled && active) {
        setState((prev) => (prev === "idle" ? "listening" : prev));
        startTimeRef.current ??= Date.now();
      }
    });
    return () => { cancelled = true; };
  }, [personaId]);

  useEffect(() => {
    if (!personaId) return;
    const sources = [
      subscribeToStatus(personaId, (newState) => {
        setState(newState);
        if (newState !== "idle") {
          startTimeRef.current ??= Date.now();
        } else {
          startTimeRef.current = null;
          setElapsedMs(0);
          setWorktreeInfo(null);
        }
      }),
      subscribeToLog(personaId, (entry) => {
        setEntries((prev) => [...prev.slice(-99), entry]);
      }),
      subscribeToPermission(personaId, (perm) => {
        console.log("[useAgentSession] permission received:", JSON.stringify(perm));
        setPermission(perm);
        setState("waiting");
      }),
      subscribeToQuestion(personaId, (q) => {
        setQuestion(q);
        setState("waiting");
      }),
      subscribeToRecording(personaId, setIsRecording),
      subscribeToWorktree(personaId, setWorktreeInfo),
    ];
    return () => sources.forEach((s) => s.close());
  }, [personaId]);

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
    if (personaId) await callRpc("interrupt-session", { personaId });
  }, [personaId]);

  const startSession = useCallback(async () => {
    if (!personaId) return;
    setError(null);
    setEntries([]);
    try {
      await callRpc("start-session", { personaId });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }, [personaId]);

  const stopSession = useCallback(async () => {
    if (!personaId) return;
    setError(null);
    try {
      await callRpc("stop-session", { personaId });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }, [personaId]);

  const sendMessage = useCallback(async (text: string, contextSessionUuid?: string) => {
    if (!personaId || !text.trim()) return;
    setEntries([]);
    try {
      const result = await callRpc("send-message", {
        personaId,
        text: text.trim(),
        ...(contextSessionUuid && { contextSessionUuid }),
      }) as { success: boolean; replayEntries?: Array<{ type: string; message: string; timestamp: number; source?: string }> };
      if (result.replayEntries && result.replayEntries.length > 0) {
        setEntries(result.replayEntries.map((e, i) => ({
          id: `replay-${i}`,
          type: e.type as LogType,
          message: e.message,
          timestamp: e.timestamp,
          source: e.source as "voice" | "text" | undefined,
        })).slice(-100));
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }, [personaId]);

  const closePanel = useCallback(async () => {
    if (personaId && state !== "idle") {
      await callRpc("stop-session", { personaId }).catch(() => {});
    }
    audio.se.play("se:close").catch(() => {});
    await Webview.current()?.close();
  }, [personaId, state]);

  return {
    personaId,
    state,
    elapsedMs,
    entries,
    permission,
    question,
    hasPending: permission !== null || question !== null,
    isRecording,
    worktreeInfo,
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

async function checkInitialStatus(personaId: string): Promise<boolean> {
  try {
    const result = await callRpc("status", {});
    if (result && typeof result === "object") {
      return personaId in (result as Record<string, string>);
    }
  } catch { /* ignore */ }
  return false;
}

function callRpc(method: string, body: Record<string, unknown>) {
  return rpc.call({ modName: "@hmcs/agent", method, body });
}

function subscribeToStatus(id: string, onState: (state: AgentState) => void) {
  return signals.stream<{ personaId: string; state: string }>(
    "agent:status",
    (p) => { if (p.personaId === id) onState(p.state as AgentState); },
  );
}

function subscribeToLog(id: string, onEntry: (entry: LogEntry) => void) {
  return signals.stream<{ personaId: string; type: string; message: string; source?: "voice" | "text"; timestamp: number }>(
    "agent:log",
    (p) => {
      if (p.personaId === id) {
        onEntry({ id: crypto.randomUUID(), type: p.type as LogType, message: p.message, source: p.source, timestamp: p.timestamp });
      }
    },
  );
}

function subscribeToPermission(id: string, onPermission: (perm: PendingPermission) => void) {
  return signals.stream<{ personaId: string; requestId: string; action: string; target: string; availableDecisions?: Decision[] }>(
    "agent:permission",
    (p) => {
      if (p.personaId === id) {
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
  return signals.stream<{ personaId: string; requestId: string; questions: unknown }>(
    "agent:question",
    (p) => { if (p.personaId === id) onQuestion({ requestId: p.requestId, questions: p.questions }); },
  );
}

function subscribeToRecording(id: string, onRecording: (recording: boolean) => void) {
  return signals.stream<{ personaId: string; recording: boolean }>(
    "agent:recording",
    (p) => { if (p.personaId === id) onRecording(p.recording); },
  );
}

function subscribeToWorktree(id: string, onWorktree: (info: { name: string; branch: string } | null) => void) {
  return signals.stream<{ personaId: string; state: string; worktreeName?: string }>(
    "agent:worktree",
    (p) => {
      if (p.personaId !== id) return;
      if (p.state === "created" && p.worktreeName) {
        onWorktree({ name: p.worktreeName, branch: p.worktreeName });
      }
    },
  );
}
