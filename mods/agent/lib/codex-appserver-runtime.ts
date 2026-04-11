/**
 * AsyncGenerator-based runtime bridging the Codex AppServer JSON-RPC protocol
 * to the unified {@link AgentRuntime} interface.
 *
 * Communicates with the shared {@link CodexAppServerProcess} to start/resume
 * threads, handle server requests (approval, elicitation), and map server
 * notifications to {@link AgentEvent} values.
 *
 * @module
 */

import type { AgentEvent, AgentResponse, AgentRuntime } from './agent-runtime.ts';
import { AsyncQueue } from './async-queue.ts';
import type { CodexAppServerProcess, ThreadHandler } from './codex-appserver-process.ts';
import type {
  CommandExecutionRequestApprovalParams,
  ErrorNotification,
  FileChangeRequestApprovalParams,
  ItemCompletedNotification,
  ItemStartedNotification,
  McpServerElicitationRequestParams,
  PermissionsRequestApprovalParams,
  RequestId,
  ThreadItem,
  ThreadResumeParams,
  ThreadStartParams,
  ThreadStartResponse,
  ToolRequestUserInputParams,
  TurnCompletedNotification,
  TurnStartedNotification,
  TurnStartParams,
} from './codex-appserver-types.ts';
import type { AgentSettings } from './types.ts';

/** Internal message pushed into the event queue by the ThreadHandler. */
type QueueMessage =
  | { kind: 'notification'; method: string; params: unknown }
  | { kind: 'server_request'; method: string; id: RequestId; params: unknown }
  | { kind: 'error'; error: Error };

/**
 * Executes agent turns via the Codex AppServer JSON-RPC protocol.
 *
 * Each call to {@link execute} registers a thread handler on the shared
 * {@link CodexAppServerProcess}, starts or resumes a thread, then runs
 * an event loop that maps server notifications and requests to
 * {@link AgentEvent} values yielded through an AsyncGenerator.
 */
export class CodexAppServerRuntime implements AgentRuntime {
  private readonly prompt: string;
  private readonly settings: AgentSettings;
  private readonly workDir: string;
  private readonly process: CodexAppServerProcess;

  constructor(
    prompt: string,
    settings: AgentSettings,
    workDir: string,
    process: CodexAppServerProcess,
  ) {
    this.prompt = prompt;
    this.settings = settings;
    this.workDir = workDir;
    this.process = process;
  }

  /**
   * Execute an agent turn as an async generator.
   *
   * Drives the AppServer protocol lifecycle: ensures the process is running,
   * starts or resumes a thread, runs a turn, and yields mapped events until
   * the turn completes. Interactive requests (approval, elicitation) are
   * yielded and the caller's response is forwarded back to the AppServer.
   *
   * @param text - The user's input message.
   * @param sessionId - An existing thread ID to resume, or null to start fresh.
   * @param signal - AbortSignal to cancel execution mid-flight.
   */
  async *execute(
    text: string,
    sessionId: string | null,
    signal: AbortSignal,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    await this.process.ensureRunning();

    const threadId = sessionId ? await this.resumeThread(sessionId) : await this.startThread();

    const queue = new AsyncQueue<QueueMessage>();
    const pendingRequests = new Map<string, string>();
    const unregister = this.registerHandler(threadId, queue);

    let turnId: string | undefined;

    const abortHandler = () => {
      this.handleAbort(threadId, turnId, queue, signal);
    };
    signal.addEventListener('abort', abortHandler, { once: true });

    try {
      await this.startTurn(threadId, text);

      yield* this.eventLoop(queue, pendingRequests, threadId, signal, (id) => {
        turnId = id;
      });
    } finally {
      signal.removeEventListener('abort', abortHandler);
      this.cancelAllPending(pendingRequests);
      unregister();
    }
  }

  /**
   * Start a new thread on the AppServer.
   *
   * @returns The new thread ID.
   */
  private async startThread(): Promise<string> {
    const params: ThreadStartParams = {
      cwd: this.workDir,
      baseInstructions: this.prompt,
      personality: 'none',
      sandbox: 'workspace-write',
      experimentalRawEvents: false,
      persistExtendedHistory: false,
      config: {
        mcp_servers: {
          homunculus: { url: 'http://localhost:3100/mcp' },
        },
      },
    };

    const response = await this.process.sendRequest<ThreadStartResponse>('thread/start', params);
    return response.thread.id;
  }

  /**
   * Resume an existing thread, falling back to a new thread on error.
   *
   * @param sessionId - The thread ID to resume.
   * @returns The thread ID (same as sessionId on success, new ID on fallback).
   */
  private async resumeThread(sessionId: string): Promise<string> {
    const params: ThreadResumeParams = {
      threadId: sessionId,
      cwd: this.workDir,
      baseInstructions: this.prompt,
      personality: 'none',
      sandbox: 'workspace-write',
      persistExtendedHistory: false,
      config: {
        mcp_servers: {
          homunculus: { url: 'http://localhost:3100/mcp' },
        },
      },
    };

    try {
      await this.process.sendRequest('thread/resume', params);
      return sessionId;
    } catch (e) {
      console.warn(
        `[codex-appserver-runtime] Failed to resume thread ${sessionId}, starting fresh:`,
        e instanceof Error ? e.message : e,
      );
      return this.startThread();
    }
  }

  /** Register a {@link ThreadHandler} that pushes messages into the queue. */
  private registerHandler(threadId: string, queue: AsyncQueue<QueueMessage>): () => void {
    const handler: ThreadHandler = {
      onServerRequest(method, id, params) {
        queue.push({ kind: 'server_request', method, id, params });
      },
      onServerNotification(method, params) {
        queue.push({ kind: 'notification', method, params });
      },
      onProcessError(error) {
        queue.push({ kind: 'error', error });
      },
    };
    return this.process.registerThread(threadId, handler);
  }

  /** Send a `turn/start` request to begin a new turn. */
  private async startTurn(threadId: string, text: string): Promise<void> {
    const params: TurnStartParams = {
      threadId,
      input: [{ type: 'text', text, text_elements: [] }],
      approvalPolicy: 'on-request',
    };
    await this.process.sendRequest('turn/start', params);
  }

  /**
   * Main event loop: pull messages from the queue and dispatch by kind.
   *
   * Runs until a terminal `turn/completed` notification is received.
   */
  private async *eventLoop(
    queue: AsyncQueue<QueueMessage>,
    pendingRequests: Map<string, string>,
    threadId: string,
    signal: AbortSignal,
    onTurnId: (id: string) => void,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    let done = false;

    while (!done) {
      const msg = await queue.shift(signal);

      if (msg.kind === 'error') {
        yield { type: 'error', message: msg.error.message };
        break;
      }

      if (msg.kind === 'notification') {
        const events = this.handleNotification(msg.method, msg.params, threadId, onTurnId);
        for (const event of events) {
          if (event.type === 'completed' || event.type === 'error') {
            done = true;
          }
          yield event;
        }
        continue;
      }

      yield* this.handleServerRequest(
        msg.method,
        msg.id,
        msg.params,
        pendingRequests,
        queue,
        signal,
      );
    }
  }

  /**
   * Handle a server request: map to event, yield, await response, reply.
   *
   * Auto-approves read-only commands without waiting for caller input.
   */
  private async *handleServerRequest(
    method: string,
    id: RequestId,
    params: unknown,
    pendingRequests: Map<string, string>,
    _queue: AsyncQueue<QueueMessage>,
    _signal: AbortSignal,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    const idKey = String(id);
    pendingRequests.set(idKey, method);

    try {
      if (this.shouldAutoApprove(method, params)) {
        yield* this.autoApproveRequest(id, method, params, pendingRequests);
        return;
      }

      const event = this.mapServerRequestToEvent(method, id, params);
      if (!event) {
        this.autoDeclineRequest(id, method, pendingRequests);
        return;
      }

      const response: AgentResponse | undefined = yield event;
      pendingRequests.delete(idKey);
      this.sendApprovalResponse(id, method, response);
    } catch {
      pendingRequests.delete(idKey);
    }
  }

  /**
   * Auto-approve a command execution and emit a tool_use event for visibility.
   */
  private async *autoApproveRequest(
    id: RequestId,
    _method: string,
    params: unknown,
    pendingRequests: Map<string, string>,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    const cmdParams = params as CommandExecutionRequestApprovalParams;
    yield {
      type: 'tool_use',
      tool: 'bash',
      summary: `[auto] $ ${cmdParams.command ?? ''}`,
    };

    this.process.sendResponse(id, { decision: 'accept' });
    pendingRequests.delete(String(id));
  }

  /** Auto-decline a request the runtime does not handle. */
  private autoDeclineRequest(
    id: RequestId,
    method: string,
    pendingRequests: Map<string, string>,
  ): void {
    this.process.sendErrorResponse(id, -32000, `Unsupported request: ${method}`);
    pendingRequests.delete(String(id));
  }

  /**
   * Map a server notification to zero or more {@link AgentEvent} values.
   *
   * Most notifications produce one event; some (like deltas) are ignored in v1.
   */
  private handleNotification(
    method: string,
    params: unknown,
    threadId: string,
    onTurnId: (id: string) => void,
  ): AgentEvent[] {
    switch (method) {
      case 'turn/started':
        return this.handleTurnStarted(params as TurnStartedNotification, onTurnId);
      case 'turn/completed':
        return this.handleTurnCompleted(params as TurnCompletedNotification, threadId);
      case 'item/started':
        return this.handleItemStarted(params as ItemStartedNotification);
      case 'item/completed':
        return this.handleItemCompleted(params as ItemCompletedNotification);
      case 'error':
        return this.handleErrorNotification(params as ErrorNotification);
      case 'item/agentMessage/delta':
      case 'item/commandExecution/output/delta':
      case 'serverRequest/resolved':
        return [];
      default:
        return [];
    }
  }

  private handleTurnStarted(
    params: TurnStartedNotification,
    onTurnId: (id: string) => void,
  ): AgentEvent[] {
    onTurnId(params.turn.id);
    return [];
  }

  private handleTurnCompleted(params: TurnCompletedNotification, threadId: string): AgentEvent[] {
    const { status, error } = params.turn;

    if (status === 'completed') {
      return [{ type: 'completed', sessionId: threadId }];
    }
    if (status === 'failed') {
      return [{ type: 'error', message: error?.message ?? 'Turn failed' }];
    }
    // "interrupted" — emit error event so the event loop terminates cleanly
    // even for server-initiated interruptions (e.g. context exhaustion, rate limits).
    console.warn(`[codex-appserver-runtime] Turn interrupted by server (status: ${status})`);
    return [{ type: 'error', message: 'Turn was interrupted by the server' }];
  }

  private handleItemStarted(params: ItemStartedNotification): AgentEvent[] {
    switch (params.item.type) {
      case 'commandExecution':
        return [
          {
            type: 'tool_use',
            tool: 'bash',
            summary: `$ ${params.item.command}`,
          },
        ];
      case 'fileChange':
        return [
          {
            type: 'tool_use',
            tool: 'file_change',
            summary: extractFileChangeSummaryFromItem(params.item),
          },
        ];
      default:
        return [];
    }
  }

  private handleItemCompleted(params: ItemCompletedNotification): AgentEvent[] {
    switch (params.item.type) {
      case 'agentMessage':
        return [{ type: 'assistant_message', text: params.item.text }];
      case 'commandExecution':
        return [
          {
            type: 'tool_use',
            tool: 'bash',
            summary: `$ ${params.item.command}`,
          },
        ];
      case 'fileChange':
        return [
          {
            type: 'tool_use',
            tool: 'file_change',
            summary: extractFileChangeSummaryFromItem(params.item),
          },
        ];
      default:
        return [];
    }
  }

  private handleErrorNotification(params: ErrorNotification): AgentEvent[] {
    return [{ type: 'error', message: params.error.message }];
  }

  /**
   * Map a server request method to an {@link AgentEvent}, or null to auto-decline.
   */
  private mapServerRequestToEvent(
    method: string,
    id: RequestId,
    params: unknown,
  ): AgentEvent | null {
    switch (method) {
      case 'item/commandExecution/requestApproval':
        return this.mapCommandApproval(id, params as CommandExecutionRequestApprovalParams, method);
      case 'item/fileChange/requestApproval':
        return this.mapFileChangeApproval(id, params as FileChangeRequestApprovalParams, method);
      case 'item/permissions/requestApproval':
        return this.mapPermissionsApproval(id, params as PermissionsRequestApprovalParams, method);
      case 'mcpServer/elicitation/request':
        return this.mapMcpElicitation(id, params as McpServerElicitationRequestParams);
      case 'item/tool/requestUserInput':
        return this.mapToolUserInput(id, params as ToolRequestUserInputParams);
      case 'item/tool/call':
      case 'account/chatgptAuthTokens/refresh':
        return null;
      case 'applyPatchApproval':
        return this.mapLegacyApproval(id, 'applyPatch', params, method);
      case 'execCommandApproval':
        return this.mapLegacyApproval(id, 'execCommand', params, method);
      default:
        return null;
    }
  }

  private mapCommandApproval(
    id: RequestId,
    params: CommandExecutionRequestApprovalParams,
    method: string,
  ): AgentEvent {
    const defaultDecisions = ['accept', 'acceptForSession', 'decline', 'cancel'];
    return {
      type: 'permission_request',
      requestId: String(id),
      tool: 'bash',
      input: { command: params.command ?? '', cwd: params.cwd },
      title: `Command: ${params.command ?? ''}`,
      description: params.reason ?? undefined,
      requestMethod: method,
      availableDecisions: params.availableDecisions ?? defaultDecisions,
    };
  }

  private mapFileChangeApproval(
    id: RequestId,
    params: FileChangeRequestApprovalParams,
    method: string,
  ): AgentEvent {
    return {
      type: 'permission_request',
      requestId: String(id),
      tool: 'file_change',
      input: { grantRoot: params.grantRoot },
      title: 'File change approval',
      description: params.reason ?? undefined,
      requestMethod: method,
      availableDecisions: ['accept', 'acceptForSession', 'decline', 'cancel'],
    };
  }

  private mapPermissionsApproval(
    id: RequestId,
    params: PermissionsRequestApprovalParams,
    method: string,
  ): AgentEvent {
    return {
      type: 'permission_request',
      requestId: String(id),
      tool: 'permissions',
      input: { permissions: params.permissions },
      title: 'Permission request',
      description: params.reason ?? undefined,
      requestMethod: method,
      availableDecisions: ['accept', 'decline'],
    };
  }

  private mapMcpElicitation(id: RequestId, params: McpServerElicitationRequestParams): AgentEvent {
    if (isMcpToolCallApproval(params)) {
      return this.mapMcpToolCallApproval(id, params);
    }
    return {
      type: 'elicitation_request',
      requestId: String(id),
      serverName: params.serverName,
      message: params.message,
      schema: params.mode === 'form' ? params.requestedSchema : undefined,
    };
  }

  private mapMcpToolCallApproval(
    id: RequestId,
    params: McpServerElicitationRequestParams,
  ): AgentEvent {
    const meta = params._meta as Record<string, unknown> | null;
    const toolDescription = (meta?.tool_description as string) ?? '';
    const toolParams = meta?.tool_params as Record<string, unknown> | undefined;
    return {
      type: 'permission_request',
      requestId: String(id),
      tool: 'mcp',
      input: toolParams ?? {},
      title: params.message,
      description: toolDescription,
      requestMethod: 'mcpServer/elicitation/request',
      availableDecisions: ['accept', 'acceptForSession', 'decline', 'cancel'],
    };
  }

  private mapToolUserInput(id: RequestId, params: ToolRequestUserInputParams): AgentEvent {
    const questionText = params.questions.map((q) => q.question).join('; ');
    return {
      type: 'elicitation_request',
      requestId: String(id),
      serverName: 'tool',
      message: questionText,
    };
  }

  private mapLegacyApproval(
    id: RequestId,
    tool: string,
    params: unknown,
    method: string,
  ): AgentEvent {
    return {
      type: 'permission_request',
      requestId: String(id),
      tool,
      input: params,
      requestMethod: method,
      availableDecisions: ['accept', 'decline'],
    };
  }

  /**
   * Send the caller's {@link AgentResponse} back to the AppServer as a JSON-RPC response.
   */
  private sendApprovalResponse(
    id: RequestId,
    method: string,
    response: AgentResponse | undefined,
  ): void {
    if (!response) {
      this.process.sendResponse(id, { decision: 'decline' });
      return;
    }

    if (response.type === 'permission') {
      this.sendPermissionResult(id, method, response);
    } else if (response.type === 'elicitation') {
      this.sendElicitationResult(id, method, response);
    }
  }

  /** Build and send the JSON-RPC result for a permission response. */
  private sendPermissionResult(
    id: RequestId,
    method: string,
    response: Extract<AgentResponse, { type: 'permission' }>,
  ): void {
    if (method === 'item/permissions/requestApproval') {
      this.process.sendResponse(id, {
        permissions: response.updatedPermissions ?? [],
        scope: 'turn',
      });
      return;
    }

    if (method === 'mcpServer/elicitation/request') {
      const action = response.approved ? 'accept' : 'decline';
      this.process.sendResponse(id, { action, content: null, _meta: null });
      return;
    }

    const decision = response.decision ?? (response.approved ? 'accept' : 'decline');
    this.process.sendResponse(id, { decision });
  }

  /** Build and send the JSON-RPC result for an elicitation response. */
  private sendElicitationResult(
    id: RequestId,
    method: string,
    response: Extract<AgentResponse, { type: 'elicitation' }>,
  ): void {
    if (method === 'item/tool/requestUserInput') {
      this.process.sendResponse(id, { answers: response.values ?? {} });
      return;
    }

    // mcpServer/elicitation/request
    this.process.sendResponse(id, {
      action: response.action,
      content: response.content ?? null,
      _meta: null,
    });
  }

  /**
   * Check whether a command execution request should be auto-approved.
   *
   * Only applies to `item/commandExecution/requestApproval` requests.
   * Tests the command against {@link AgentSettings.commandAutoApprovePatterns}.
   */
  private shouldAutoApprove(method: string, params: unknown): boolean {
    if (method !== 'item/commandExecution/requestApproval') return false;

    const cmdParams = params as CommandExecutionRequestApprovalParams;
    const command = cmdParams.command ?? '';
    return this.settings.commandAutoApprovePatterns.some((pattern) => {
      try {
        return new RegExp(pattern).test(command);
      } catch {
        return false;
      }
    });
  }

  /** Handle abort signal: cancel pending turn then interrupt. */
  private handleAbort(
    threadId: string,
    turnId: string | undefined,
    queue: AsyncQueue<QueueMessage>,
    signal: AbortSignal,
  ): void {
    queue.rejectAll(signal.reason);

    if (turnId) {
      this.process.sendRequest('turn/interrupt', { threadId, turnId }).catch((e) => {
        console.warn(
          '[codex-appserver-runtime] Failed to interrupt turn:',
          e instanceof Error ? e.message : e,
        );
      });
    }
  }

  /** Cancel all tracked pending server requests to prevent AppServer hangs. */
  private cancelAllPending(pendingRequests: Map<string, string>): void {
    for (const [idStr, method] of pendingRequests) {
      try {
        this.process.sendErrorResponse(
          idStr,
          -32800,
          `Request cancelled during cleanup: ${method}`,
        );
      } catch {
        // Process may already be dead; ignore
      }
    }
    pendingRequests.clear();
  }
}

/** Check if an MCP elicitation request is actually a tool call approval. */
function isMcpToolCallApproval(params: McpServerElicitationRequestParams): boolean {
  const meta = params._meta as Record<string, unknown> | null;
  return meta?.codex_approval_kind === 'mcp_tool_call';
}

/** Build a summary string from a fileChange ThreadItem. */
function extractFileChangeSummaryFromItem(
  item: Extract<ThreadItem, { type: 'fileChange' }>,
): string {
  if (item.changes.length === 0) return '(unknown file change)';
  return item.changes.map((c) => `${c.kind}: ${c.path}`).join(', ');
}
