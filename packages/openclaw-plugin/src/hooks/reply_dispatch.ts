import type { PluginLogger } from '../deps.js';
import { resolveSession } from '../session-key.js';
import type { SpeakDebouncer } from '../speak-debouncer.js';
import { errorMessage } from '../util/error.js';

export interface ReplyDispatchDeps {
  debouncer: SpeakDebouncer;
  logger: PluginLogger;
}

interface ReplyDispatchEventLite {
  sessionKey?: string;
}

type DispatcherMethod = 'sendFinalReply' | 'sendBlockReply' | 'sendToolResult';

const DISPATCHER_METHODS: readonly DispatcherMethod[] = [
  'sendFinalReply',
  'sendBlockReply',
  'sendToolResult',
];

interface SpeakRoute {
  key: string;
  agentId: string;
}

const wrappedDispatchers = new WeakMap<object, SpeakRoute>();

/**
 * Handler for `reply_dispatch`. Wraps the supplied `ctx.dispatcher` so that
 * every dispatched reply payload is routed through the SpeakDebouncer with
 * the agentId resolved from `event.sessionKey`.
 *
 * # Why dispatcher wrapping
 * OpenClaw's Slack `deliverReplies` does not invoke `runMessageSending`, and
 * the cli-backend bypasses `llm_output` / `before_message_write`. The only
 * reliably reachable point that carries the outbound reply text + an agentId
 * source on the Slack path is `dispatcher.sendFinalReply(payload)`. The
 * dispatcher is a plain object whose methods can be re-assigned in place;
 * the local `dispatcher` reference inside `dispatchReplyFromConfig` and
 * `ctx.dispatcher` point to the same object, so property re-assignment
 * intercepts subsequent calls without breaking the upstream flow.
 *
 * Per-dispatcher routing state lives in a `WeakMap` so that (a) the same
 * dispatcher can be safely re-encountered with a different sessionKey
 * (the latest route wins) and (b) we never mutate upstream objects with
 * branding properties.
 *
 * The handler returns `undefined` unconditionally — it MUST NOT return
 * `{handled: true}` because that would claim dispatch and stop OpenClaw from
 * running the agent.
 */
export function createReplyDispatchHandler(deps: ReplyDispatchDeps) {
  return (event: ReplyDispatchEventLite, ctx: unknown): undefined => {
    if (!event.sessionKey) return undefined;
    const route = resolveSession(event.sessionKey);
    if (!route) {
      deps.logger.debug?.(`[reply_dispatch] unparseable sessionKey=${event.sessionKey}`);
      return undefined;
    }
    const dispatcher = (ctx as { dispatcher?: object } | undefined)?.dispatcher;
    if (!dispatcher) return undefined;

    const alreadyWrapped = wrappedDispatchers.has(dispatcher);
    wrappedDispatchers.set(dispatcher, route);
    if (alreadyWrapped) return undefined;

    for (const method of DISPATCHER_METHODS) {
      wrapDispatcherMethod(dispatcher, method, deps);
    }
    return undefined;
  };
}

function wrapDispatcherMethod(
  dispatcher: object,
  method: DispatcherMethod,
  deps: ReplyDispatchDeps,
): void {
  const target = dispatcher as Record<string, unknown>;
  const original = target[method];
  if (typeof original !== 'function') return;
  const bound = (original as (payload: unknown) => unknown).bind(dispatcher);
  target[method] = (payload: unknown): unknown => {
    pushSpeakSafely(payload, dispatcher, deps);
    return bound(payload);
  };
}

function pushSpeakSafely(payload: unknown, dispatcher: object, deps: ReplyDispatchDeps): void {
  const route = wrappedDispatchers.get(dispatcher);
  if (!route) return;
  try {
    const text = extractText(payload);
    if (!text) return;
    deps.debouncer.push(route.key, route.agentId, text);
  } catch (err) {
    deps.logger.warn(`[reply_dispatch] speak push failed key=${route.key}: ${errorMessage(err)}`);
  }
}

function extractText(payload: unknown): string | null {
  if (!payload || typeof payload !== 'object') return null;
  const text = (payload as { text?: unknown }).text;
  if (typeof text !== 'string') return null;
  if (!text.trim()) return null;
  return text;
}
