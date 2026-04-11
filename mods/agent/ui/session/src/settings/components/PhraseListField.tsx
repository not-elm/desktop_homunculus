import { useState } from 'react';

type BadgeVariant = 'default' | 'violet' | 'green' | 'rose';

interface PhraseListFieldProps {
  label: string;
  description?: string;
  phrases: string[];
  onAdd: (phrase: string) => void;
  onRemove: (index: number) => void;
  badgeVariant?: BadgeVariant;
}

export function PhraseListField({
  label,
  description,
  phrases,
  onAdd,
  onRemove,
  badgeVariant = 'default',
}: PhraseListFieldProps) {
  const [inputValue, setInputValue] = useState('');

  function handleAdd() {
    const trimmed = inputValue.trim();
    if (!trimmed) return;
    onAdd(trimmed);
    setInputValue('');
  }

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === 'Enter') handleAdd();
  }

  return (
    <label className="settings-label">
      {label}
      {description && <span className="settings-label-desc">{description}</span>}
      <div className="agent-phrase-badges">
        {phrases.map((phrase, i) => (
          <PhraseBadge
            key={i}
            phrase={phrase}
            variant={badgeVariant}
            onRemove={() => onRemove(i)}
          />
        ))}
      </div>
      <div className="agent-add-row">
        <input
          className="agent-add-input"
          type="text"
          placeholder="Add phrase..."
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <button className="agent-add-btn" type="button" onClick={handleAdd}>
          Add
        </button>
      </div>
    </label>
  );
}

interface PhraseBadgeProps {
  phrase: string;
  variant: BadgeVariant;
  onRemove: () => void;
}

function PhraseBadge({ phrase, variant, onRemove }: PhraseBadgeProps) {
  const variantClass =
    variant === 'violet'
      ? 'agent-badge--violet'
      : variant === 'green'
        ? 'agent-badge--green'
        : variant === 'rose'
          ? 'agent-badge--rose'
          : 'agent-badge--cyan';

  return (
    <span className={`agent-badge ${variantClass}`}>
      {phrase}
      <button
        className="agent-badge-remove"
        type="button"
        onClick={onRemove}
        aria-label={`Remove ${phrase}`}
      >
        ×
      </button>
    </span>
  );
}
