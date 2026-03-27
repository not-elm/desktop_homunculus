/**
 * Sanitizes text for TTS by stripping Markdown, replacing URLs,
 * normalizing whitespace, and splitting into sentences.
 *
 * @returns An array of sentence strings suitable for VOICEVOX `speak`.
 *          Returns empty array if nothing speakable remains.
 */
export function sanitizeForTts(text: string): string[] {
  let cleaned = text;
  cleaned = cleaned.replace(/```[\s\S]*?```/g, "");
  cleaned = cleaned.replace(/`[^`]+`/g, "");
  cleaned = cleaned.replace(/#{1,6}\s/g, "");
  cleaned = cleaned.replace(/[*_~]{1,3}/g, "");
  cleaned = cleaned.replace(/\[([^\]]+)\]\([^)]+\)/g, "$1");
  cleaned = cleaned.replace(/https?:\/\/\S+/g, "URL省略");
  cleaned = cleaned.replace(/\n{2,}/g, "\n").trim();
  if (!cleaned) return [];
  return cleaned
    .split(/(?<=[。！？\n])/)
    .map((s) => s.trim())
    .filter(Boolean);
}
