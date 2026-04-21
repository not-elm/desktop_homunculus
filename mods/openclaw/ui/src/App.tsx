import { audio, type Persona, type PersonaSnapshot, Webview } from '@hmcs/sdk';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Toolbar,
} from '@hmcs/ui';
import { useCallback, useMemo, useState } from 'react';
import { Decorations } from './components/Decorations';
import { useLinkedPersona } from './hooks/useLinkedPersona';
import { type TtsEngine, useTtsEngines } from './hooks/useTtsEngines';
import { setTtsModName } from './lib/metadata';

const NONE_VALUE = '__none__';

export function App() {
  const linked = useLinkedPersona();
  const ttsEngines = useTtsEngines();
  const loading = linked.loading || ttsEngines.loading;
  const error = linked.error ?? ttsEngines.error;

  const handleClose = useCallback(() => {
    audio.se.play('se:close').catch(() => {
      // best-effort; ignore if SE service is unavailable
    });
    Webview.current()?.close();
  }, []);

  if (loading) {
    return (
      <div className="settings-panel holo-noise">
        <Decorations />
        <Toolbar title="Openclaw" onClose={handleClose} />
        <div className="settings-content">
          <div className="settings-loading">
            <div className="settings-loading-text">Loading…</div>
          </div>
        </div>
      </div>
    );
  }

  if (error || !linked.persona || !linked.snapshot) {
    return (
      <div className="settings-panel holo-noise">
        <Decorations />
        <Toolbar title="Openclaw" onClose={handleClose} />
        <div className="settings-content">
          <div className="openclaw-error-block">
            <span className="openclaw-error-text">{error ?? 'No linked persona'}</span>
            <button
              type="button"
              className="openclaw-retry"
              onClick={() => {
                linked.refetch();
                ttsEngines.refetch();
              }}
            >
              Retry
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="settings-panel holo-noise">
      <Decorations />
      <Toolbar title="Openclaw" onClose={handleClose} />
      <div className="settings-content">
        <section className="settings-section">
          <PersonaPanel
            persona={linked.persona}
            snapshot={linked.snapshot}
            engines={ttsEngines.data}
          />
        </section>
      </div>
    </div>
  );
}

function PersonaPanel({
  persona,
  snapshot,
  engines,
}: {
  persona: Persona;
  snapshot: PersonaSnapshot;
  engines: TtsEngine[];
}) {
  const initial = useMemo(() => readTtsModName(snapshot), [snapshot]);
  const [value, setValue] = useState<string>(initial);
  const [saving, setSaving] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  async function onChange(next: string) {
    setValue(next);
    setSaving(true);
    setErr(null);
    try {
      const existing = await persona.metadata();
      const merged = setTtsModName(existing, next === NONE_VALUE ? null : next);
      await persona.setMetadata(merged);
    } catch (e) {
      setErr((e as Error).message ?? 'Save failed');
      setValue(initial);
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="openclaw-persona-row">
      <div className="openclaw-persona-meta">
        <span className="openclaw-persona-name">{snapshot.name ?? snapshot.id}</span>
        <span className="openclaw-persona-id">{snapshot.id}</span>
        {err && <span className="openclaw-error-text">{err}</span>}
      </div>
      <Select value={value} onValueChange={onChange} disabled={saving}>
        <SelectTrigger className="openclaw-select">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value={NONE_VALUE}>None (no TTS)</SelectItem>
            {engines.map((engine) => (
              <SelectItem key={engine.modName} value={engine.modName}>
                {engine.description}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  );
}

function readTtsModName(snapshot: PersonaSnapshot): string {
  const metadata = (snapshot.metadata ?? {}) as { ttsModName?: unknown };
  const value = metadata.ttsModName;
  if (typeof value === 'string' && value.length > 0) return value;
  return NONE_VALUE;
}
