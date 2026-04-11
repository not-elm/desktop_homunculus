/** A decision can be a simple string ("accept", "decline") or a tagged-union object from the AppServer. */
export type Decision = string | Record<string, unknown>;

/**
 * Events yielded by an AgentRuntime during execution.
 *
 * The generator yields these events to communicate progress, permission/elicitation
 * requests, completion, and errors back to the caller.
 */
export type AgentEvent =
  | { type: 'assistant_message'; text: string }
  | { type: 'tool_use'; tool: string; summary: string }
  | {
      type: 'permission_request';
      requestId: string;
      tool: string;
      input: unknown;
      title?: string;
      description?: string;
      suggestions?: unknown[];
      /** Original JSON-RPC method name from the AppServer protocol. */
      requestMethod?: string;
      /** Available decision options for this request (AppServer only), e.g. `["accept", "acceptForSession", "decline", "cancel"]`. */
      availableDecisions?: Decision[];
    }
  | {
      type: 'elicitation_request';
      requestId: string;
      serverName: string;
      message: string;
      schema?: unknown;
    }
  | { type: 'completed'; sessionId: string }
  | { type: 'error'; message: string };

/**
 * Responses sent back into the generator via `next()`.
 *
 * - `permission`: Approves or denies a tool-use permission request. When `decision` is present, it takes precedence over `approved`.
 * - `elicitation`: Accepts, declines, or cancels an MCP server elicitation request.
 * - `undefined`: No response (used when the event does not require one).
 */
export type AgentResponse =
  | {
      type: 'permission';
      approved: boolean;
      message?: string;
      updatedPermissions?: unknown[];
      /** Rich decision payload for AppServer. When present, takes precedence over `approved`. */
      decision?: string | Record<string, unknown>;
    }
  | {
      type: 'elicitation';
      action: 'accept' | 'decline' | 'cancel';
      values?: Record<string, string>;
      /** Structured content for MCP elicitation (AppServer only). */
      content?: unknown;
    }
  | undefined;

/**
 * Abstract interface for agent runtime backends.
 *
 * Implementations wrap a specific AI SDK (e.g. Claude Agent SDK) and expose
 * a uniform AsyncGenerator-based streaming interface. The caller drives
 * execution by iterating events and sending back responses for interactive
 * requests (permission, elicitation).
 */
export interface AgentRuntime {
  /**
   * Execute an agent turn.
   *
   * @param text - The user's input message.
   * @param sessionId - An existing session to resume, or null to start a new one.
   * @param signal - AbortSignal to cancel execution mid-flight.
   * @returns An async generator yielding AgentEvents, accepting AgentResponses.
   */
  execute(
    text: string,
    sessionId: string | null,
    signal: AbortSignal,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined>;
}
