/**
 * Returns a new metadata object with `ttsModName` set to `value`, preserving
 * all other fields. `null` indicates explicit "no TTS". The input object is
 * not mutated.
 */
export function setTtsModName(
  existing: Record<string, unknown>,
  value: string | null,
): Record<string, unknown> {
  return { ...existing, ttsModName: value };
}
