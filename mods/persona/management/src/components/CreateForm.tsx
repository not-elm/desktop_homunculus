import { Button, Input, Label } from '@hmcs/ui';
import { useMemo, useState } from 'react';

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
    <div className="no-scrollbar flex flex-1 items-start justify-center overflow-y-auto p-5">
      <form className="flex w-full max-w-md flex-col gap-4" onSubmit={handleSubmit}>
        <h2 className="text-xs font-semibold uppercase tracking-[0.15em] text-primary">
          Create Persona
        </h2>

        <Label className="flex flex-col gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
          ID
          <Input
            value={id}
            onChange={(e) => setId(e.target.value)}
            placeholder="e.g. alice, my-persona"
            aria-invalid={idError || undefined}
          />
          {idError && (
            <span className="text-[0.7rem] tracking-[0.04em] text-holo-amber/80">
              Only letters, numbers, hyphens, and underscores (1-64 chars)
            </span>
          )}
        </Label>

        <Label className="flex flex-col gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
          Name
          <Input
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Display name"
          />
        </Label>

        {error && (
          <div className="rounded-md border border-destructive/40 bg-destructive/15 px-3 py-2 text-xs text-destructive">
            {error}
          </div>
        )}

        <div className="flex justify-end gap-2 pt-2">
          <Button variant="hud-ghost" size="hud" onClick={onCancel}>
            Cancel
          </Button>
          <Button variant="hud" size="hud" type="submit" disabled={!formValid || submitting}>
            {submitting ? 'Creating...' : 'Create'}
          </Button>
        </div>
      </form>
    </div>
  );
}
