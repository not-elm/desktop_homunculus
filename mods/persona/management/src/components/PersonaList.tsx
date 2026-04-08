import { useState } from "react";
import type { PersonaSnapshot } from "@hmcs/sdk";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@hmcs/ui";
import PersonaCard from "./PersonaCard";

interface PersonaListProps {
  personas: PersonaSnapshot[];
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
  onSpawn: (id: string) => void;
  onDespawn: (id: string) => void;
  onAutoSpawnChange: (id: string, value: boolean) => void;
  onCreateClick: () => void;
}

export default function PersonaList({
  personas,
  onEdit,
  onDelete,
  onSpawn,
  onDespawn,
  onAutoSpawnChange,
  onCreateClick,
}: PersonaListProps) {
  const [deleteTarget, setDeleteTarget] = useState<PersonaSnapshot | null>(
    null,
  );

  function handleDeleteRequest(id: string) {
    const target = personas.find((p) => p.id === id) ?? null;
    setDeleteTarget(target);
  }

  async function handleDeleteConfirm() {
    if (deleteTarget) {
      await onDelete(deleteTarget.id);
      setDeleteTarget(null);
    }
  }

  return (
    <>
      <div className="management-header">
        <h1 className="management-title">Personas</h1>
        <button className="management-btn" onClick={onCreateClick}>
          + Create
        </button>
      </div>

      <div className="management-content">
        <div className="card-grid">
          {personas.map((persona) => (
            <PersonaCard
              key={persona.id}
              persona={persona}
              onEdit={onEdit}
              onDelete={handleDeleteRequest}
              onSpawn={onSpawn}
              onDespawn={onDespawn}
              onAutoSpawnChange={onAutoSpawnChange}
            />
          ))}
          <PlaceholderCard onClick={onCreateClick} />
        </div>
      </div>

      <DeleteConfirmDialog
        target={deleteTarget}
        onOpenChange={(open) => {
          if (!open) setDeleteTarget(null);
        }}
        onConfirm={handleDeleteConfirm}
      />
    </>
  );
}

function PlaceholderCard({ onClick }: { onClick: () => void }) {
  return (
    <div
      className="persona-card not-spawned"
      onClick={onClick}
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        minHeight: 180,
        border: "1px dashed oklch(0.72 0.14 192 / 0.2)",
        background: "transparent",
      }}
    >
      <span
        style={{
          fontSize: 14,
          letterSpacing: "0.08em",
          textTransform: "uppercase",
          color: "oklch(0.72 0.14 192 / 0.3)",
        }}
      >
        + New Persona
      </span>
    </div>
  );
}

function DeleteConfirmDialog({
  target,
  onOpenChange,
  onConfirm,
}: {
  target: PersonaSnapshot | null;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
}) {
  return (
    <Dialog open={target !== null} onOpenChange={onOpenChange}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>Delete {target?.name ?? target?.id}?</DialogTitle>
          <DialogDescription>
            This action cannot be undone.
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
            onClick={onConfirm}
          >
            Delete
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
