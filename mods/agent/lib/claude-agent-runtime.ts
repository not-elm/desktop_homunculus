import type {
  Options,
  PermissionResult,
  PermissionUpdate,
  Query,
  SDKMessage,
} from '@anthropic-ai/claude-agent-sdk';
import { query } from '@anthropic-ai/claude-agent-sdk';
import type { AgentEvent, AgentResponse, AgentRuntime } from './agent-runtime.ts';
import { AsyncQueue, Deferred } from './async-queue.ts';
import type { AgentSettings } from './types.ts';

/** Item enqueued by canUseTool; awaited by mergeStreams. */
interface PermissionQueueItem {
  requestId: string;
  tool: string;
  input: unknown;
  title?: string;
  description?: string;
  suggestions?: PermissionUpdate[];
  deferred: Deferred<PermissionResult>;
}

/** Sentinel value to distinguish permission items from SDK messages in a race. */
const PERMISSION_TAG = Symbol('permission');

type RaceResult =
  | { tag: 'sdk'; value: IteratorResult<SDKMessage, void> }
  | { tag: typeof PERMISSION_TAG; value: PermissionQueueItem };

/**
 * Wraps the Claude Agent SDK `query()` in the AgentRuntime interface.
 *
 * Bridges the SDK's `canUseTool` callback into the async generator event stream
 * so the caller can approve or deny tool use via `generator.next(response)`.
 */
export class ClaudeAgentRuntime implements AgentRuntime {
  constructor(
    private readonly prompt: string,
    private readonly settings: AgentSettings,
    private readonly apiKey: string,
    private readonly workDir: string,
  ) {}

  async *execute(
    text: string,
    sessionId: string | null,
    signal: AbortSignal,
  ): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
    const permQueue = new AsyncQueue<PermissionQueueItem>();
    const canUseTool = createCanUseToolHandler(permQueue);
    const options = buildQueryOptions(
      this.prompt,
      this.settings,
      this.apiKey,
      this.workDir,
      sessionId,
      canUseTool,
    );
    const handle = query({ prompt: text, options });

    const onAbort = () => handle.close();
    signal.addEventListener('abort', onAbort, { once: true });

    try {
      yield* mergeStreams(handle, permQueue, signal);
    } finally {
      signal.removeEventListener('abort', onAbort);
      permQueue.rejectAll(new Error('stream closed'));
      handle.close();
    }
  }
}

/**
 * Creates a `canUseTool` callback that enqueues permission requests
 * into the AsyncQueue and returns a deferred promise for the result.
 */
function createCanUseToolHandler(
  permQueue: AsyncQueue<PermissionQueueItem>,
): Options['canUseTool'] {
  return (toolName, input, options) => {
    console.log(`[agent] canUseTool called: ${toolName} (${options.toolUseID})`);
    const deferred = new Deferred<PermissionResult>();
    permQueue.trackDeferred(deferred);

    permQueue.push({
      requestId: options.toolUseID,
      tool: toolName,
      input,
      title: options.title,
      description: options.description,
      suggestions: options.suggestions,
      deferred,
    });

    return deferred.promise;
  };
}

/**
 * Merges the SDK message stream with the permission queue into a single
 * async generator of AgentEvents. Permission events pause and wait for
 * a response via `generator.next(response)`.
 */
async function* mergeStreams(
  handle: Query,
  permQueue: AsyncQueue<PermissionQueueItem>,
  signal: AbortSignal,
): AsyncGenerator<AgentEvent, void, AgentResponse | undefined> {
  const sdkIter = handle[Symbol.asyncIterator]();
  let sdkNext = wrapSdkNext(sdkIter.next());
  let permNext = wrapPermNext(permQueue.shift(signal));

  try {
    while (!signal.aborted) {
      const result = await Promise.race([sdkNext, permNext]);

      if (result.tag === 'sdk') {
        if (result.value.done) return;
        const event = mapSdkMessageToEvent(result.value.value);
        if (event) yield event;
        sdkNext = wrapSdkNext(sdkIter.next());
      } else {
        const response = yield buildPermissionEvent(result.value);
        resolvePermissionDeferred(result.value.deferred, response);
        permNext = wrapPermNext(permQueue.shift(signal));
      }
    }
  } finally {
    permQueue.rejectAll(new Error('stream closed'));
    await sdkIter.return?.();
  }
}

/** Tags an SDK iterator result for discrimination in Promise.race. */
function wrapSdkNext(promise: Promise<IteratorResult<SDKMessage, void>>): Promise<RaceResult> {
  return promise.then((value): RaceResult => ({ tag: 'sdk', value }));
}

/** Tags a permission queue item for discrimination in Promise.race. Swallows rejection to prevent unhandled errors during cleanup. */
function wrapPermNext(promise: Promise<PermissionQueueItem>): Promise<RaceResult> {
  const wrapped = promise.then((value): RaceResult => ({ tag: PERMISSION_TAG, value }));
  wrapped.catch(() => {});
  return wrapped;
}

/** Builds the AgentEvent for a permission request. */
function buildPermissionEvent(item: PermissionQueueItem): AgentEvent {
  return {
    type: 'permission_request',
    requestId: item.requestId,
    tool: item.tool,
    input: item.input,
    title: item.title,
    description: item.description,
    suggestions: item.suggestions,
  };
}

/** Resolves the deferred with the caller's permission decision. */
function resolvePermissionDeferred(
  deferred: Deferred<PermissionResult>,
  response: AgentResponse | undefined,
): void {
  deferred.resolve(mapResponseToPermissionResult(response));
}

/** Maps an AgentResponse to the SDK's PermissionResult. */
function mapResponseToPermissionResult(response: AgentResponse | undefined): PermissionResult {
  if (response?.type === 'permission' && response.approved) {
    return {
      behavior: 'allow',
      updatedPermissions: response.updatedPermissions as PermissionUpdate[] | undefined,
    };
  }
  const message =
    response?.type === 'permission' ? (response.message ?? 'User denied') : 'No response';
  return { behavior: 'deny', message };
}

/**
 * Maps an SDKMessage to an AgentEvent.
 * Returns null for message types that should be silently skipped.
 */
function mapSdkMessageToEvent(msg: SDKMessage): AgentEvent | null {
  switch (msg.type) {
    case 'assistant':
      return mapAssistantMessage(msg);
    case 'tool_use_summary':
      return { type: 'tool_use', tool: '', summary: msg.summary };
    case 'result':
      return mapResultMessage(msg);
    default:
      return null;
  }
}

/** Extracts text content from an assistant message's content blocks. */
function mapAssistantMessage(msg: { message: { content: unknown[] } }): AgentEvent | null {
  const blocks = msg.message.content as Array<{ type: string; text?: string }>;
  const text = blocks
    .filter((b) => b.type === 'text' && b.text)
    .map((b) => b.text as string)
    .join('');
  if (!text) return null;
  return { type: 'assistant_message', text };
}

/** Maps a result message to a completed or error event. */
function mapResultMessage(msg: {
  subtype: string;
  session_id: string;
  is_error: boolean;
}): AgentEvent {
  if (msg.is_error) {
    return { type: 'error', message: `Agent ended with error (${msg.subtype})` };
  }
  return { type: 'completed', sessionId: msg.session_id };
}

/** Constructs the SDK Options for a query call. */
function buildQueryOptions(
  prompt: string,
  settings: AgentSettings,
  apiKey: string,
  workDir: string,
  sessionId: string | null,
  canUseTool: Options['canUseTool'],
): Options {
  const options: Options = {
    systemPrompt: prompt,
    cwd: workDir,
    mcpServers: {
      homunculus: { type: 'http', url: 'http://localhost:3100/mcp' },
    },
    hooks: {
      PreToolUse: [buildReadOnlyHook()],
    },
    settings: { permissions: { allow: ['Bash(*)', 'Write(*)', 'Edit(*)'] } },
    allowedTools: settings.allowList.length > 0 ? settings.allowList : undefined,
    disallowedTools: settings.disallowedTools,
    canUseTool,
    env: {
      ...process.env,
      NODE_OPTIONS: '',
      ANTHROPIC_API_KEY: apiKey,
      DEBUG_CLAUDE_AGENT_SDK: '1',
    },
    maxTurns: 100,
    stderr: (data: string) => {
      for (const line of data.split('\n')) {
        if (line.trim()) console.log(`[sdk-stderr] ${line}`);
      }
    },
  };
  if (settings.claudeModel) options.model = settings.claudeModel;
  if (sessionId) options.resume = sessionId;
  return options;
}

/** Hook that auto-allows read-only tools without prompting. */
function buildReadOnlyHook() {
  return {
    matcher: '^(Read|Glob|Grep|mcp__homunculus__list|mcp__homunculus__get)',
    hooks: [
      () =>
        Promise.resolve({
          hookSpecificOutput: {
            hookEventName: 'PreToolUse' as const,
            permissionDecision: 'allow' as const,
          },
        }),
    ],
  };
}
