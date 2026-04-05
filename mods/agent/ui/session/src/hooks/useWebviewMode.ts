import { useEffect } from "react";
import { Webview } from "@hmcs/sdk";

export type WebviewMode = "settings" | "session";

const SETTINGS_GEOMETRY = {
  size: [1.3333, 1.0] as [number, number],
  viewportSize: [1200, 900] as [number, number],
  offset: [-0.6, -0.3, -10.0] as [number, number, number],
};

const SESSION_GEOMETRY = {
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
    const geom = mode === "settings" ? SETTINGS_GEOMETRY : SESSION_GEOMETRY;
    void applyGeometry(wv, geom);
  }, [mode]);
}

async function applyGeometry(
  wv: Webview,
  geom: {
    size: [number, number];
    viewportSize: [number, number];
    offset: [number, number] | [number, number, number];
  },
): Promise<void> {
  await wv.setSize(geom.size);
  await wv.setViewportSize(geom.viewportSize);
  await wv.setOffset(geom.offset);
}
