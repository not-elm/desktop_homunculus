import { preferences, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useState } from 'react';

// Re-define the types locally to avoid cross-entry-point import issues.
// These match the shapes in useAgentSettings.ts from the session HUD.

export interface WorkspaceSelection {
  workspaceIndex: number;
  worktreeName: string | null;
}

export interface PttKey {
  code: string;
  modifiers: string[];
}

export interface AgentSettings {
  runtime: 'sdk' | 'cli' | 'codex';
  workspaces: { paths: string[]; selection: WorkspaceSelection };
  pttKey: PttKey | null;
  approvalPhrases: string[];
  denyPhrases: string[];
  allowList: string[];
  disallowedTools: string[];
  claudeModel: string;
}

const DEFAULT_SETTINGS: AgentSettings = {
  runtime: 'codex',
  workspaces: { paths: [], selection: { workspaceIndex: 0, worktreeName: null } },
  pttKey: null,
  approvalPhrases: [],
  denyPhrases: [],
  allowList: [],
  disallowedTools: [],
  claudeModel: '',
};

export function useSettingsDraft() {
  const [loading, setLoading] = useState(true);
  const [settings, setSettings] = useState<AgentSettings>(DEFAULT_SETTINGS);
  const [savedSettings, setSavedSettings] = useState<AgentSettings>(DEFAULT_SETTINGS);
  const [saving, setSaving] = useState(false);
  const [personaId, setPersonaId] = useState<string | null>(null);
  const [apiKey, setApiKey] = useState('');
  const [savingApiKey, setSavingApiKey] = useState(false);

  const isDirty = JSON.stringify(settings) !== JSON.stringify(savedSettings);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const p = await Webview.current()?.linkedPersona();
      if (cancelled) return;
      const id = p ? p.id : null;
      if (cancelled) return;
      setPersonaId(id);
      const [loaded, loadedApiKey] = await Promise.all([
        id ? preferences.load<AgentSettings>(`agent::${id}`) : undefined,
        preferences.load<string>('agent::api-key'),
      ]);
      if (cancelled) return;
      const merged = loaded ? { ...DEFAULT_SETTINGS, ...loaded } : DEFAULT_SETTINGS;
      setSettings(merged);
      setSavedSettings(merged);
      setApiKey(loadedApiKey ?? '');
      setLoading(false);
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const saveSettings = useCallback(async () => {
    if (saving || !personaId) return;
    setSaving(true);
    try {
      await preferences.save(`agent::${personaId}`, settings);
      setSavedSettings(settings);
    } catch (err) {
      console.error('Failed to save settings:', err);
    } finally {
      setSaving(false);
    }
  }, [saving, personaId, settings]);

  const saveApiKey = useCallback(async () => {
    if (savingApiKey) return;
    setSavingApiKey(true);
    try {
      await preferences.save('agent::api-key', apiKey);
    } catch (err) {
      console.error('Failed to save API key:', err);
    } finally {
      setSavingApiKey(false);
    }
  }, [savingApiKey, apiKey]);

  const autoSave = useCallback(
    async (newSettings: AgentSettings) => {
      setSettings(newSettings);
      setSavedSettings(newSettings);
      if (!personaId) return;
      try {
        await preferences.save(`agent::${personaId}`, newSettings);
      } catch (err) {
        console.error('Failed to auto-save settings:', err);
      }
    },
    [personaId],
  );

  return {
    loading,
    settings,
    setSettings,
    isDirty,
    saving,
    saveSettings,
    savedSettings,
    apiKey,
    setApiKey,
    savingApiKey,
    saveApiKey,
    autoSave,
  };
}
