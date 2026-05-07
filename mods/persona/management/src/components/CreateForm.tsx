import { useMemo, useState } from 'react';
import { managementBtnClasses, managementBtnSecondaryClasses } from '../styles';

interface CreateFormProps {
  onCreate: (id: string, name: string) => Promise<void>;
  onCancel: () => void;
}

const ID_PATTERN = /^[a-zA-Z0-9_-]{1,64}$/;

export default function CreateForm({ onCreate, onCancel }: CreateFormProps) {
  const [id, setId] = useState('');
  const [name, setName] = useState('');
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
      const message = (err as Error).message ?? 'Failed to create persona';
      setError(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="flex h-full items-center justify-center p-5">
      <form
        className="flex w-80 flex-col gap-3.5 rounded-lg border border-[oklch(0.72_0.14_192/0.15)] bg-[oklch(0.72_0.14_192/0.04)] p-6"
        onSubmit={handleSubmit}
      >
        <h2 className="m-0 mb-1 text-sm font-semibold tracking-[0.06em] text-[oklch(0.72_0.14_192/0.9)]">
          Create Persona
        </h2>

        <label className="settings-label">
          ID
          <input
            type="text"
            className="settings-input"
            value={id}
            onChange={(e) => setId(e.target.value)}
            placeholder="e.g. alice, my-persona"
          />
          {idError && (
            <span className="mt-0.5 text-[11px] text-[oklch(0.62_0.2_25)]">
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
          <div className="rounded-md border border-[oklch(0.62_0.2_25/0.2)] bg-[oklch(0.62_0.2_25/0.08)] px-2.5 py-2 text-xs text-[oklch(0.62_0.2_25)]">
            {error}
          </div>
        )}

        <div className="mt-1 flex justify-end gap-2">
          <button
            type="button"
            className={`${managementBtnClasses} ${managementBtnSecondaryClasses}`}
            onClick={onCancel}
          >
            Cancel
          </button>
          <button
            type="submit"
            className={managementBtnClasses}
            disabled={!formValid || submitting}
          >
            {submitting ? 'Creating...' : 'Create'}
          </button>
        </div>
      </form>
    </div>
  );
}
