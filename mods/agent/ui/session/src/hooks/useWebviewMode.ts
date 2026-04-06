import { useEffect } from "react";
import { Webview } from "@hmcs/sdk";

export type WebviewMode = "expanded" | "collapsed";

const VIEW_GEOMETRY = {
  size: [1.07, 0.8] as [number, number],
  viewportSize: [800, 500] as [number, number],
  offset: [1.2, -0.3] as [number, number],
};

export function useWebviewMode(mode: WebviewMode | null): void {
  useEffect(() => {
    if (!mode) return;
    document.documentElement.dataset.mode = mode;
  }, [mode]);

  // Apply geometry once on mount
  useEffect(() => {
    const wv = Webview.current();
    if (!wv) return;
    void applyGeometry(wv, VIEW_GEOMETRY);
  }, []);
}

async function applyGeometry(
  wv: Webview,
  geom: {
    size: [number, number];
    viewportSize: [number, number];
    offset: [number, number];
  },
): Promise<void> {
  await wv.setSize(geom.size);
  await wv.setViewportSize(geom.viewportSize);
  await wv.setOffset(geom.offset);
}
