/**
 * Parsed form of an OpenClaw sessionKey.
 *
 * Format: `agent:{agentId}:{channelId}:{kind}:{conversationId}`
 * Example: `agent:elmer:slack:direct:U123`.
 */
export interface ParsedSessionKey {
  agentId: string;
  channelId: string;
  kind: string;
  conversationId: string;
}

/**
 * Parses an OpenClaw sessionKey. Returns `null` when the string is not
 * exactly five non-empty colon-separated parts starting with `agent`.
 */
export function parseSessionKey(sessionKey: string): ParsedSessionKey | null {
  const parts = sessionKey.split(':');
  if (parts.length !== 5) return null;
  const [prefix, agentId, channelId, kind, conversationId] = parts;
  if (prefix !== 'agent') return null;
  if (!agentId || !channelId || !kind || !conversationId) return null;
  return { agentId, channelId, kind, conversationId };
}

/**
 * Builds the `${channelId}:${conversationId}` key used by both `reply_dispatch`
 * (to scope dispatcher-wrapped TTS pushes) and `session_end` (to flush the
 * matching debouncer entry). `channelId` is lowercased to keep the key stable
 * regardless of casing in `parseSessionKey` output.
 */
export function buildCorrelationKey(channelId: string, conversationId: string): string {
  return `${channelId.toLowerCase()}:${conversationId}`;
}

/**
 * Convenience wrapper that parses a sessionKey and derives the correlation
 * key in one call. Returns `null` for unparseable input. Used by every hook
 * that needs both `agentId` and the `channelId:conversationId` key.
 */
export function resolveSession(sessionKey: string): { agentId: string; key: string } | null {
  const parsed = parseSessionKey(sessionKey);
  if (!parsed) return null;
  return {
    agentId: parsed.agentId,
    key: buildCorrelationKey(parsed.channelId, parsed.conversationId),
  };
}
