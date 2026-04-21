/**
 * Returns a new metadata object with `ttsModName` set to `value`, preserving
 * all other fields. `null` indicates explicit "no TTS". The input object is
 * not mutated. A `null` or `undefined` `existing` (which the server may
 * legitimately return when a persona has no stored metadata) is treated as
 * an empty object.
 */
export function setTtsModName(
  existing: Record<string, unknown> | null | undefined,
  value: string | null,
): Record<string, unknown> {
  return { ...(existing ?? {}), ttsModName: value };
}
