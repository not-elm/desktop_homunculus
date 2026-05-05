import { audio, type Persona, type PersonaSnapshot, Webview } from '@hmcs/sdk';
import {
  Button,
  cn,
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
      <Shell onClose={handleClose}>
        <div className="flex min-h-[200px] items-center justify-center">
          <div className="animate-[holo-corner-pulse_2s_ease-in-out_infinite] text-[var(--hud-font-size-sm)] uppercase tracking-[0.12em] text-[oklch(0.72_0.14_192/0.5)] motion-reduce:animate-none motion-reduce:opacity-50">
            Loading…
          </div>
        </div>
      </Shell>
    );
  }

  if (error || !linked.persona || !linked.snapshot) {
    return (
      <Shell onClose={handleClose}>
        <div className="flex flex-col gap-[var(--hud-space-md)] p-[var(--hud-space-lg)] text-left">
          <span className="text-[var(--hud-font-size-sm)] text-holo-rose/85">
            {error ?? 'No linked persona'}
          </span>
          <Button
            variant="hud"
            size="hud"
            className="relative z-[7] self-start"
            onClick={() => {
              linked.refetch();
              ttsEngines.refetch();
            }}
          >
            Retry
          </Button>
        </div>
      </Shell>
    );
  }

  return (
    <Shell onClose={handleClose}>
      <section className="flex flex-col gap-[var(--hud-space-xl)]">
        <PersonaPanel
          persona={linked.persona}
          snapshot={linked.snapshot}
          engines={ttsEngines.data}
        />
      </section>
    </Shell>
  );
}

function Shell({ children, onClose }: { children: React.ReactNode; onClose: () => void }) {
  return (
    <div
      className={cn(
        'holo-noise relative box-border flex h-screen max-h-screen max-w-screen flex-col gap-0 overflow-hidden rounded-xl bg-[oklch(0.15_0.01_250/0.92)] p-0',
        'animate-settings-in motion-reduce:animate-none motion-reduce:opacity-100',
      )}
    >
      <Decorations />
      <Toolbar title="Openclaw" onClose={onClose} />
      <div className="relative z-[7] flex min-h-0 flex-1 flex-col gap-[var(--hud-space-xl)] overflow-y-auto px-5 py-[var(--hud-space-xl)] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
        {children}
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
    <label
      className="flex flex-col gap-[var(--hud-space-sm)] text-[var(--hud-font-size-sm)] uppercase tracking-[0.1em] text-[oklch(0.72_0.14_192/0.7)]"
      htmlFor="openclaw-tts-engine-select"
    >
      TTS Engine
      <Select value={value} onValueChange={onChange} disabled={saving}>
        <SelectTrigger id="openclaw-tts-engine-select" className="w-full">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value={NONE_VALUE}>None (no TTS)</SelectItem>
            {engines.map((engine) => (
              <SelectItem key={engine.modName} value={engine.modName}>
                {engine.modName}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
      {err && (
        <span className="text-[var(--hud-font-size-sm)] text-[oklch(0.7_0.16_350/0.85)]">
          {err}
        </span>
      )}
    </label>
  );
}

function readTtsModName(snapshot: PersonaSnapshot): string {
  const metadata = (snapshot.metadata ?? {}) as { ttsModName?: unknown };
  const value = metadata.ttsModName;
  if (typeof value === 'string' && value.length > 0) return value;
  return NONE_VALUE;
}
