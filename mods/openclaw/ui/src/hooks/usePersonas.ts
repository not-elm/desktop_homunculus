import { Persona, type PersonaSnapshot } from '@hmcs/sdk';
import { useCallback, useEffect, useState } from 'react';

interface State {
  data: PersonaSnapshot[];
  loading: boolean;
  error: string | null;
}

export function usePersonas(): State & { refetch: () => void } {
  const [state, setState] = useState<State>({
    data: [],
    loading: true,
    error: null,
  });

  const load = useCallback(async () => {
    setState((s) => ({ ...s, loading: true, error: null }));
    try {
      const data = await Persona.list();
      setState({ data, loading: false, error: null });
    } catch (err) {
      setState({
        data: [],
        loading: false,
        error: (err as Error).message ?? 'Failed to load personas',
      });
    }
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  return { ...state, refetch: load };
}
