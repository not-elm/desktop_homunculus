import { preferences, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useRef } from 'react';

const POLL_INTERVAL = 1000;
const DEBOUNCE_DELAY = 500;
const PREFS_KEY_PREFIX = 'agent::workers-position::';

/**
 * Poll the current webview position and persist changes to preferences.
 *
 * When the position changes, a debounced save is scheduled after 500ms of
 * stability so that rapid drags don't spam storage.
 */
export function usePositionPersist(personaId: string | null): void {
  const lastPosition = useRef<string>('');
  const debounceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const scheduleSave = useCallback((id: string, translation: unknown) => {
    if (debounceTimer.current) clearTimeout(debounceTimer.current);

    debounceTimer.current = setTimeout(async () => {
      debounceTimer.current = null;
      try {
        await preferences.save(`${PREFS_KEY_PREFIX}${id}`, translation);
      } catch (err) {
        console.error('[workers] Failed to persist position:', err);
      }
    }, DEBOUNCE_DELAY);
  }, []);

  useEffect(() => {
    if (!personaId) return;

    const interval = setInterval(async () => {
      const info = await Webview.current()?.info();
      if (!info?.transform?.translation) return;

      const posKey = JSON.stringify(info.transform.translation);
      if (posKey === lastPosition.current) return;

      lastPosition.current = posKey;
      scheduleSave(personaId, info.transform.translation);
    }, POLL_INTERVAL);

    return () => {
      clearInterval(interval);
      if (debounceTimer.current) clearTimeout(debounceTimer.current);
    };
  }, [personaId, scheduleSave]);
}
