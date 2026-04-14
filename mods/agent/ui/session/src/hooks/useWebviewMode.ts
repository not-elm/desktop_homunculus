import { useEffect } from 'react';

export type WebviewMode = 'expanded' | 'collapsed';

export const PX_PER_UNIT = 800;
export const VIEWPORT_HEIGHT = 600;
export const VIEWPORT_WIDTH_EXPANDED = 750;
export const VIEWPORT_WIDTH_COLLAPSED = 510;

export function useWebviewMode(mode: WebviewMode | null): void {
  useEffect(() => {
    if (!mode) return;
    document.documentElement.dataset.mode = mode;
  }, [mode]);
}
