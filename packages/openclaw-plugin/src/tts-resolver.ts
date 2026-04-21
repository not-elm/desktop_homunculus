import { Persona } from '@hmcs/sdk';

/**
 * Resolve the TTS MOD name for a persona from its metadata.
 *
 * Returns `null` when no TTS engine is selected (including any read/parse
 * failure). Callers must treat `null` as "skip speech output".
 */
export async function resolveTtsModName(personaId: string): Promise<string | null> {
  try {
    const persona = await Persona.load(personaId);
    const meta = (await persona.metadata()) as { ttsModName?: unknown };
    const value = meta.ttsModName;
    return typeof value === 'string' && value.length > 0 ? value : null;
  } catch {
    return null;
  }
}
