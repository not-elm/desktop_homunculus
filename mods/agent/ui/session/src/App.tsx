import { useState, useEffect, useCallback } from "react";
import { signals, Webview } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { SessionView } from "./SessionView";
import { SettingsView } from "./SettingsView";
import { useWebviewMode, type WebviewMode } from "./hooks/useWebviewMode";

type SessionStatus = "idle" | "active";

export function App() {
  const [sessionStatus, setSessionStatus] = useState<SessionStatus | null>(null);
  const [viewOverride, setViewOverride] = useState<"settings" | null>(null);
  const [characterId, setCharacterId] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    resolveCharacterId().then((id) => {
      if (!cancelled) setCharacterId(id);
    });
    return () => { cancelled = true; };
  }, []);

  useEffect(() => {
    if (!characterId) return;
    let cancelled = false;
    fetchSessionStatus(characterId).then((status) => {
      if (!cancelled) setSessionStatus(status);
    });
    return () => { cancelled = true; };
  }, [characterId]);

  useEffect(() => {
    if (!characterId) return;
    const sub = signals.stream<{ characterId: string; state: string }>(
      "agent:status",
      (payload) => {
        if (payload.characterId !== characterId) return;
        const newStatus: SessionStatus = payload.state === "idle" ? "idle" : "active";
        setSessionStatus(newStatus);
        if (newStatus === "idle") setViewOverride(null);
      },
    );
    return () => sub.close();
  }, [characterId]);

  const handleOpenSettings = useCallback(() => {
    setViewOverride("settings");
  }, []);

  const handleStartSession = useCallback(() => {
    setViewOverride(null);
  }, []);

  const handleBack = useCallback(() => {
    setViewOverride(null);
  }, []);

  const mode: WebviewMode = viewOverride ?? (sessionStatus === "idle" ? "settings" : "session");
  useWebviewMode(mode);

  if (sessionStatus === null) return null;

  if (mode === "settings") {
    return (
      <SettingsView
        onStartSession={handleStartSession}
        onBack={handleBack}
        sessionActive={sessionStatus === "active"}
      />
    );
  }

  return <SessionView onOpenSettings={handleOpenSettings} />;
}

async function resolveCharacterId(): Promise<string | null> {
  const vrm = await Webview.current()?.linkedVrm();
  return vrm ? await vrm.name() : null;
}

async function fetchSessionStatus(characterId: string): Promise<SessionStatus> {
  try {
    const result = await rpc.call<{ status: string }>({
      modName: "@hmcs/agent",
      method: "get-session-status",
      body: { characterId },
    });
    return result.status === "active" ? "active" : "idle";
  } catch {
    return "idle";
  }
}
