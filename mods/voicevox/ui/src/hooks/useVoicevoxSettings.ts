import { audio, preferences, Webview } from '@hmcs/sdk';
import { rpc } from '@hmcs/sdk/rpc';
import { useCallback, useEffect, useState } from 'react';

const VOICEVOX_HOST = 'http://localhost:50021';

const DEFAULTS: VoicevoxSettings = {
  speakerId: 0,
  speedScale: 1.0,
  pitchScale: 0.0,
  intonationScale: 1.0,
  volumeScale: 1.0,
  pauseLength: 1.0,
  prePhonemeLength: 0.1,
  postPhonemeLength: 0.1,
};

export interface VoicevoxSettings {
  speakerId: number;
  speedScale: number;
  pitchScale: number;
  intonationScale: number;
  volumeScale: number;
  pauseLength: number;
  prePhonemeLength: number;
  postPhonemeLength: number;
}

export interface VoicevoxStyle {
  name: string;
  id: number;
}

export interface VoicevoxSpeaker {
  name: string;
  styles: VoicevoxStyle[];
}

export function useVoicevoxSettings() {
  const [loading, setLoading] = useState(true);
  const [connected, setConnected] = useState(false);
  const [speakers, setSpeakers] = useState<VoicevoxSpeaker[]>([]);
  const [settings, setSettings] = useState<VoicevoxSettings>(DEFAULTS);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [assetId, setAssetId] = useState<string | null>(null);
  const [characterName, setCharacterName] = useState('');
  const [invalidSpeaker, setInvalidSpeaker] = useState(false);
  const [personaId, setPersonaId] = useState<string | null>(null);
  const [speaking, setSpeaking] = useState(false);

  const prefsKey = assetId ? `voicevox::${assetId}` : null;

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const linked = await resolveLinkedPersona();
        if (cancelled) return;

        setPersonaId(linked?.personaId ?? null);
        const resolvedAssetId = linked?.assetId ?? null;
        setAssetId(resolvedAssetId);

        const [speakersResult, savedSettings] = await Promise.all([
          fetchSpeakers(),
          resolvedAssetId
            ? preferences.load<VoicevoxSettings>(`voicevox::${resolvedAssetId}`)
            : undefined,
        ]);
        if (cancelled) return;

        setCharacterName(linked?.name ?? '');

        if (speakersResult) {
          setConnected(true);
          setSpeakers(speakersResult);

          if (savedSettings) {
            const speakerExists = speakersResult.some((s) =>
              s.styles.some((st) => st.id === savedSettings.speakerId),
            );
            if (speakerExists) {
              setSettings(savedSettings);
            } else {
              setSettings({ ...savedSettings, speakerId: -1 });
              setInvalidSpeaker(true);
            }
          }
        }
      } catch (err) {
        console.error('Failed to initialize:', err);
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const handleSave = useCallback(async () => {
    if (saving || !prefsKey) return;
    setSaving(true);
    try {
      await preferences.save(prefsKey, settings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err) {
      console.error('Save failed:', err);
    } finally {
      setSaving(false);
    }
  }, [saving, prefsKey, settings]);

  const handleReset = useCallback(() => {
    setSettings(DEFAULTS);
    setInvalidSpeaker(false);
  }, []);

  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

  const handleRetry = useCallback(async () => {
    setLoading(true);
    setConnected(false);
    try {
      const result = await fetchSpeakers();
      if (result) {
        setConnected(true);
        setSpeakers(result);
      }
    } catch {
      // stay disconnected
    } finally {
      setLoading(false);
    }
  }, []);

  const handleSpeak = useCallback(
    async (text: string) => {
      if (speaking || !personaId || !prefsKey) return;
      setSpeaking(true);
      try {
        await preferences.save(prefsKey, settings);
        await rpc.call({
          modName: '@hmcs/voicevox',
          method: 'speak',
          body: { personaId, text },
        });
      } catch (err) {
        console.error('Speech test failed:', err);
      } finally {
        setSpeaking(false);
      }
    },
    [speaking, personaId, prefsKey, settings],
  );

  return {
    loading,
    connected,
    speakers,
    settings,
    setSettings,
    saving,
    saved,
    assetId,
    characterName,
    invalidSpeaker,
    personaId,
    speaking,
    handleSave,
    handleReset,
    handleClose,
    handleRetry,
    handleSpeak,
  };
}

/** Resolves the linked persona's ID, name, and VRM asset ID. */
async function resolveLinkedPersona(): Promise<{
  personaId: string;
  name: string;
  assetId: string | null;
} | null> {
  const webview = Webview.current();
  if (!webview) return null;
  const linked = await webview.linkedPersona();
  if (!linked) return null;
  const snapshot = await linked.snapshot();
  return {
    personaId: linked.id,
    name: snapshot.name ?? '',
    assetId: snapshot.vrmAssetId ?? null,
  };
}

async function fetchSpeakers(): Promise<VoicevoxSpeaker[] | null> {
  try {
    const cefFetch = (globalThis as unknown as { cef?: { fetch: typeof fetch } }).cef?.fetch;
    const fetchFn = cefFetch ?? fetch;
    const response = await fetchFn(`${VOICEVOX_HOST}/speakers`, {
      signal: AbortSignal.timeout(10000),
    });
    if (!response.ok) return null;
    return (await response.json()) as VoicevoxSpeaker[];
  } catch {
    return null;
  }
}
