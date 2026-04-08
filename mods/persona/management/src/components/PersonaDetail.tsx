import { useState, useEffect, useCallback, useRef } from "react";
import { Persona, type PersonaSnapshot } from "@hmcs/sdk";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@hmcs/ui";
import {
  PersonaFields,
  type PersonaFormValues,
} from "@persona/shared/components/PersonaFields";
import VrmSelect from "./VrmSelect";

interface PersonaDetailProps {
  personaId: string;
  onBack: () => void;
}

function snapshotToFormValues(snapshot: PersonaSnapshot): PersonaFormValues {
  return {
    name: snapshot.name ?? "",
    age: snapshot.age ?? null,
    gender: snapshot.gender,
    firstPersonPronoun: snapshot.firstPersonPronoun ?? "",
    profile: snapshot.profile,
    personality: snapshot.personality ?? "",
  };
}

export default function PersonaDetail({
  personaId,
  onBack,
}: PersonaDetailProps) {
  const [snapshot, setSnapshot] = useState<PersonaSnapshot | null>(null);
  const [formValues, setFormValues] = useState<PersonaFormValues | null>(null);
  const [vrmAssetId, setVrmAssetId] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [discardOpen, setDiscardOpen] = useState(false);
  const initialValues = useRef<PersonaFormValues | null>(null);
  const initialVrm = useRef<string | null>(null);

  const persona = new Persona(personaId);

  const loadSnapshot = useCallback(async () => {
    try {
      const snap = await new Persona(personaId).snapshot();
      setSnapshot(snap);
      const values = snapshotToFormValues(snap);
      setFormValues(values);
      setVrmAssetId(snap.vrmAssetId ?? null);
      initialValues.current = values;
      initialVrm.current = snap.vrmAssetId ?? null;
    } catch (e) {
      console.error("Failed to load persona:", e);
    }
  }, [personaId]);

  useEffect(() => {
    loadSnapshot();
  }, [loadSnapshot]);

  const isDirty = useCallback(() => {
    if (!formValues || !initialValues.current) return false;
    const iv = initialValues.current;
    return (
      formValues.name !== iv.name ||
      formValues.age !== iv.age ||
      formValues.gender !== iv.gender ||
      formValues.firstPersonPronoun !== iv.firstPersonPronoun ||
      formValues.profile !== iv.profile ||
      formValues.personality !== iv.personality ||
      vrmAssetId !== initialVrm.current
    );
  }, [formValues, vrmAssetId]);

  function handleBackClick() {
    if (isDirty()) {
      setDiscardOpen(true);
    } else {
      onBack();
    }
  }

  async function handleSave() {
    if (!formValues || saving) return;
    setSaving(true);
    try {
      await persona.patch({
        name: formValues.name,
        age: formValues.age ?? undefined,
        gender: formValues.gender,
        firstPersonPronoun: formValues.firstPersonPronoun || undefined,
        profile: formValues.profile,
        personality: formValues.personality || undefined,
      });

      if (vrmAssetId !== initialVrm.current) {
        if (vrmAssetId) {
          await persona.attachVrm(vrmAssetId);
        } else if (initialVrm.current) {
          await persona.detachVrm();
        }
      }

      await loadSnapshot();
    } catch (e) {
      console.error("Failed to save persona:", e);
    } finally {
      setSaving(false);
    }
  }

  async function handleSpawnToggle() {
    if (!snapshot) return;
    try {
      if (snapshot.spawned) {
        await persona.despawn();
      } else {
        await persona.spawn();
      }
      await loadSnapshot();
    } catch (e) {
      console.error("Failed to toggle spawn:", e);
    }
  }

  async function handleAutoSpawnToggle() {
    if (!snapshot) return;
    const current = snapshot.metadata?.["auto-spawn"] === true;
    try {
      await persona.patch({
        metadata: { ...(snapshot.metadata ?? {}), "auto-spawn": !current },
      });
      await loadSnapshot();
    } catch (e) {
      console.error("Failed to toggle auto-spawn:", e);
    }
  }

  if (!snapshot || !formValues) {
    return (
      <div className="management-loading">
        <div className="management-loading-text">Loading...</div>
      </div>
    );
  }

  const autoSpawn = snapshot.metadata?.["auto-spawn"] === true;

  return (
    <>
      <DetailHeader
        onBack={handleBackClick}
        onCancel={handleBackClick}
        onSave={handleSave}
        saving={saving}
      />

      <div className="management-content">
        <div className="detail-layout">
          <LeftColumn
            personaId={personaId}
            thumbnailUrl={persona.thumbnailUrl()}
            vrmAssetId={vrmAssetId}
            onVrmChange={setVrmAssetId}
            isSpawned={snapshot.spawned}
            autoSpawn={autoSpawn}
            onSpawnToggle={handleSpawnToggle}
            onAutoSpawnToggle={handleAutoSpawnToggle}
          />
          <RightColumn
            personaId={personaId}
            formValues={formValues}
            onFormChange={setFormValues}
          />
        </div>
      </div>

      <DiscardDialog
        open={discardOpen}
        onOpenChange={setDiscardOpen}
        onDiscard={onBack}
      />
    </>
  );
}

function DetailHeader({
  onBack,
  onCancel,
  onSave,
  saving,
}: {
  onBack: () => void;
  onCancel: () => void;
  onSave: () => void;
  saving: boolean;
}) {
  return (
    <div className="management-header">
      <div style={{ display: "flex", alignItems: "center", gap: 16 }}>
        <button className="management-back" onClick={onBack}>
          &#8592; Back
        </button>
        <h1 className="management-title">Edit Persona</h1>
      </div>
      <div style={{ display: "flex", gap: 8 }}>
        <button
          className="management-btn management-btn--secondary"
          onClick={onCancel}
        >
          Cancel
        </button>
        <button
          className="management-btn management-btn--success"
          onClick={onSave}
          disabled={saving}
        >
          {saving ? "Saving..." : "Save"}
        </button>
      </div>
    </div>
  );
}

function LeftColumn({
  personaId,
  thumbnailUrl,
  vrmAssetId,
  onVrmChange,
  isSpawned,
  autoSpawn,
  onSpawnToggle,
  onAutoSpawnToggle,
}: {
  personaId: string;
  thumbnailUrl: string;
  vrmAssetId: string | null;
  onVrmChange: (assetId: string | null) => void;
  isSpawned: boolean;
  autoSpawn: boolean;
  onSpawnToggle: () => void;
  onAutoSpawnToggle: () => void;
}) {
  return (
    <div className="detail-left">
      <div className="detail-thumb">
        <img src={thumbnailUrl} alt="Thumbnail" />
        <div className="change-overlay">
          <span>Change Image...</span>
        </div>
      </div>

      <VrmSelect
        personaId={personaId}
        value={vrmAssetId}
        onChange={onVrmChange}
      />

      <div className="detail-spawn-section">
        <div className="detail-spawn-status">
          <div
            className={`status-dot ${isSpawned ? "active" : "inactive"}`}
          />
          <span
            className={`status-label ${isSpawned ? "active" : "inactive"}`}
          >
            {isSpawned ? "Online" : "Offline"}
          </span>
        </div>

        <button
          className={`detail-spawn-btn ${isSpawned ? "deactivate" : "activate"}`}
          onClick={onSpawnToggle}
        >
          {isSpawned ? "Despawn" : "Spawn"}
        </button>

        <div className="detail-separator" />

        <div className="detail-auto-row">
          <div>
            <div className="detail-auto-label">Auto Spawn</div>
            <div className="detail-auto-sublabel">Launch at startup</div>
          </div>
          <button
            className={`toggle-mini ${autoSpawn ? "on" : "off"}`}
            onClick={onAutoSpawnToggle}
            aria-label="Toggle auto spawn"
            role="switch"
            aria-checked={autoSpawn}
          >
            <span className="knob" />
          </button>
        </div>
      </div>
    </div>
  );
}

function RightColumn({
  personaId,
  formValues,
  onFormChange,
}: {
  personaId: string;
  formValues: PersonaFormValues;
  onFormChange: (values: PersonaFormValues) => void;
}) {
  return (
    <div className="detail-right">
      <div className="detail-field">
        <div className="detail-field-label">ID</div>
        <input
          type="text"
          className="settings-input"
          value={personaId}
          readOnly
          style={{ opacity: 0.5, cursor: "not-allowed", width: "100%" }}
        />
      </div>
      <PersonaFields values={formValues} onChange={onFormChange} />
    </div>
  );
}

function DiscardDialog({
  open,
  onOpenChange,
  onDiscard,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onDiscard: () => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>Unsaved Changes</DialogTitle>
          <DialogDescription>
            You have unsaved changes. Discard?
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <button
            className="management-btn management-btn--secondary"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </button>
          <button
            className="management-btn management-btn--danger"
            onClick={onDiscard}
          >
            Discard
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
