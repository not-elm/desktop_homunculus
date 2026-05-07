import { Persona } from '@hmcs/sdk';
import {
  cn,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@hmcs/ui';
import { BehaviorSection } from '@persona/shared/components/BehaviorSection';
import { PersonaDetailBody } from '@persona/shared/components/PersonaDetailBody';
import { usePersonaDetail } from '@persona/shared/hooks/usePersonaDetail';
import { useThumbnailImport } from '@persona/shared/hooks/useThumbnailImport';
import { useMemo, useState } from 'react';
import {
  loadingTextClasses,
  managementBtnClasses,
  managementBtnDangerClasses,
  managementBtnSecondaryClasses,
  managementBtnSuccessClasses,
} from '../styles';

type Tab = 'persona' | 'appearance';

const TABS: { id: Tab; label: string }[] = [
  { id: 'persona', label: 'Persona' },
  { id: 'appearance', label: 'Appearance' },
];

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
    behaviorProcess,
    behaviorAnimations,
    setBehaviorProcess,
    setBehaviorAnimations,
  } = usePersonaDetail(personaId, callbacks);

  const persona = useMemo(() => new Persona(personaId), [personaId]);
  const { importThumbnail } = useThumbnailImport();
  const [tab, setTab] = useState<Tab>('persona');
  const [deleteOpen, setDeleteOpen] = useState(false);

  if (!snapshot || !formValues) {
    return (
      <div className="flex h-full min-h-[200px] items-center justify-center">
        <div className={loadingTextClasses}>Loading...</div>
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
    <div className="flex h-full flex-col">
      <DetailHeader
        name={snapshot.name ?? ''}
        personaId={personaId}
        isSpawned={snapshot.spawned}
        onSpawnToggle={toggleSpawn}
        onSave={save}
        saving={saving}
        saved={saved}
      />

      <div className="relative z-[7] flex flex-row gap-1 border-b border-[oklch(0.72_0.14_192/0.1)] pb-0">
        {TABS.map((t) => (
          <button
            type="button"
            key={t.id}
            className={cn(
              'cursor-pointer border-none border-b-2 border-transparent bg-transparent px-3 py-2 font-[inherit] text-xs uppercase tracking-[0.08em] text-[oklch(0.75_0.02_250/0.6)] transition-[color,border-color,text-shadow] duration-[var(--hud-duration-content)] ease-linear hover:text-[oklch(0.85_0.06_192/0.8)]',
              tab === t.id &&
                'border-b-[oklch(0.72_0.14_192/0.8)] text-[oklch(0.72_0.14_192)] [text-shadow:0_0_12px_oklch(0.72_0.14_192/0.3)]',
            )}
            onClick={() => setTab(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>

      <div className="flex-1 overflow-y-auto px-[18px] py-3">
        {tab === 'persona' && (
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
        )}
        {tab === 'appearance' && (
          <BehaviorSection
            process={behaviorProcess}
            animations={behaviorAnimations}
            onProcessChange={setBehaviorProcess}
            onAnimationsChange={setBehaviorAnimations}
          />
        )}
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

const spawnBtnBaseClasses =
  'cursor-pointer rounded-md border-none px-3.5 py-1.5 text-center font-[inherit] text-[10px] font-semibold uppercase tracking-[0.12em] transition-all duration-[250ms] ease-linear disabled:opacity-50 disabled:cursor-not-allowed';

const spawnBtnActivateClasses =
  'border border-[oklch(0.72_0.14_192/0.4)] bg-[oklch(0.72_0.14_192/0.15)] text-[oklch(0.72_0.14_192)] hover:bg-[oklch(0.72_0.14_192/0.25)] hover:[box-shadow:0_0_12px_oklch(0.72_0.14_192/0.15)]';

const spawnBtnDeactivateClasses =
  'border border-[oklch(0.62_0.2_25/0.25)] bg-[oklch(0.62_0.2_25/0.08)] text-[oklch(0.7_0.15_25)] hover:bg-[oklch(0.62_0.2_25/0.15)] hover:[box-shadow:0_0_12px_oklch(0.62_0.2_25/0.3)]';

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
    <div className="flex flex-shrink-0 items-center justify-between border-b border-[oklch(0.72_0.14_192/0.08)] px-[18px] py-3">
      <div className="flex items-baseline gap-2.5">
        <span className="text-[15px] font-bold tracking-[0.06em] text-[oklch(0.72_0.14_192/0.95)]">
          {name}
        </span>
        <span className="font-mono text-[9px] tracking-[0.5px] text-[oklch(0.72_0.14_192/0.25)]">
          {personaId}
        </span>
      </div>
      <div className="flex gap-1.5">
        <button
          type="button"
          className={cn(
            spawnBtnBaseClasses,
            isSpawned ? spawnBtnDeactivateClasses : spawnBtnActivateClasses,
          )}
          onClick={onSpawnToggle}
          disabled={saving}
        >
          {isSpawned ? 'Despawn' : 'Spawn'}
        </button>
        <button
          type="button"
          className={`${managementBtnClasses} ${managementBtnSuccessClasses}`}
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
    <div className="flex-shrink-0 border-t border-[oklch(0.62_0.2_25/0.08)] px-[18px] py-3">
      <button
        type="button"
        className={`${managementBtnClasses} ${managementBtnDangerClasses}`}
        onClick={onDelete}
      >
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
            type="button"
            className={`${managementBtnClasses} ${managementBtnSecondaryClasses}`}
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </button>
          <button
            type="button"
            className={`${managementBtnClasses} ${managementBtnDangerClasses}`}
            onClick={onConfirm}
          >
            Delete
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
