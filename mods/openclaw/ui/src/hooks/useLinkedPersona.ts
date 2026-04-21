import { type Persona, type PersonaSnapshot, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useState } from 'react';

interface State {
  persona: Persona | null;
  snapshot: PersonaSnapshot | null;
  loading: boolean;
  error: string | null;
}

/**
 * Resolves the persona linked to the current Webview and fetches its snapshot.
 * Returns `{ persona: null, snapshot: null }` when no persona is linked.
 */
export function useLinkedPersona(): State & { refetch: () => void } {
  const [state, setState] = useState<State>({
    persona: null,
    snapshot: null,
    loading: true,
    error: null,
  });

  const load = useCallback(async () => {
    setState((s) => ({ ...s, loading: true, error: null }));
    try {
      const webview = Webview.current();
      if (!webview) {
        setState({
          persona: null,
          snapshot: null,
          loading: false,
          error: 'No webview context available',
        });
        return;
      }
      const persona = await webview.linkedPersona();
      if (!persona) {
        setState({
          persona: null,
          snapshot: null,
          loading: false,
          error: 'This settings UI must be opened from a persona context menu.',
        });
        return;
      }
      const snapshot = await persona.snapshot();
      setState({ persona, snapshot, loading: false, error: null });
    } catch (err) {
      setState({
        persona: null,
        snapshot: null,
        loading: false,
        error: (err as Error).message ?? 'Failed to load persona',
      });
    }
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  return { ...state, refetch: load };
}
