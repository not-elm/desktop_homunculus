/** PTT key configuration. */
export interface PttKey {
  code: string;
  modifiers: string[];
}

/** Agent settings stored in preferences. */
export interface AgentSettings {
  executor: "sdk" | "cli" | "codex";
  pttKey: PttKey | null;
  approvalPhrases: string[];
  denyPhrases: string[];
  greetingPhrases: string[];
  completionPhrases: string[];
  errorPhrases: string[];
  workingDirectories: { paths: string[]; default: number };
  allowList: string[];
  disallowedTools: string[];
  model: string;
}

/** Session lifecycle states. */
export type SessionState = "idle" | "listening" | "thinking" | "executing";

/** Agent status signal payload — superset of SessionState for UI. */
export type AgentStatus =
  | "idle"
  | "listening"
  | "thinking"
  | "executing"
  | "waiting";

/** Activity log entry types. */
export type LogType =
  | "read"
  | "edit"
  | "run"
  | "tool"
  | "assistant"
  | "done"
  | "error"
  | "warning"
  | "user"
  | "interrupt";

/** Activity log entry. */
export interface LogEntry {
  type: LogType;
  message: string;
  timestamp: number;
}

/** Character persona information. */
export interface Persona {
  name: string;
  personality: string;
}

export const DEFAULT_SETTINGS: AgentSettings = {
  executor: "sdk",
  pttKey: null,
  approvalPhrases: ["はい", "yes", "ok", "allow"],
  denyPhrases: ["いいえ", "no", "deny", "cancel"],
  greetingPhrases: [],
  completionPhrases: [],
  errorPhrases: [],
  workingDirectories: { paths: [], default: 0 },
  allowList: [],
  disallowedTools: [],
  model: "",
};

export type { AgentEvent, AgentResponse, AIAgentExecuter } from "./ai-agent-executer.ts";
