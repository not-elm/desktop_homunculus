/**
 * Type definitions for the Codex AppServer JSON-RPC 2.0 protocol.
 *
 * The AppServer is started via `codex --app-server --listen stdio://` and
 * communicates over stdin/stdout using newline-delimited JSON-RPC 2.0 messages.
 * This module defines all message envelopes, handshake types, thread/turn
 * lifecycle types, server requests (approval/elicitation), and server
 * notifications used by the protocol.
 *
 * @module
 */

// ---------------------------------------------------------------------------
// JSON-RPC 2.0 Envelope Types
// ---------------------------------------------------------------------------

/** JSON-RPC 2.0 allows both string and number request identifiers. */
export type RequestId = string | number;

/** A JSON-RPC 2.0 request sent by the client to the server. */
export interface JsonRpcRequest {
  jsonrpc: "2.0";
  id: RequestId;
  method: string;
  params?: unknown;
}

/** A JSON-RPC 2.0 notification sent by the client (no `id`, no response expected). */
export interface JsonRpcNotification {
  jsonrpc: "2.0";
  method: string;
  params?: unknown;
}

/** A successful JSON-RPC 2.0 response from the server. */
export interface JsonRpcResponse {
  jsonrpc: "2.0";
  id: RequestId;
  result?: unknown;
  error?: JsonRpcError;
}

/** JSON-RPC 2.0 error object included in error responses. */
export interface JsonRpcError {
  code: number;
  message: string;
  data?: unknown;
}

/** A JSON-RPC 2.0 request sent by the server to the client (ServerRequest). */
export interface JsonRpcServerRequest {
  jsonrpc: "2.0";
  id: RequestId;
  method: string;
  params?: unknown;
}

/** A JSON-RPC 2.0 notification sent by the server to the client (ServerNotification). */
export interface JsonRpcServerNotification {
  jsonrpc: "2.0";
  method: string;
  params?: unknown;
}

/**
 * Union of all message types that can arrive on the server's stdout.
 *
 * Discriminated by presence/absence of `id` and `method`:
 * - `id` + `method` → ServerRequest
 * - `id` + no `method` → Response to a ClientRequest
 * - no `id` + `method` → ServerNotification
 */
export type IncomingMessage =
  | JsonRpcResponse
  | JsonRpcServerRequest
  | JsonRpcServerNotification;

// ---------------------------------------------------------------------------
// Initialize Handshake
// ---------------------------------------------------------------------------

/** Parameters for the `initialize` client request. */
export interface InitializeParams {
  /** Client name reported to the server. */
  clientName: string;
  /** Client version string. */
  clientVersion: string;
  /** Protocol version the client supports. */
  protocolVersion: string;
  /** Client capabilities (currently empty). */
  capabilities?: Record<string, unknown>;
}

/** Server response to the `initialize` request. */
export interface InitializeResponse {
  /** Server name. */
  name: string;
  /** Server version string. */
  version: string;
  /** Protocol version agreed upon. */
  protocolVersion: string;
  /** Server capabilities. */
  capabilities?: Record<string, unknown>;
}

// ---------------------------------------------------------------------------
// Thread / Turn Lifecycle
// ---------------------------------------------------------------------------

/** Personality mode controlling the agent's built-in tone. */
export type Personality = "none" | "friendly" | "pragmatic";

/**
 * Approval policy controlling when the server requests permission.
 *
 * - `"untrusted"`: Always ask for approval before any tool use.
 * - `"on-failure"`: Ask only when a command fails.
 * - `"on-request"`: Ask when the model requests a potentially dangerous action.
 * - `"never"`: Never ask; auto-approve everything.
 * - `{ reject: string }`: Reject with a reason message.
 */
export type AskForApproval =
  | "untrusted"
  | "on-failure"
  | "on-request"
  | "never"
  | { reject: string };

/**
 * Sandbox execution mode for command execution.
 *
 * Controls the isolation level for shell commands run by the agent.
 */
export type SandboxMode =
  | "workspace-read"
  | "workspace-write"
  | "none";

/** Parameters for the `thread/start` client request. */
export interface ThreadStartParams {
  /** Base instructions injected as the system prompt (replaces default persona). */
  baseInstructions?: string;
  /** Developer instructions appended after base instructions. */
  developerInstructions?: string;
  /** Working directory for the thread's command execution. */
  cwd?: string;
  /** Model configuration (model name, provider, etc.). */
  model?: string;
  /** Personality mode. Set to `"none"` when providing custom persona via `baseInstructions`. */
  personality?: Personality;
  /** Sandbox mode for command execution. */
  sandbox?: SandboxMode;
  /** Whether to emit raw protocol events (required for full event mapping). */
  experimentalRawEvents: boolean;
  /** Whether to persist extended history for thread resume. */
  persistExtendedHistory: boolean;
  /** Additional configuration (e.g. MCP servers). */
  config?: Record<string, unknown>;
}

/** Server response to `thread/start`, containing the new thread. */
export interface ThreadStartResponse {
  thread: Thread;
}

/** A thread object representing a conversation session. */
export interface Thread {
  /** Unique thread identifier used for resume and turn operations. */
  id: string;
  /** Thread title (may be auto-generated). */
  title?: string;
}

/** Parameters for the `thread/resume` client request. */
export interface ThreadResumeParams {
  /** ID of the thread to resume. */
  threadId: string;
  /** Override base instructions for the resumed thread. */
  baseInstructions?: string;
  /** Override developer instructions for the resumed thread. */
  developerInstructions?: string;
  /** Override working directory for the resumed thread. */
  cwd?: string;
  /** Override model for the resumed thread. */
  model?: string;
  /** Override personality for the resumed thread. */
  personality?: Personality;
  /** Override sandbox mode for the resumed thread. */
  sandbox?: SandboxMode;
  /** Whether to emit raw protocol events. */
  experimentalRawEvents?: boolean;
  /** Whether to persist extended history. */
  persistExtendedHistory?: boolean;
  /** Override configuration. */
  config?: Record<string, unknown>;
}

/** User input message for starting a turn. */
export interface UserInput {
  /** The user's text message. */
  text: string;
}

/** Parameters for the `turn/start` client request. */
export interface TurnStartParams {
  /** Thread to run the turn in. */
  threadId: string;
  /** User input for this turn. */
  input: UserInput;
  /** Approval policy for this turn (persists for subsequent turns). */
  approvalPolicy?: AskForApproval;
}

/** Parameters for the `turn/interrupt` client request. */
export interface TurnInterruptParams {
  /** Thread containing the turn to interrupt. */
  threadId: string;
  /** ID of the specific turn to interrupt. */
  turnId: string;
}

// ---------------------------------------------------------------------------
// ServerRequest: Approval Types
// ---------------------------------------------------------------------------

/**
 * Decision for command execution approval.
 *
 * Simple string variants handle common cases. Object variants allow
 * policy amendments (v2 feature).
 */
export type CommandExecutionApprovalDecision =
  | "accept"
  | "acceptForSession"
  | "decline"
  | "cancel"
  | { acceptWithExecPolicyAmendment: string }
  | { applyNetworkPolicyAmendment: string };

/** Parameters for `item/commandExecution/requestApproval` ServerRequest. */
export interface CommandExecutionRequestApprovalParams {
  /** Unique approval ID for correlating concurrent requests. */
  approvalId: string;
  /** Thread this approval belongs to. */
  threadId: string;
  /** Shell command requesting approval. */
  command: string;
  /** Working directory for the command. */
  cwd?: string;
  /** Reason the server is requesting approval. */
  reason?: string;
}

/**
 * Decision for file change approval.
 */
export type FileChangeApprovalDecision =
  | "accept"
  | "acceptForSession"
  | "decline"
  | "cancel";

/** Parameters for `item/fileChange/requestApproval` ServerRequest. */
export interface FileChangeRequestApprovalParams {
  /** Unique approval ID for correlating concurrent requests. */
  approvalId: string;
  /** Thread this approval belongs to. */
  threadId: string;
  /** File path being modified. */
  path: string;
  /** Kind of change (create, modify, delete). */
  kind?: string;
  /** Unified diff of the change. */
  diff?: string;
  /** Reason the server is requesting approval. */
  reason?: string;
}

/** Parameters for `item/permissions/requestApproval` ServerRequest. */
export interface PermissionsRequestApprovalParams {
  /** Unique approval ID for correlating concurrent requests. */
  approvalId: string;
  /** Thread this approval belongs to. */
  threadId: string;
  /** Description of the permissions being requested. */
  description?: string;
  /** Specific permissions requested. */
  permissions?: unknown[];
}

// ---------------------------------------------------------------------------
// ServerRequest: Elicitation Types
// ---------------------------------------------------------------------------

/** Action for MCP server elicitation responses. */
export type McpServerElicitationAction = "accept" | "decline" | "cancel";

/** Parameters for `mcpServer/elicitation/request` ServerRequest. */
export interface McpServerElicitationRequestParams {
  /** Thread this elicitation belongs to. */
  threadId: string;
  /** MCP server name requesting elicitation. */
  serverName: string;
  /** Message displayed to the user. */
  message: string;
  /** JSON Schema for the expected response. */
  schema?: unknown;
}

/** Parameters for `item/tool/requestUserInput` ServerRequest. */
export interface ToolRequestUserInputParams {
  /** Thread this request belongs to. */
  threadId: string;
  /** Tool name requesting user input. */
  toolName: string;
  /** Questions to present to the user. */
  questions: ToolInputQuestion[];
}

/** A single question in a tool user-input request. */
export interface ToolInputQuestion {
  /** Question identifier. */
  id: string;
  /** Question text. */
  text: string;
  /** Whether an answer is required. */
  required?: boolean;
  /** Default value. */
  defaultValue?: string;
}

// ---------------------------------------------------------------------------
// ServerNotification Types
// ---------------------------------------------------------------------------

/** Notification emitted when a new turn begins executing. */
export interface TurnStartedNotification {
  /** Thread the turn belongs to. */
  threadId: string;
  /** Unique turn identifier (used for interrupt). */
  turnId: string;
}

/** Completion status of a finished turn. */
export type TurnStatus = "completed" | "interrupted" | "failed";

/** Notification emitted when a turn finishes. */
export interface TurnCompletedNotification {
  /** Thread the turn belongs to. */
  threadId: string;
  /** Completed turn details. */
  turn: {
    /** Turn identifier. */
    id: string;
    /** Final status of the turn. */
    status: TurnStatus;
    /** Error message if status is `"failed"`. */
    error?: string;
  };
}

/** Notification emitted when a new item starts processing. */
export interface ItemStartedNotification {
  /** Thread the item belongs to. */
  threadId: string;
  /** Unique item identifier. */
  itemId: string;
  /** Item type (e.g. `"agent_message"`, `"command_execution"`, `"file_change"`, `"mcp_tool_call"`). */
  itemType: string;
  /** Item-specific metadata. */
  metadata?: Record<string, unknown>;
}

/** Notification emitted when an item completes. */
export interface ItemCompletedNotification {
  /** Thread the item belongs to. */
  threadId: string;
  /** Item identifier. */
  itemId: string;
  /** Item type. */
  itemType: string;
  /** Completed item data (structure depends on `itemType`). */
  item?: Record<string, unknown>;
}

/** Notification emitted as the agent streams message text. */
export interface AgentMessageDeltaNotification {
  /** Thread the message belongs to. */
  threadId: string;
  /** Item identifier for the agent message. */
  itemId: string;
  /** Incremental text delta. */
  delta: string;
}

/** Notification emitted as command execution produces output. */
export interface CommandExecutionOutputDeltaNotification {
  /** Thread the command belongs to. */
  threadId: string;
  /** Item identifier for the command execution. */
  itemId: string;
  /** Incremental output delta (stdout/stderr). */
  delta: string;
  /** Output stream: `"stdout"` or `"stderr"`. */
  stream?: "stdout" | "stderr";
}

/** Notification emitted when a pending ServerRequest is resolved (e.g. on interrupt). */
export interface ServerRequestResolvedNotification {
  /** The request ID that was resolved. */
  requestId: RequestId;
  /** Resolution reason (e.g. `"interrupted"`, `"cancelled"`). */
  reason?: string;
}

/** Notification emitted when the server encounters an error. */
export interface ErrorNotification {
  /** Error message. */
  message: string;
  /** Error code. */
  code?: number;
  /** Additional error data. */
  data?: unknown;
}
