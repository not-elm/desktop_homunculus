import { useCallback, useEffect, useMemo, useState } from "react";
import { Persona, Webview, audio } from "@hmcs/sdk";
import { PersonaDetailBody } from "@persona/shared/components/PersonaDetailBody";
import { usePersonaDetail } from "@persona/shared/hooks/usePersonaDetail";
import { useThumbnailImport } from "@persona/shared/hooks/useThumbnailImport";
import { AppearanceTab } from "./components/AppearanceTab";
import { useScale } from "./hooks/useScale";

type Tab = "persona" | "appearance";

const NOOP = () => {};
const DETAIL_CALLBACKS = { onDirtyChange: NOOP, onSaved: NOOP };

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
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  return <SettingsContent personaId={personaId} />;
}

function SettingsContent({ personaId }: { personaId: string }) {
  const [tab, setTab] = useState<Tab>("persona");

  const detail = usePersonaDetail(personaId, DETAIL_CALLBACKS);
  const scaleState = useScale(personaId);
  const { importThumbnail } = useThumbnailImport();
  const persona = useMemo(() => new Persona(personaId), [personaId]);

  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  const handleClose = useCallback(() => {
    audio.se.play("se:close");
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
      console.error("Save failed:", err);
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
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  const autoSpawn = detail.snapshot.metadata?.["auto-spawn"] === true;
  const name = detail.snapshot.name ?? "";

  const tabs: { id: Tab; label: string }[] = [
    { id: "persona", label: "Persona" },
    { id: "appearance", label: "Appearance" },
  ];

  return (
    <div className="settings-panel holo-refract-border holo-noise">
      {/* Decorative layers */}
      <div className="settings-highlight" />
      <div className="settings-bottom-line" />
      <div className="settings-scanline" />
      <span className="settings-corner settings-corner--tl" />
      <span className="settings-corner settings-corner--tr" />
      <span className="settings-corner settings-corner--bl" />
      <span className="settings-corner settings-corner--br" />

      {/* Header */}
      <div className="settings-header">
        <h1 className="settings-title">Settings</h1>
        <span className="settings-entity-name">{name}</span>
      </div>

      {/* Tabs */}
      <div className="settings-tabs">
        {tabs.map((t) => (
          <button
            key={t.id}
            className={`settings-tab ${tab === t.id ? "settings-tab--active" : ""}`}
            onClick={() => setTab(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className={`settings-content${tab === "persona" ? " settings-content--visible" : ""}`}>
        {tab === "persona" && (
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
        {tab === "appearance" && (
          <AppearanceTab
            scale={scaleState.scale}
            onScaleChange={scaleState.setScale}
          />
        )}
      </div>

      {/* Footer */}
      <div className="settings-footer">
        <button className="settings-close" onClick={handleClose}>
          Close
        </button>
        <button
          className={`settings-save ${saved ? "settings-save--success" : ""}`}
          onClick={handleSave}
          disabled={saving}
        >
          {saving ? "Saving..." : saved ? "Saved!" : "Save"}
        </button>
      </div>
    </div>
  );
}
