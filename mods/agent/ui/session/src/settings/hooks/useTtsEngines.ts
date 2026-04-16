import { rpc } from '@hmcs/sdk/rpc';
import { useEffect, useState } from 'react';

export interface TtsEngine {
  modName: string;
  description?: string;
}

export function useTtsEngines() {
  const [engines, setEngines] = useState<TtsEngine[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const entries = await rpc.registrations({ category: 'tts' });
        if (cancelled) return;
        setEngines(
          entries.map((e) => ({
            modName: e.modName,
            description: e.description,
          })),
        );
      } catch (err) {
        console.error('Failed to load TTS engines:', err);
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  return { engines, loading };
}
