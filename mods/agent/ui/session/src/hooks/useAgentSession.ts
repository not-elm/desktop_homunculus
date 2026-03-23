import { useCallback, useEffect, useRef, useState } from "react";
import { signals } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";

export type AgentState = "idle" | "thinking" | "executing" | "waiting";

export type LogType = "read" | "edit" | "run" | "tool" | "assistant" | "done" | "error" | "warning";

export interface LogEntry {
  id: string;
  type: LogType;
  message: string;
  timestamp: number;
}

export interface PendingPermission {
  requestId: string;
  action: string;
  target: string;
}

export interface PendingQuestion {
  requestId: string;
  questions: unknown;
}

export interface AgentSessionState {
  state: AgentState;
  elapsedMs: number;
  entries: LogEntry[];
  permission: PendingPermission | null;
  question: PendingQuestion | null;
  hasPending: boolean;
}

export interface AgentSessionActions {
  approvePermission: (requestId: string) => Promise<void>;
  denyPermission: (requestId: string) => Promise<void>;
  answerQuestion: (requestId: string, answers: Record<string, string>) => Promise<void>;
}

const characterId = new URLSearchParams(location.search).get("linkedCharacter") ?? "";

export function useAgentSession(): AgentSessionState & AgentSessionActions {
  const [state, setState] = useState<AgentState>("idle");
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [permission, setPermission] = useState<PendingPermission | null>(null);
  const [question, setQuestion] = useState<PendingQuestion | null>(null);
  const [elapsedMs, setElapsedMs] = useState(0);
  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
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
    ];
    return () => sources.forEach((s) => s.close());
  }, []);

  useEffect(() => {
    if (state === "idle") return;
    const interval = setInterval(() => {
      if (startTimeRef.current !== null) {
        setElapsedMs(Date.now() - startTimeRef.current);
      }
    }, 500);
    return () => clearInterval(interval);
  }, [state]);

  const approvePermission = useCallback(async (requestId: string) => {
    await callApprovePermission(requestId, true);
    setPermission(null);
  }, []);

  const denyPermission = useCallback(async (requestId: string) => {
    await callApprovePermission(requestId, false);
    setPermission(null);
  }, []);

  const answerQuestion = useCallback(async (requestId: string, answers: Record<string, string>) => {
    await callAnswerQuestion(requestId, answers);
    setQuestion(null);
  }, []);

  return {
    state,
    elapsedMs,
    entries,
    permission,
    question,
    hasPending: permission !== null || question !== null,
    approvePermission,
    denyPermission,
    answerQuestion,
  };
}

function subscribeToStatus(
  id: string,
  onState: (state: AgentState) => void,
) {
  return signals.stream<{ characterId: string; state: string }>(
    "agent:status",
    (payload) => {
      if (payload.characterId === id) {
        onState(payload.state as AgentState);
      }
    },
  );
}

function subscribeToLog(
  id: string,
  onEntry: (entry: LogEntry) => void,
) {
  return signals.stream<{ characterId: string; type: string; message: string; timestamp: number }>(
    "agent:log",
    (payload) => {
      if (payload.characterId === id) {
        onEntry({
          id: crypto.randomUUID(),
          type: payload.type as LogType,
          message: payload.message,
          timestamp: payload.timestamp,
        });
      }
    },
  );
}

function subscribeToPermission(
  id: string,
  onPermission: (perm: PendingPermission) => void,
) {
  return signals.stream<{ characterId: string; requestId: string; action: string; target: string }>(
    "agent:permission",
    (payload) => {
      if (payload.characterId === id) {
        onPermission({
          requestId: payload.requestId,
          action: payload.action,
          target: payload.target,
        });
      }
    },
  );
}

function subscribeToQuestion(
  id: string,
  onQuestion: (q: PendingQuestion) => void,
) {
  return signals.stream<{ characterId: string; requestId: string; questions: unknown }>(
    "agent:question",
    (payload) => {
      if (payload.characterId === id) {
        onQuestion({
          requestId: payload.requestId,
          questions: payload.questions,
        });
      }
    },
  );
}

async function callApprovePermission(requestId: string, approved: boolean): Promise<void> {
  await rpc.call({
    modName: "@hmcs/agent",
    method: "approve-permission",
    body: { requestId, approved },
  });
}

async function callAnswerQuestion(requestId: string, answers: Record<string, string>): Promise<void> {
  await rpc.call({
    modName: "@hmcs/agent",
    method: "answer-question",
    body: { requestId, answers },
  });
}
