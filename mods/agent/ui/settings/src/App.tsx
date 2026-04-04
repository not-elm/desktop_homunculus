import { useState } from "react";
import type { MainPanelContent } from "./types";

export function App() {
  const [content, setContent] = useState<MainPanelContent>({ kind: "empty" });

  return (
    <div style={{ width: "var(--hud-settings-width)", height: "var(--hud-settings-height)", background: "var(--hud-settings-chrome-bg)", borderRadius: "var(--hud-settings-chrome-radius)", display: "flex", color: "var(--hud-text-primary)", fontFamily: "var(--hud-font-family)" }}>
      <div style={{ width: "var(--hud-settings-sidebar-width)", background: "var(--hud-settings-sidebar-tree)" }}>Sidebar</div>
      <div style={{ flex: 1, background: "var(--hud-settings-main)", padding: "20px" }}>Main: {content.kind}</div>
    </div>
  );
}
