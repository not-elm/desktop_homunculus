/**
 * Result of sanitizing text for TTS synthesis.
 */
export interface SanitizeResult {
  /** Sentence strings suitable for VOICEVOX `speak`. */
  sentences: string[];
  /** Log entries describing each transformation that was applied. */
  log: string[];
}

/**
 * Sanitizes text for TTS by stripping Markdown, replacing URLs,
 * normalizing whitespace, and splitting into sentences.
 *
 * @returns A `SanitizeResult` with `sentences` (speakable strings) and
 *          `log` (descriptions of applied transformations).
 *          Returns empty arrays if nothing speakable remains.
 */
export function sanitizeForTts(text: string): SanitizeResult {
  if (!text) return { sentences: [], log: [] };
  const log: string[] = [];
  let cleaned = text;
  cleaned = applyRule(cleaned, /```[\s\S]*?```/g, "", "fenced-code", log);
  cleaned = applyRule(cleaned, /`[^`]+`/g, "", "inline-code", log);
  cleaned = applyRule(cleaned, /#{1,6}\s/g, "", "heading", log);
  cleaned = applyRule(cleaned, /[*_~]{1,3}/g, "", "emphasis", log);
  cleaned = applyRule(cleaned, /\[([^\]]+)\]\([^)]+\)/g, "$1", "md-link", log);
  cleaned = applyRule(cleaned, /https?:\/\/\S+/g, "URL省略", "bare-url", log);
  cleaned = cleaned.replace(/\n{2,}/g, "\n").trim();
  if (!cleaned) return { sentences: [], log };
  const sentences = cleaned
    .split(/(?<=[。！？\n])/)
    .map((s) => s.trim())
    .filter(Boolean);
  return { sentences, log };
}

function applyRule(
  text: string,
  pattern: RegExp,
  replacement: string,
  ruleName: string,
  log: string[],
): string {
  const matches = text.match(pattern);
  if (!matches) return text;
  for (const match of matches) {
    const truncated = match.length > 80 ? match.slice(0, 80) + "…" : match;
    log.push(`[${ruleName}] removed: ${truncated}`);
  }
  return text.replace(pattern, replacement);
}
