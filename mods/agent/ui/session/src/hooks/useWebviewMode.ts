import { Webview } from '@hmcs/sdk';
import { useEffect, useRef } from 'react';

export type WebviewMode = 'expanded' | 'collapsed';

const PX_PER_UNIT = 800;
const VIEWPORT_HEIGHT = 800;

export const VIEWPORT_WIDTH_EXPANDED = 1000;
export const VIEWPORT_WIDTH_COLLAPSED = 680;

function patchWebviewSize(viewportWidth: number): void {
  const size: [number, number] = [viewportWidth / PX_PER_UNIT, VIEWPORT_HEIGHT / PX_PER_UNIT];
  const viewportSize: [number, number] = [viewportWidth, VIEWPORT_HEIGHT];
  Webview.current()?.patch({ size, viewportSize }).catch(() => {});
}

export function useWebviewMode(mode: WebviewMode | null): void {
  const prevMode = useRef<WebviewMode | null>(null);

  useEffect(() => {
    if (!mode) return;
    document.documentElement.dataset.mode = mode;

    if (mode !== prevMode.current) {
      const width = mode === 'expanded' ? VIEWPORT_WIDTH_EXPANDED : VIEWPORT_WIDTH_COLLAPSED;
      patchWebviewSize(width);
    }
    prevMode.current = mode;
  }, [mode]);
}
