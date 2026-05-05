import { audio, Persona, Webview } from '@hmcs/sdk';
import { Button, cn, Tabs, TabsContent, TabsList, TabsTrigger, Toolbar } from '@hmcs/ui';
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
  'holo-noise relative box-border flex h-screen max-h-screen max-w-screen flex-col overflow-hidden rounded-xl bg-panel/92 animate-settings-in motion-reduce:animate-none motion-reduce:opacity-100';

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
    return (
      <div className={cn(panelClasses, 'items-center justify-center')}>
        <div className="text-xs uppercase tracking-[0.12em] text-primary/50">Loading...</div>
      </div>
    );
  }

  return <SettingsContent personaId={personaId} />;
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
    return (
      <div className={cn(panelClasses, 'items-center justify-center')}>
        <div className="text-xs uppercase tracking-[0.12em] text-primary/50">Loading...</div>
      </div>
    );
  }

  const autoSpawn = detail.snapshot.metadata?.['auto-spawn'] === true;
  const name = detail.snapshot.name ?? '';

  return (
    <div className={panelClasses}>
      <Decorations />

      <Toolbar
        title="Settings"
        onClose={handleClose}
        className="border-b-0 bg-transparent px-5 pt-4 pb-0"
      >
        {name && (
          <span className="font-mono text-[10px] tracking-[0.04em] text-primary/50">{name}</span>
        )}
      </Toolbar>

      <Tabs
        value={tab}
        onValueChange={(v) => setTab(v as Tab)}
        className="relative z-[7] flex min-h-0 flex-1 flex-col gap-0"
      >
        <TabsList className="mx-5 mt-2 h-auto justify-start gap-1 rounded-none border-0 border-b border-primary/12 bg-transparent p-0">
          <TabsTrigger
            value="persona"
            className="rounded-none border-0 px-4 py-2 text-xs font-medium uppercase tracking-[0.08em] data-[state=active]:bg-transparent data-[state=active]:text-primary data-[state=active]:shadow-none data-[state=active]:[box-shadow:inset_0_-2px_0_var(--primary)]"
          >
            Persona
          </TabsTrigger>
          <TabsTrigger
            value="appearance"
            className="rounded-none border-0 px-4 py-2 text-xs font-medium uppercase tracking-[0.08em] data-[state=active]:bg-transparent data-[state=active]:text-primary data-[state=active]:shadow-none data-[state=active]:[box-shadow:inset_0_-2px_0_var(--primary)]"
          >
            Appearance
          </TabsTrigger>
        </TabsList>

        <TabsContent
          value="persona"
          className="no-scrollbar flex-1 overflow-y-auto px-[18px] py-[14px]"
        >
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
        </TabsContent>

        <TabsContent
          value="appearance"
          className="no-scrollbar flex-1 overflow-y-auto px-[18px] py-[14px]"
        >
          <AppearanceTab
            scale={scaleState.scale}
            onScaleChange={scaleState.setScale}
            behaviorProcess={detail.behaviorProcess}
            behaviorAnimations={detail.behaviorAnimations}
            onBehaviorProcessChange={detail.setBehaviorProcess}
            onBehaviorAnimationsChange={detail.setBehaviorAnimations}
          />
        </TabsContent>
      </Tabs>

      <Footer onSave={handleSave} saving={saving} saved={saved} />
    </div>
  );
}

function Footer({
  onSave,
  saving,
  saved,
}: {
  onSave: () => void;
  saving: boolean;
  saved: boolean;
}) {
  return (
    <div className="relative z-[7] flex shrink-0 justify-end gap-2 border-t border-primary/12 bg-primary/4 px-3.5 py-2">
      <Button variant={saved ? 'hud-success' : 'hud'} size="hud" onClick={onSave} disabled={saving}>
        {saving ? 'Saving...' : saved ? 'Saved!' : 'Save'}
      </Button>
    </div>
  );
}
