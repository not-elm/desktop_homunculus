import { buildPersonaPrompt } from './prompt.ts';
import type { AgentRuntime } from './runtime/agent-runtime.ts';
import { ClaudeAgentRuntime } from './runtime/claude-agent-runtime.ts';
import type { CodexAppServerProcess } from './runtime/codex-appserver-process.ts';
import { CodexAppServerRuntime } from './runtime/codex-appserver-runtime.ts';
import type { AgentSettings, Persona } from './types.ts';

const FRONTMAN_INSTRUCTIONS = `
## Your role

You are the user-facing voice of this persona. Your entire job is conversation:
- Reply in 1-3 spoken sentences. No markdown, no bullet points.
- You do NOT edit files, run commands, or do any implementation work yourself.
- When the user asks for implementation, research, or any non-trivial task, delegate it using the agent mod's \`delegate-task\` RPC method (discoverable via \`homunculus://rpc\`).
- You may freely use other homunculus MCP tools for expression, animation, webviews, and audio.

When delegating, keep the task description specific and self-contained so the Worker can execute without further clarification.

When the user asks about a Worker's progress, use \`get-task-status\` to check before answering.

When the user asks you to talk to another persona, use \`send-to-peer\`. Do not initiate peer conversations on your own — only when the user explicitly asks.
`;

/**
 * Build the system prompt for a Frontman LLM session.
 *
 * Frontmen are conversational only — they never touch files or run commands.
 * Implementation work is delegated to Workers via RPC.
 */
export function buildFrontmanPrompt(persona: Persona): string {
  return `${buildPersonaPrompt(persona)}\n\n${FRONTMAN_INSTRUCTIONS}`;
}

/** Options for building a Frontman runtime. */
export interface FrontmanRuntimeOptions {
  settings: AgentSettings;
  prompt: string;
  apiKey: string | null;
  workDir: string;
  appServerProcess: CodexAppServerProcess;
}

/**
 * Create a Frontman-configured {@link AgentRuntime}.
 *
 * The key difference from Workers: Bash, Write, and Edit are disallowed so
 * that the LLM is forced to delegate implementation work.
 */
export function createFrontmanRuntime(options: FrontmanRuntimeOptions): AgentRuntime {
  const { settings, prompt, apiKey, workDir, appServerProcess } = options;
  const frontmanSettings: AgentSettings = {
    ...settings,
    disallowedTools: [...(settings.disallowedTools ?? []), 'Bash', 'Write', 'Edit'],
  };
  switch (frontmanSettings.runtime) {
    case 'codex':
      return new CodexAppServerRuntime(prompt, frontmanSettings, workDir, appServerProcess);
    case 'sdk':
      if (!apiKey) throw new Error('SDK runtime requires an API key');
      return new ClaudeAgentRuntime(prompt, frontmanSettings, apiKey, workDir);
    default:
      throw new Error(`Runtime "${frontmanSettings.runtime}" is not implemented.`);
  }
}
