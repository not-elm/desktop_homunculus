import { useEffect } from "react";
import { Webview } from "@hmcs/sdk";

export type WebviewMode = "expanded" | "collapsed";

const EXPANDED_GEOMETRY = {
  size: [0.85, 0.8] as [number, number],
  viewportSize: [640, 500] as [number, number],
  offset: [1.2, -0.3] as [number, number],
};

const COLLAPSED_GEOMETRY = {
  size: [0.6, 0.8] as [number, number],
  viewportSize: [400, 500] as [number, number],
  offset: [0.8, -0.5] as [number, number],
};

export function useWebviewMode(mode: WebviewMode | null): void {
  useEffect(() => {
    if (!mode) return;
    document.documentElement.dataset.mode = mode;

    const wv = Webview.current();
    if (!wv) return;
    const geom = mode === "expanded" ? EXPANDED_GEOMETRY : COLLAPSED_GEOMETRY;
    void applyGeometry(wv, geom);
  }, [mode]);
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
