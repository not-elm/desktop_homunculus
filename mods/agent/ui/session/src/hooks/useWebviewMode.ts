import { useEffect } from 'react';

export type WebviewMode = 'expanded' | 'collapsed';

export function useWebviewMode(mode: WebviewMode | null): void {
  useEffect(() => {
    if (!mode) return;
    document.documentElement.dataset.mode = mode;
  }, [mode]);
}
