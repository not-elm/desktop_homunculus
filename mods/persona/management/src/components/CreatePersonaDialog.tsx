import { useState, useMemo } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@hmcs/ui";

interface CreatePersonaDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreate: (id: string, name: string) => Promise<void>;
}

const ID_PATTERN = /^[a-zA-Z0-9_-]{1,64}$/;

export default function CreatePersonaDialog({
  open,
  onOpenChange,
  onCreate,
}: CreatePersonaDialogProps) {
  const [id, setId] = useState("");
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const idValid = useMemo(() => ID_PATTERN.test(id), [id]);
  const idError = id.length > 0 && !idValid;
  const formValid = idValid && name.trim().length > 0;

  function resetForm() {
    setId("");
    setName("");
    setError(null);
    setSubmitting(false);
  }

  function handleOpenChange(nextOpen: boolean) {
    if (!nextOpen) resetForm();
    onOpenChange(nextOpen);
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!formValid || submitting) return;

    setSubmitting(true);
    setError(null);
    try {
      await onCreate(id, name.trim());
      resetForm();
    } catch (err) {
      const message = (err as Error).message ?? "Failed to create persona";
      setError(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>Create Persona</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit}>
          <div style={{ display: "flex", flexDirection: "column", gap: 14 }}>
            <label className="settings-label">
              ID
              <input
                type="text"
                className="settings-input"
                value={id}
                onChange={(e) => setId(e.target.value)}
                placeholder="e.g. alice, my-persona"
                autoFocus
              />
              {idError && (
                <span
                  style={{
                    fontSize: 11,
                    color: "oklch(0.62 0.2 25)",
                    marginTop: 2,
                  }}
                >
                  Only letters, numbers, hyphens, and underscores (1-64 chars)
                </span>
              )}
            </label>

            <label className="settings-label">
              Name
              <input
                type="text"
                className="settings-input"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Display name"
              />
            </label>

            {error && (
              <div
                style={{
                  fontSize: 12,
                  color: "oklch(0.62 0.2 25)",
                  padding: "8px 10px",
                  background: "oklch(0.62 0.2 25 / 0.08)",
                  borderRadius: 6,
                  border: "1px solid oklch(0.62 0.2 25 / 0.2)",
                }}
              >
                {error}
              </div>
            )}
          </div>

          <DialogFooter style={{ marginTop: 18 }}>
            <button
              type="button"
              className="management-btn management-btn--secondary"
              onClick={() => handleOpenChange(false)}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="management-btn"
              disabled={!formValid || submitting}
            >
              {submitting ? "Creating..." : "Create"}
            </button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
