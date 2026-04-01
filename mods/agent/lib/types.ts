import type { Gender, Ocean } from "@hmcs/sdk";

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
  workingDirectories: { paths: string[]; default: number };
  allowList: string[];
  disallowedTools: string[];
  /** Shell command patterns auto-approved by the Codex executor (regex strings). */
  commandAutoApprovePatterns: string[];
  claudeModel: string;
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
  age: number | null;
  gender: Gender;
  firstPersonPronoun: string | null;
  profile: string;
  ocean: Ocean;
}

export const DEFAULT_SETTINGS: AgentSettings = {
  executor: "codex",
  pttKey: null,
  approvalPhrases: ["はい", "yes", "ok", "allow"],
  denyPhrases: ["いいえ", "no", "deny", "cancel"],
  workingDirectories: { paths: [], default: 0 },
  allowList: [],
  disallowedTools: [],
  commandAutoApprovePatterns: ["^(cat|head|tail|less|more)\\s", "^ls\\b", "^pwd$", "^echo\\s", "^wc\\s", "^find\\s", "^grep\\s", "^rg\\s"],
  claudeModel: "",
};

export type { AgentEvent, AgentResponse, AIAgentExecuter } from "./ai-agent-executer.ts";
