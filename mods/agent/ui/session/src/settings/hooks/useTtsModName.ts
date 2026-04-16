import { Persona as SdkPersona, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useState } from 'react';

export interface UseTtsModNameReturn {
  /** Currently selected TTS MOD name, or null if disabled. undefined while loading. */
  value: string | null | undefined;
  /** Update the selected TTS engine. Pass null to disable. */
  onChange: (modName: string | null) => Promise<void>;
  /** Whether the initial load is in progress. */
  loading: boolean;
}

export function useTtsModName(): UseTtsModNameReturn {
  const [value, setValue] = useState<string | null | undefined>(undefined);
  const [loading, setLoading] = useState(true);
  const [personaId, setPersonaId] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const wv = await Webview.current();
        const p = wv ? await wv.linkedPersona() : null;
        if (cancelled) return;
        const id = p ? p.id : null;
        setPersonaId(id);

        const metadata = id
          ? await SdkPersona.load(id).then((persona) => persona.metadata())
          : undefined;
        if (cancelled) return;

        if (metadata && metadata.ttsModName !== undefined) {
          setValue(metadata.ttsModName as string | null);
        } else {
          setValue(null);
        }
      } catch (e) {
        console.error('Failed to load TTS mod name:', e);
        setValue(null);
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const onChange = useCallback(
    async (modName: string | null) => {
      setValue(modName);
      if (!personaId) return;
      try {
        const p = await SdkPersona.load(personaId);
        const existing = await p.metadata();
        await p.setMetadata({ ...existing, ttsModName: modName });
      } catch (e) {
        console.error('Failed to save TTS mod name:', e);
      }
    },
    [personaId],
  );

  return { value, onChange, loading };
}
