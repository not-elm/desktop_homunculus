import { Persona, type PersonaSnapshot } from '@hmcs/sdk';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@hmcs/ui';
import { useMemo, useState } from 'react';
import { usePersonas } from './hooks/usePersonas';
import { type TtsEngine, useTtsEngines } from './hooks/useTtsEngines';
import { setTtsModName } from './lib/metadata';

const NONE_VALUE = '__none__';

export function App() {
  const personas = usePersonas();
  const ttsEngines = useTtsEngines();
  const loading = personas.loading || ttsEngines.loading;
  const error = personas.error ?? ttsEngines.error;

  if (loading) {
    return <div className="p-6 text-white/80 text-sm">Loading…</div>;
  }

  if (error) {
    return (
      <div className="p-6 text-white/90 flex flex-col gap-3">
        <div className="text-sm text-red-300">{error}</div>
        <button
          type="button"
          className="self-start px-3 py-1.5 text-sm bg-white/10 border border-white/20 rounded hover:bg-white/15"
          onClick={() => {
            personas.refetch();
            ttsEngines.refetch();
          }}
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="p-6 flex flex-col gap-4 text-white/90">
      <header>
        <h1 className="text-lg font-semibold tracking-wide">OpenClaw Settings</h1>
        <p className="text-xs text-white/60">
          Pick a TTS engine per persona. "None" disables speech output — text replies still flow.
        </p>
      </header>

      {ttsEngines.data.length === 0 && (
        <div className="text-xs text-amber-200/90 bg-amber-400/10 border border-amber-300/30 rounded p-2">
          No TTS engine installed. Install a MOD whose RPC method declares <code>meta.category = "tts"</code>.
        </div>
      )}

      <section className="flex flex-col gap-3">
        {personas.data.length === 0 ? (
          <div className="text-sm text-white/70">No personas available.</div>
        ) : (
          personas.data.map((p) => (
            <PersonaRow key={p.id} persona={p} engines={ttsEngines.data} />
          ))
        )}
      </section>
    </div>
  );
}

function PersonaRow({
  persona,
  engines,
}: {
  persona: PersonaSnapshot;
  engines: TtsEngine[];
}) {
  const initial = useMemo(() => readTtsModName(persona), [persona]);
  const [value, setValue] = useState<string>(initial);
  const [saving, setSaving] = useState(false);
  const [err, setErr] = useState<string | null>(null);

  async function onChange(next: string) {
    setValue(next);
    setSaving(true);
    setErr(null);
    try {
      const p = await Persona.load(persona.id);
      const existing = await p.metadata();
      const merged = setTtsModName(existing, next === NONE_VALUE ? null : next);
      await p.setMetadata(merged);
    } catch (e) {
      setErr((e as Error).message ?? 'Save failed');
      setValue(initial);
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="flex items-center justify-between gap-4 bg-white/5 border border-white/15 rounded p-3">
      <div className="flex flex-col min-w-0">
        <span className="text-sm font-medium truncate">{persona.name ?? persona.id}</span>
        <span className="text-xs text-white/50 truncate">{persona.id}</span>
        {err && <span className="text-xs text-red-300 mt-1">{err}</span>}
      </div>
      <Select value={value} onValueChange={onChange} disabled={saving}>
        <SelectTrigger className="w-56">
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

function readTtsModName(persona: PersonaSnapshot): string {
  const metadata = (persona.metadata ?? {}) as { ttsModName?: unknown };
  const value = metadata.ttsModName;
  if (typeof value === 'string' && value.length > 0) return value;
  return NONE_VALUE;
}
