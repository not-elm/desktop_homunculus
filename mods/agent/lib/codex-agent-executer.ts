import {
  Codex,
  type ThreadOptions,
  type ThreadEvent,
  type ThreadItem,
} from "@openai/codex-sdk";
import type {
  AIAgentExecuter,
  AgentEvent,
  AgentResponse,
} from "./ai-agent-executer.ts";
import { buildCharacterPrompt } from "./prompt.ts";
import type { AgentSettings, Persona } from "./types.ts";

/**
 * Wraps the OpenAI Codex SDK in the AIAgentExecuter interface.
 *
 * Uses `approvalPolicy: "never"` for fully automatic tool approval
 * and `sandboxMode: "workspace-write"` for safety. Authentication
 * relies on prior `codex login` (no API key needed).
 */
export class CodexAgentExecuter implements AIAgentExecuter {
  private readonly codex: Codex;
  private readonly threadOptions: ThreadOptions;

  constructor(
    persona: Persona,
    private readonly settings: AgentSettings,
    workDir: string,
  ) {
    this.codex = new Codex({
      config: {
        instructions: buildCharacterPrompt(persona),
        mcp_servers: {
          homunculus: { url: "http://localhost:3100/mcp" },
        },
      },
    });
    this.threadOptions = {
      model: settings.model || undefined,
      sandboxMode: "workspace-write",
      workingDirectory: workDir,
      approvalPolicy: "never",
    };
  }

  async *execute(
    text: string,
    sessionId: string | null,
    signal: AbortSignal,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    const thread = sessionId
      ? this.codex.resumeThread(sessionId, this.threadOptions)
      : this.codex.startThread(this.threadOptions);

    const { events } = await thread.runStreamed(text, { signal });
    let currentSessionId = sessionId ?? thread.id ?? null;

    for await (const event of events) {
      if (event.type === "thread.started") {
        currentSessionId = event.thread_id;
      }
      const mapped = mapThreadEvent(event, currentSessionId);
      if (mapped) yield mapped;
    }
  }
}

/** Maps a Codex ThreadEvent to an AgentEvent, or null to skip. */
function mapThreadEvent(
  event: ThreadEvent,
  sessionId: string | null,
): AgentEvent | null {
  switch (event.type) {
    case "item.started":
      return mapItemStarted(event.item);
    case "item.completed":
      return mapItemCompleted(event.item);
    case "turn.completed":
      return { type: "completed", sessionId: sessionId ?? "" };
    case "turn.failed":
      return { type: "error", message: event.error.message };
    case "error":
      return { type: "error", message: event.message };
    default:
      return null;
  }
}

/** Maps an item.started event to a tool_use AgentEvent for real-time feedback. */
function mapItemStarted(item: ThreadItem): AgentEvent | null {
  switch (item.type) {
    case "command_execution":
      return { type: "tool_use", tool: "bash", summary: `$ ${item.command}` };
    case "mcp_tool_call":
      return {
        type: "tool_use",
        tool: item.tool,
        summary: `${item.server}:${item.tool}`,
      };
    default:
      return null;
  }
}

/** Maps an item.completed event to an AgentEvent. */
function mapItemCompleted(item: ThreadItem): AgentEvent | null {
  switch (item.type) {
    case "agent_message":
      return { type: "assistant_message", text: item.text };
    case "file_change": {
      const files = item.changes
        .map((c) => `${c.kind}: ${c.path}`)
        .join(", ");
      return { type: "tool_use", tool: "file_change", summary: files };
    }
    case "error":
      return { type: "error", message: item.message };
    default:
      return null;
  }
}
