import type { AgentRuntime } from './runtime/agent-runtime.ts';
import { ClaudeAgentRuntime } from './runtime/claude-agent-runtime.ts';
import type { CodexAppServerProcess } from './runtime/codex-appserver-process.ts';
import { CodexAppServerRuntime } from './runtime/codex-appserver-runtime.ts';
import { buildPersonaPrompt, type WorktreeContext } from './prompt.ts';
import type { AgentSettings, Persona } from './types.ts';

/** Options for building a Worker prompt. */
export interface WorkerPromptOptions {
  taskDescription: string;
  worktree?: WorktreeContext;
  /** Optional history injection for session continuity. */
  sessionContext?: string;
}

/**
 * Build the system prompt for a Worker.
 *
 * Reuses the base persona prompt and adds the task description. Workers
 * preserve the persona's voice but have a single concrete goal — the
 * delegated task.
 */
export function buildWorkerPrompt(persona: Persona, options: WorkerPromptOptions): string {
  const base = buildPersonaPrompt(persona, options.worktree, options.sessionContext);
  const task = `\n\n## Delegated Task\n\n${options.taskDescription}`;
  return `${base}${task}`;
}

/** Options for building a Worker runtime. */
export interface WorkerRuntimeOptions {
  settings: AgentSettings;
  prompt: string;
  apiKey: string | null;
  workDir: string;
  appServerProcess: CodexAppServerProcess;
}

/**
 * Create a Worker-configured {@link AgentRuntime}.
 *
 * Workers use the same tool set as today's single-session agents — they are
 * the concrete actors that edit files, run commands, and produce results.
 */
export function createWorkerRuntime(options: WorkerRuntimeOptions): AgentRuntime {
  const { settings, prompt, apiKey, workDir, appServerProcess } = options;
  switch (settings.runtime) {
    case 'codex':
      return new CodexAppServerRuntime(prompt, settings, workDir, appServerProcess);
    case 'sdk':
      if (!apiKey) throw new Error('SDK runtime requires an API key');
      return new ClaudeAgentRuntime(prompt, settings, apiKey, workDir);
    default:
      throw new Error(`Runtime "${settings.runtime}" is not implemented.`);
  }
}
