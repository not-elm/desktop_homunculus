import { useCallback, useEffect, useRef, useState } from 'react';
import { Webview } from '@hmcs/sdk';
import type { NavigationState } from '@hmcs/sdk';

interface UseNavigationStateResult {
  canGoBack: boolean;
  canGoForward: boolean;
  navigateBack: () => void;
  navigateForward: () => void;
}

export function useNavigationState(
  webview: Webview | undefined,
  intervalMs = 500,
): UseNavigationStateResult {
  const [state, setState] = useState<NavigationState>({ canGoBack: false, canGoForward: false });
  const stoppedRef = useRef(false);

  const fetchState = useCallback(async () => {
    if (!webview || stoppedRef.current) return;
    try {
      const newState = await webview.navigationState();
      setState(newState);
    } catch {
      stoppedRef.current = true;
    }
  }, [webview]);

  useEffect(() => {
    if (!webview) return;
    stoppedRef.current = false;
    fetchState();
    const id = setInterval(fetchState, intervalMs);
    return () => clearInterval(id);
  }, [webview, intervalMs, fetchState]);

  const navigateBack = useCallback(async () => {
    if (!webview) return;
    await webview.navigateBack();
    const newState = await webview.navigationState();
    setState(newState);
  }, [webview]);

  const navigateForward = useCallback(async () => {
    if (!webview) return;
    await webview.navigateForward();
    const newState = await webview.navigationState();
    setState(newState);
  }, [webview]);

  return { ...state, navigateBack, navigateForward };
}
