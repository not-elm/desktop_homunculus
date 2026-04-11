import type { Gender } from '@hmcs/sdk';
import type { AgentEvent } from './agent-runtime.ts';

/** PTT key configuration. */
export interface PttKey {
  code: string;
  modifiers: string[];
}

/** Agent settings stored in preferences. */
export interface AgentSettings {
  runtime: 'sdk' | 'cli' | 'codex';
  pttKey: PttKey | null;
  approvalPhrases: string[];
  denyPhrases: string[];
  /** Workspace paths and the current selection. */
  workspaces: { paths: string[]; selection: WorkspaceSelection };
  allowList: string[];
  disallowedTools: string[];
  /** Shell command patterns auto-approved by the Codex runtime (regex strings). */
  commandAutoApprovePatterns: string[];
  claudeModel: string;
}

/** Session lifecycle states. */
export type SessionState = 'idle' | 'listening' | 'thinking' | 'executing';

/** Agent status signal payload — superset of SessionState for UI. */
export type AgentStatus = 'idle' | 'listening' | 'thinking' | 'executing' | 'waiting';

/** Activity log entry types. */
export type LogType =
  | 'read'
  | 'edit'
  | 'run'
  | 'tool'
  | 'assistant'
  | 'done'
  | 'error'
  | 'warning'
  | 'user'
  | 'interrupt';

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
  personality: string | null;
}

/** Selection state: which workspace/worktree is active. */
export interface WorkspaceSelection {
  /** Index into the workspaces array. */
  workspaceIndex: number;
  /** If set, the selected worktree name within that workspace. Null = root workspace selected. */
  worktreeName: string | null;
}

/** Worktree lifecycle state for the agent:worktree signal. */
export type WorktreeState = 'created' | 'orphaned' | 'error';

/** Payload for the agent:worktree signal. */
export interface WorktreeSignalPayload {
  personaId: string;
  state: WorktreeState;
  worktreeName?: string;
  workspacePath?: string;
  error?: string;
}

/** A worker task spawned by the session manager on behalf of a persona. */
export interface WorkerTask {
  taskId: string;
  personaId: string;
  controller: AbortController;
  sessionId: string | null;
  status: 'running' | 'completed' | 'failed' | 'cancelled';
  worktreeName: string | null;
  description: string;
  /** ISO timestamp when the task was spawned. */
  startedAt: string;
  /** ISO timestamp when the task reached a terminal state. */
  endedAt: string | null;
  /** Last error message when status is 'failed'. */
  errorMessage: string | null;
}

/** Active sessions for a single persona (frontman + worker pool). */
export interface PersonaSessions {
  frontman?: {
    controller: AbortController;
    sessionId: string | null;
  };
  workers: Map<string, WorkerTask>;
}

/** Event payload forwarded from a worker task to the orchestrator. */
export interface WorkerEventPayload {
  personaId: string;
  taskId: string;
  event: AgentEvent;
}

/** A peer-to-peer message routed between personas. */
export interface PeerMessage {
  from: string;
  to: string;
  message: string;
  replyTo?: string;
  timestamp: string;
}

/** Maximum number of concurrent worker tasks per persona. */
export const DEFAULT_WORKER_LIMIT = 3;

/** Worker task timeout in milliseconds (10 minutes). */
export const DEFAULT_WORKER_TIMEOUT_MS = 10 * 60 * 1000;

export const DEFAULT_SETTINGS: AgentSettings = {
  runtime: 'codex',
  pttKey: null,
  approvalPhrases: ['はい', 'yes', 'ok', 'allow'],
  denyPhrases: ['いいえ', 'no', 'deny', 'cancel'],
  workspaces: { paths: [], selection: { workspaceIndex: 0, worktreeName: null } },
  allowList: [],
  disallowedTools: [],
  commandAutoApprovePatterns: [
    '^(cat|head|tail|less|more)\\s',
    '^ls\\b',
    '^pwd$',
    '^echo\\s',
    '^wc\\s',
    '^find\\s',
    '^grep\\s',
    '^rg\\s',
  ],
  claudeModel: '',
};

export type { AgentEvent, AgentResponse, AgentRuntime } from './agent-runtime.ts';
