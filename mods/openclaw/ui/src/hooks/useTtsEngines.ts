import { rpc } from '@hmcs/sdk/rpc';
import { useCallback, useEffect, useState } from 'react';

export interface TtsEngine {
  modName: string;
}

interface State {
  data: TtsEngine[];
  loading: boolean;
  error: string | null;
}

export function useTtsEngines(): State & { refetch: () => void } {
  const [state, setState] = useState<State>({
    data: [],
    loading: true,
    error: null,
  });

  const load = useCallback(async () => {
    setState((s) => ({ ...s, loading: true, error: null }));
    try {
      const entries = await rpc.registrations({ category: 'tts' });
      const engines = entries.map((entry) => ({ modName: entry.modName }));
      setState({ data: engines, loading: false, error: null });
    } catch (err) {
      setState({
        data: [],
        loading: false,
        error: (err as Error).message ?? 'Failed to load TTS engines',
      });
    }
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  return { ...state, refetch: load };
}
