import { resolveSession } from '../session-key.js';
import type { SpeakDebouncer } from '../speak-debouncer.js';

export interface SessionEndDeps {
  debouncer: SpeakDebouncer;
}

interface SessionEndEventLite {
  sessionKey?: string;
}

interface SessionEndCtxLite {
  sessionKey?: string;
}

/**
 * Handler for `session_end`. Force-flushes any pending TTS text for the
 * session's `${channelId}:${conversationId}` key so the last reply does not
 * stay buffered after the session ends.
 */
export function createSessionEndHandler(deps: SessionEndDeps) {
  return async (event: SessionEndEventLite, ctx: SessionEndCtxLite): Promise<void> => {
    const sessionKey = event.sessionKey ?? ctx.sessionKey;
    if (!sessionKey) return;
    const route = resolveSession(sessionKey);
    if (!route) return;
    await deps.debouncer.forceFlush(route.key);
  };
}
