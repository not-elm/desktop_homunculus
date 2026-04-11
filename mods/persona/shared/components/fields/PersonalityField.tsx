interface PersonalityFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function PersonalityField({ value, onChange, disabled }: PersonalityFieldProps) {
  return (
    <label className="settings-label">
      Personality
      <textarea
        className="settings-textarea"
        rows={3}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="e.g. Sarcastic but caring, uses formal speech"
        disabled={disabled}
      />
    </label>
  );
}
