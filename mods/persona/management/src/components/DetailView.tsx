import { useState, useMemo } from "react";
import { Persona } from "@hmcs/sdk";
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
import { usePersonaDetail } from "../hooks/usePersonaDetail";

interface DetailViewProps {
  personaId: string;
  onDirtyChange: (dirty: boolean) => void;
  onSaved: () => void;
  onDelete: () => Promise<void>;
}

export default function DetailView({
  personaId,
  onDirtyChange,
  onSaved,
  onDelete,
}: DetailViewProps) {
  const callbacks = useMemo(() => ({ onDirtyChange, onSaved }), [onDirtyChange, onSaved]);
  const {
    snapshot,
    formValues,
    vrmAssetId,
    saving,
    saved,
    setFormValues,
    setVrmAssetId,
    save,
    toggleSpawn,
    toggleAutoSpawn,
  } = usePersonaDetail(personaId, callbacks);

  const persona = useMemo(() => new Persona(personaId), [personaId]);
  const [deleteOpen, setDeleteOpen] = useState(false);

  if (!snapshot || !formValues) {
    return (
      <div className="main-loading">
        <div className="main-loading-text">Loading...</div>
      </div>
    );
  }

  const autoSpawn = snapshot.metadata?.["auto-spawn"] === true;

  async function handleDelete() {
    try {
      await onDelete();
    } catch (e) {
      console.error("Failed to delete persona:", e);
    }
  }

  return (
    <div className="detail-view">
      <DetailHeader
        name={snapshot.name ?? ""}
        personaId={personaId}
        isSpawned={snapshot.spawned}
        onSpawnToggle={toggleSpawn}
        onSave={save}
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
          onAutoSpawnToggle={toggleAutoSpawn}
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
