import { useCallback, useEffect, useRef, useState } from "react";
import { preferences, Webview } from "@hmcs/sdk";

export interface WorkspaceSelection {
  workspaceIndex: number;
  worktreeName: string | null;
}

export interface PttKey {
  code: string;
  modifiers: string[];
}

export interface AgentSettings {
  executor: "sdk" | "cli" | "codex";
  workspaces: { paths: string[]; selection: WorkspaceSelection };
  pttKey: PttKey | null;
  approvalPhrases: string[];
  denyPhrases: string[];
  allowList: string[];
  disallowedTools: string[];
  claudeModel: string;
}

const DEFAULT_SETTINGS: AgentSettings = {
  executor: "codex",
  workspaces: { paths: [], selection: { workspaceIndex: 0, worktreeName: null } },
  pttKey: null,
  approvalPhrases: [],
  denyPhrases: [],
  allowList: [],
  disallowedTools: [],
  claudeModel: "",
};

export function useAgentSettings() {
  const [loading, setLoading] = useState(true);
  const [settings, setSettings] = useState<AgentSettings>(DEFAULT_SETTINGS);
  const [saving, setSaving] = useState(false);
  const [apiKey, setApiKey] = useState("");
  const [savingApiKey, setSavingApiKey] = useState(false);
  const [characterId, setCharacterId] = useState<string | null>(null);
  const saveVersionRef = useRef(0);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const vrm = await Webview.current()?.linkedVrm();
      if (cancelled) return;
      const id = vrm ? await vrm.name() : null;
      if (cancelled) return;
      setCharacterId(id);
      const { settings: s, apiKey: k } = await loadAllPreferences(id);
      if (cancelled) return;
      setSettings(s);
      setApiKey(k);
      setLoading(false);
    })();
    return () => { cancelled = true; };
  }, []);

  const setAndSaveSettings = useCallback(async (newSettings: AgentSettings) => {
    setSettings(newSettings);
    if (!characterId) return;
    const version = ++saveVersionRef.current;
    try {
      await preferences.save(`agent::${characterId}`, newSettings);
    } catch (err) {
      if (version === saveVersionRef.current) {
        console.error("Failed to auto-save agent settings:", err);
      }
    }
  }, [characterId]);

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

  return {
    loading,
    settings,
    setSettings,
    setAndSaveSettings,
    saving,
    saveSettings,
    apiKey,
    setApiKey,
    savingApiKey,
    saveApiKey,
  };
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
    settings: savedSettings
      ? { ...DEFAULT_SETTINGS, ...savedSettings }
      : DEFAULT_SETTINGS,
    apiKey: savedApiKey ?? "",
  };
}
