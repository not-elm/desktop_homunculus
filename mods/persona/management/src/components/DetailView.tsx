import { Persona } from '@hmcs/sdk';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@hmcs/ui';
import { PersonaDetailBody } from '@persona/shared/components/PersonaDetailBody';
import { usePersonaDetail } from '@persona/shared/hooks/usePersonaDetail';
import { useThumbnailImport } from '@persona/shared/hooks/useThumbnailImport';
import { useMemo, useState } from 'react';

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
    thumbnail,
    setThumbnail,
    saving,
    saved,
    setFormValues,
    setVrmAssetId,
    save,
    toggleSpawn,
    toggleAutoSpawn,
  } = usePersonaDetail(personaId, callbacks);

  const persona = useMemo(() => new Persona(personaId), [personaId]);
  const { importThumbnail } = useThumbnailImport();
  const [deleteOpen, setDeleteOpen] = useState(false);

  if (!snapshot || !formValues) {
    return (
      <div className="main-loading">
        <div className="main-loading-text">Loading...</div>
      </div>
    );
  }

  const autoSpawn = snapshot.metadata?.['auto-spawn'] === true;

  async function handleThumbnailChange() {
    const assetId = await importThumbnail(personaId);
    if (assetId) {
      setThumbnail(assetId);
    }
  }

  async function handleDelete() {
    try {
      await onDelete();
    } catch (e) {
      console.error('Failed to delete persona:', e);
    }
  }

  return (
    <div className="detail-view">
      <DetailHeader
        name={snapshot.name ?? ''}
        personaId={personaId}
        isSpawned={snapshot.spawned}
        onSpawnToggle={toggleSpawn}
        onSave={save}
        saving={saving}
        saved={saved}
      />

      <PersonaDetailBody
        personaId={personaId}
        thumbnailUrl={persona.thumbnailUrl(thumbnail)}
        onThumbnailChange={handleThumbnailChange}
        vrmAssetId={vrmAssetId}
        onVrmChange={setVrmAssetId}
        autoSpawn={autoSpawn}
        onAutoSpawnToggle={toggleAutoSpawn}
        formValues={formValues}
        onFormChange={setFormValues}
      />

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
          className={`detail-spawn-btn ${isSpawned ? 'deactivate' : 'activate'}`}
          onClick={onSpawnToggle}
          disabled={saving}
        >
          {isSpawned ? 'Despawn' : 'Spawn'}
        </button>
        <button
          className="management-btn management-btn--success"
          onClick={onSave}
          disabled={saving}
        >
          {saving ? 'Saving...' : saved ? 'Saved!' : 'Save'}
        </button>
      </div>
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
          <DialogDescription>This action cannot be undone. Are you sure?</DialogDescription>
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
