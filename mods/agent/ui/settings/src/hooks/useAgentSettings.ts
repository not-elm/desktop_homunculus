import { useCallback, useEffect, useState } from "react";
import { preferences, audio, Webview } from "@hmcs/sdk";

export interface WorkingDirectories {
  paths: string[];
  default: number;
}

export interface AgentSettings {
  wakeWords: string[];
  shutdownWords: string[];
  greetingPhrases: string[];
  completionPhrases: string[];
  errorPhrases: string[];
  workingDirectories: WorkingDirectories;
  listeningMode: "ptt" | "always-on";
  pttKeycode: number | null;
  approvalPhrases: string[];
  denyPhrases: string[];
  allowList: string[];
  disallowedTools: string[];
}

const DEFAULT_SETTINGS: AgentSettings = {
  wakeWords: [],
  shutdownWords: [],
  greetingPhrases: [],
  completionPhrases: [],
  errorPhrases: [],
  workingDirectories: { paths: [], default: 0 },
  listeningMode: "always-on",
  pttKeycode: null,
  approvalPhrases: [],
  denyPhrases: [],
  allowList: [],
  disallowedTools: [],
};

export function useAgentSettings() {
  const [loading, setLoading] = useState(true);
  const [settings, setSettings] = useState<AgentSettings>(DEFAULT_SETTINGS);
  const [saving, setSaving] = useState(false);
  const [apiKey, setApiKey] = useState("");
  const [savingApiKey, setSavingApiKey] = useState(false);
  const [characterId, setCharacterId] = useState<string | null>(null);

  useEffect(() => {
    const id = resolveCharacterId();
    setCharacterId(id);
    loadAllPreferences(id).then(({ settings: s, apiKey: k }) => {
      setSettings(s);
      setApiKey(k);
      setLoading(false);
    });
  }, []);

  const saveSettings = useCallback(async () => {
    if (saving || !characterId) return;
    setSaving(true);
    try {
      await preferences.save(`agent::${characterId}`, settings);
    } catch (err) {
      console.error("Failed to save agent settings:", err);
    } finally {
      setSaving(false);
    }
  }, [saving, characterId, settings]);

  const saveApiKey = useCallback(async () => {
    if (savingApiKey) return;
    setSavingApiKey(true);
    try {
      await preferences.save("agent::api-key", apiKey);
    } catch (err) {
      console.error("Failed to save API key:", err);
    } finally {
      setSavingApiKey(false);
    }
  }, [savingApiKey, apiKey]);

  const handleClose = useCallback(() => {
    audio.se.play("se:close");
    Webview.current()?.close();
  }, []);

  return {
    loading,
    settings,
    setSettings,
    saving,
    saveSettings,
    apiKey,
    setApiKey,
    savingApiKey,
    saveApiKey,
    handleClose,
  };
}

function resolveCharacterId(): string | null {
  return new URLSearchParams(location.search).get("linkedCharacter");
}

async function loadAllPreferences(
  characterId: string | null,
): Promise<{ settings: AgentSettings; apiKey: string }> {
  const [savedSettings, savedApiKey] = await Promise.all([
    characterId
      ? preferences.load<AgentSettings>(`agent::${characterId}`)
      : undefined,
    preferences.load<string>("agent::api-key"),
  ]);
  return {
    settings: savedSettings ?? DEFAULT_SETTINGS,
    apiKey: savedApiKey ?? "",
  };
}
