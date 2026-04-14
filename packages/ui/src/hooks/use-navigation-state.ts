import { useCallback, useEffect, useState } from 'react';
import { type Webview, type NavigationState } from '@hmcs/sdk';

declare global {
  interface Window {
    cef?: {
      listen: (id: string, callback: (payload: string) => void) => void;
      emit: (id: string, payload: string) => void;
    };
  }
}

/** Return type of the {@link useNavigationState} hook. */
export interface UseNavigationStateResult {
  /** Whether the webview can navigate back in history. */
  canGoBack: boolean;
  /** Whether the webview can navigate forward in history. */
  canGoForward: boolean;
  /** Navigate the webview back in history. */
  navigateBack: () => void;
  /** Navigate the webview forward in history. */
  navigateForward: () => void;
}

/**
 * Subscribes to webview navigation state via CEF push events.
 *
 * Registers a `window.cef.listen` callback for real-time updates and
 * seeds the initial state with an HTTP fetch. Push updates take priority
 * over the initial fetch to avoid race conditions.
 *
 * @param webview - The webview instance to track, or undefined if not yet available
 *
 * @example
 * ```tsx
 * const webview = useMemo(() => Webview.current(), []);
 * const nav = useNavigationState(webview);
 *
 * <Toolbar
 *   title="Agent"
 *   onClose={handleClose}
 *   navigation={{
 *     canGoBack: nav.canGoBack,
 *     canGoForward: nav.canGoForward,
 *     onBack: nav.navigateBack,
 *     onForward: nav.navigateForward,
 *   }}
 * />
 * ```
 */
export function useNavigationState(
  webview: Webview | undefined,
): UseNavigationStateResult {
  const [state, setState] = useState<NavigationState>({ canGoBack: false, canGoForward: false });

  useEffect(() => {
    if (!webview) return;

    // Register push listeners FIRST to avoid race condition with initial fetch
    let pushReceived = false;
    const updateNavState = (payload: string) => {
      try {
        const data = JSON.parse(payload);
        setState({ canGoBack: data.canGoBack, canGoForward: data.canGoForward });
        pushReceived = true;
      } catch { /* ignore malformed payload */ }
    };
    if (window.cef?.listen) {
      // Cross-document navigations (page loads)
      window.cef.listen('loading-state-changed', updateNavState);
      // Same-document navigations (hash changes, pushState)
      window.cef.listen('navigation-state-changed', updateNavState);
    }

    // Fetch initial state (skip if push already arrived)
    webview.navigationState().then((s) => {
      if (!pushReceived) setState(s);
    }).catch(() => { /* ignore — initial state stays { false, false } */ });
  }, [webview]);

  const navigateBack = useCallback(() => { webview?.navigateBack(); }, [webview]);
  const navigateForward = useCallback(() => { webview?.navigateForward(); }, [webview]);

  return { ...state, navigateBack, navigateForward };
}
