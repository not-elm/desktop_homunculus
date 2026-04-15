import type { Gender } from '@hmcs/sdk';

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
  /** TTS engine MOD name. Null means no TTS (text chat only). */
  ttsModName: string | null;
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
  ttsModName: null,
};

export type { AgentEvent, AgentResponse, AgentRuntime } from './agent-runtime.ts';
