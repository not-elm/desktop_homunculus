import { useState, useEffect, useCallback, useRef, useMemo } from "react";
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

interface DetailViewProps {
  personaId: string;
  onDirtyChange: (dirty: boolean) => void;
  onSaved: () => void;
  onDelete: () => Promise<void>;
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

export default function DetailView({
  personaId,
  onDirtyChange,
  onSaved,
  onDelete,
}: DetailViewProps) {
  const [snapshot, setSnapshot] = useState<PersonaSnapshot | null>(null);
  const [formValues, setFormValues] = useState<PersonaFormValues | null>(null);
  const [vrmAssetId, setVrmAssetId] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [deleteOpen, setDeleteOpen] = useState(false);
  const initialValues = useRef<PersonaFormValues | null>(null);
  const initialVrm = useRef<string | null>(null);

  const persona = useMemo(() => new Persona(personaId), [personaId]);

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

  useEffect(() => {
    onDirtyChange(isDirty());
  }, [isDirty, onDirtyChange]);

  async function saveDraft(options?: { reload?: boolean }): Promise<boolean> {
    if (!formValues) return false;
    try {
      const vrmChanged = vrmAssetId !== initialVrm.current;
      await persona.patch({
        name: formValues.name,
        age: formValues.age ?? undefined,
        gender: formValues.gender,
        firstPersonPronoun: formValues.firstPersonPronoun || undefined,
        profile: formValues.profile,
        personality: formValues.personality || undefined,
        vrmAssetId: vrmChanged ? (vrmAssetId ?? undefined) : undefined,
      });

      if (vrmChanged && snapshot?.spawned) {
        if (vrmAssetId) {
          await persona.attachVrm(vrmAssetId);
        } else if (initialVrm.current) {
          await persona.detachVrm();
        }
      }

      if (options?.reload !== false) {
        await loadSnapshot();
      }
      return true;
    } catch (e) {
      console.error("Failed to save persona:", e);
      return false;
    }
  }

  async function handleSave() {
    if (saving) return;
    setSaving(true);
    try {
      await saveDraft();
      onSaved();
      setSaved(true);
      setTimeout(() => setSaved(false), 1500);
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
        const ok = await saveDraft({ reload: false });
        if (!ok) return;
        await persona.spawn();
      }
      await loadSnapshot();
      onSaved();
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
      onSaved();
    } catch (e) {
      console.error("Failed to toggle auto-spawn:", e);
    }
  }

  async function handleDelete() {
    try {
      await onDelete();
    } catch (e) {
      console.error("Failed to delete persona:", e);
    }
  }

  if (!snapshot || !formValues) {
    return (
      <div className="main-loading">
        <div className="main-loading-text">Loading...</div>
      </div>
    );
  }

  const autoSpawn = snapshot.metadata?.["auto-spawn"] === true;

  return (
    <div className="detail-view">
      <DetailHeader
        name={snapshot.name ?? ""}
        personaId={personaId}
        isSpawned={snapshot.spawned}
        onSpawnToggle={handleSpawnToggle}
        onSave={handleSave}
        saving={saving}
        saved={saved}
      />

      <div className="detail-body">
        <LeftColumn
          personaId={personaId}
          thumbnailUrl={persona.thumbnailUrl()}
          vrmAssetId={vrmAssetId}
          onVrmChange={setVrmAssetId}
          autoSpawn={autoSpawn}
          onAutoSpawnToggle={handleAutoSpawnToggle}
        />
        <RightColumn
          personaId={personaId}
          formValues={formValues}
          onFormChange={setFormValues}
        />
      </div>

      <DeleteSection onDelete={() => setDeleteOpen(true)} />

      <DeleteConfirmDialog
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
        onConfirm={handleDelete}
      />
    </div>
  );
}

function DetailHeader({
  name,
  personaId,
  isSpawned,
  onSpawnToggle,
  onSave,
  saving,
  saved,
}: {
  name: string;
  personaId: string;
  isSpawned: boolean;
  onSpawnToggle: () => void;
  onSave: () => void;
  saving: boolean;
  saved: boolean;
}) {
  return (
    <div className="detail-header">
      <div className="detail-header-left">
        <span className="detail-header-name">{name}</span>
        <span className="detail-header-id">{personaId}</span>
      </div>
      <div className="detail-header-actions">
        <button
          className={`detail-spawn-btn ${isSpawned ? "deactivate" : "activate"}`}
          onClick={onSpawnToggle}
          disabled={saving}
        >
          {isSpawned ? "Despawn" : "Spawn"}
        </button>
        <button
          className="management-btn management-btn--success"
          onClick={onSave}
          disabled={saving}
        >
          {saving ? "Saving..." : saved ? "Saved!" : "Save"}
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
  autoSpawn,
  onAutoSpawnToggle,
}: {
  personaId: string;
  thumbnailUrl: string;
  vrmAssetId: string | null;
  onVrmChange: (assetId: string | null) => void;
  autoSpawn: boolean;
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

      <VrmSelect personaId={personaId} value={vrmAssetId} onChange={onVrmChange} />

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

function DeleteSection({ onDelete }: { onDelete: () => void }) {
  return (
    <div className="delete-section">
      <button className="management-btn management-btn--danger" onClick={onDelete}>
        Delete Persona
      </button>
    </div>
  );
}

function DeleteConfirmDialog({
  open,
  onOpenChange,
  onConfirm,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>Delete Persona</DialogTitle>
          <DialogDescription>
            This action cannot be undone. Are you sure?
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <button
            className="management-btn management-btn--secondary"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </button>
          <button className="management-btn management-btn--danger" onClick={onConfirm}>
            Delete
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
