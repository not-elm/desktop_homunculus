import { audio, Persona, Webview } from '@hmcs/sdk';
import { cn } from '@hmcs/ui';
import { PersonaDetailBody } from '@persona/shared/components/PersonaDetailBody';
import { usePersonaDetail } from '@persona/shared/hooks/usePersonaDetail';
import { useThumbnailImport } from '@persona/shared/hooks/useThumbnailImport';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { AppearanceTab } from './components/AppearanceTab';
import { Decorations } from './components/Decorations';
import { useScale } from './hooks/useScale';

type Tab = 'persona' | 'appearance';

const NOOP = () => {};
const DETAIL_CALLBACKS = { onDirtyChange: NOOP, onSaved: NOOP };

const panelClasses =
  'relative box-border flex h-screen max-h-screen max-w-[100vw] flex-col gap-4 overflow-hidden rounded-xl bg-[oklch(0.15_0.01_250/0.92)] p-5 animate-settings-in motion-reduce:animate-none motion-reduce:opacity-100';

const loadingTextClasses =
  'text-xs uppercase tracking-[0.12em] text-[oklch(0.72_0.14_192/0.5)] animate-holo-corner-pulse-fast motion-reduce:animate-none motion-reduce:opacity-50';

const tabClasses =
  'border-none border-b-2 border-transparent bg-transparent px-4 py-2 font-[inherit] text-xs uppercase tracking-[0.08em] text-[oklch(0.75_0.02_250/0.6)] cursor-pointer transition-[color,border-color,text-shadow] duration-[var(--hud-duration-content)] ease-linear hover:text-[oklch(0.85_0.06_192/0.8)]';

const tabActiveClasses =
  'text-[oklch(0.72_0.14_192)] border-b-[oklch(0.72_0.14_192/0.8)] [text-shadow:0_0_12px_oklch(0.72_0.14_192/0.3)]';

const closeButtonClasses =
  'rounded-md border border-[oklch(0.4_0.02_250/0.25)] bg-transparent px-6 py-2 font-[inherit] text-xs uppercase tracking-[0.08em] text-[oklch(0.75_0.04_250/0.75)] cursor-pointer transition-[color,border-color,text-shadow] duration-[var(--hud-duration-content)] ease-linear hover:text-[oklch(0.88_0.08_250/0.9)] hover:border-[oklch(0.55_0.04_250/0.35)] hover:[text-shadow:0_0_10px_oklch(0.72_0.14_192/0.2)] active:scale-[0.97]';

const saveButtonClasses =
  'rounded-md border border-[oklch(0.72_0.14_192/0.3)] bg-[oklch(0.72_0.14_192/0.15)] px-6 py-2 font-[inherit] text-xs uppercase tracking-[0.08em] text-[oklch(0.72_0.14_192)] cursor-pointer transition-[background,border-color,box-shadow,color] duration-[var(--hud-duration-content)] ease-linear hover:bg-[oklch(0.72_0.14_192/0.25)] hover:border-[oklch(0.72_0.14_192/0.5)] hover:[box-shadow:0_0_12px_oklch(0.72_0.14_192/0.2)] active:scale-[0.97] disabled:opacity-50 disabled:cursor-not-allowed';

const saveButtonSuccessClasses =
  'bg-[oklch(0.65_0.18_145/0.15)] border-[oklch(0.65_0.18_145/0.4)] text-[oklch(0.65_0.18_145)] [box-shadow:0_0_12px_oklch(0.65_0.18_145/0.15)] hover:bg-[oklch(0.65_0.18_145/0.2)] hover:border-[oklch(0.65_0.18_145/0.5)] hover:[box-shadow:0_0_16px_oklch(0.65_0.18_145/0.25)]';

export function App() {
  const [personaId, setPersonaId] = useState<string | null>(null);

  useEffect(() => {
    const webview = Webview.current();
    if (!webview) return;
    let cancelled = false;
    (async () => {
      const linked = await webview.linkedPersona();
      if (cancelled || !linked) return;
      setPersonaId(linked.id);
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  if (!personaId) {
    return <LoadingPanel />;
  }

  return <SettingsContent personaId={personaId} />;
}

function LoadingPanel() {
  return (
    <div className={cn(panelClasses, 'items-center justify-center min-h-[200px]')}>
      <div className={loadingTextClasses}>Loading...</div>
    </div>
  );
}

function SettingsContent({ personaId }: { personaId: string }) {
  const [tab, setTab] = useState<Tab>('persona');

  const detail = usePersonaDetail(personaId, DETAIL_CALLBACKS);
  const scaleState = useScale(personaId);
  const { importThumbnail } = useThumbnailImport();
  const persona = useMemo(() => new Persona(personaId), [personaId]);

  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

  const handleSave = useCallback(async () => {
    if (saving) return;
    setSaving(true);
    try {
      await detail.save();
      await scaleState.saveScale();
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err) {
      console.error('Save failed:', err);
    } finally {
      setSaving(false);
    }
  }, [saving, detail, scaleState]);

  const handleThumbnailChange = useCallback(async () => {
    const assetId = await importThumbnail(personaId);
    if (assetId) {
      detail.setThumbnail(assetId);
    }
  }, [personaId, importThumbnail, detail]);

  if (!detail.snapshot || !detail.formValues || scaleState.loading) {
    return <LoadingPanel />;
  }

  const autoSpawn = detail.snapshot.metadata?.['auto-spawn'] === true;
  const name = detail.snapshot.name ?? '';

  const tabs: { id: Tab; label: string }[] = [
    { id: 'persona', label: 'Persona' },
    { id: 'appearance', label: 'Appearance' },
  ];

  return (
    <div className={cn(panelClasses, 'holo-refract-border holo-noise')}>
      <Decorations />

      <div className="relative z-[7] flex flex-row items-center justify-between">
        <h1 className="m-0 text-sm font-semibold uppercase tracking-[0.15em] text-[oklch(0.72_0.14_192/0.9)]">
          Settings
        </h1>
        <span className="font-mono text-xs text-[oklch(0.72_0.14_192/0.5)]">{name}</span>
      </div>

      <div className="relative z-[7] flex flex-row gap-1 border-b border-[oklch(0.72_0.14_192/0.1)] pb-0">
        {tabs.map((t) => (
          <button
            type="button"
            key={t.id}
            className={cn(tabClasses, tab === t.id && tabActiveClasses)}
            onClick={() => setTab(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>

      <div className="no-scrollbar relative z-[7] min-h-0 flex-1 overflow-y-auto">
        {tab === 'persona' && (
          <PersonaDetailBody
            personaId={personaId}
            thumbnailUrl={persona.thumbnailUrl(detail.thumbnail)}
            onThumbnailChange={handleThumbnailChange}
            vrmAssetId={detail.vrmAssetId}
            onVrmChange={detail.setVrmAssetId}
            autoSpawn={autoSpawn}
            onAutoSpawnToggle={detail.toggleAutoSpawn}
            formValues={detail.formValues}
            onFormChange={detail.setFormValues}
          />
        )}
        {tab === 'appearance' && (
          <AppearanceTab
            scale={scaleState.scale}
            onScaleChange={scaleState.setScale}
            behaviorProcess={detail.behaviorProcess}
            behaviorAnimations={detail.behaviorAnimations}
            onBehaviorProcessChange={detail.setBehaviorProcess}
            onBehaviorAnimationsChange={detail.setBehaviorAnimations}
          />
        )}
      </div>

      <div className="relative z-[7] flex justify-end gap-2 pt-2">
        <button type="button" className={closeButtonClasses} onClick={handleClose}>
          Close
        </button>
        <button
          type="button"
          className={cn(saveButtonClasses, saved && saveButtonSuccessClasses)}
          onClick={handleSave}
          disabled={saving}
        >
          {saving ? 'Saving...' : saved ? 'Saved!' : 'Save'}
        </button>
      </div>
    </div>
  );
}
