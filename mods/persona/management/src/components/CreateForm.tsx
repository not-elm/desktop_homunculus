import { useState, useMemo } from "react";

interface CreateFormProps {
  onCreate: (id: string, name: string) => Promise<void>;
  onCancel: () => void;
}

const ID_PATTERN = /^[a-zA-Z0-9_-]{1,64}$/;

export default function CreateForm({ onCreate, onCancel }: CreateFormProps) {
  const [id, setId] = useState("");
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const idValid = useMemo(() => ID_PATTERN.test(id), [id]);
  const idError = id.length > 0 && !idValid;
  const formValid = idValid && name.trim().length > 0;

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!formValid || submitting) return;

    setSubmitting(true);
    setError(null);
    try {
      await onCreate(id, name.trim());
    } catch (err) {
      const message = (err as Error).message ?? "Failed to create persona";
      setError(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="create-form-wrapper">
      <form className="create-form" onSubmit={handleSubmit}>
        <h2 className="create-form-title">Create Persona</h2>

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
            <span className="create-form-error-hint">
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

        {error && <div className="create-form-error">{error}</div>}

        <div className="create-form-actions">
          <button
            type="button"
            className="management-btn management-btn--secondary"
            onClick={onCancel}
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
        </div>
      </form>
    </div>
  );
}
