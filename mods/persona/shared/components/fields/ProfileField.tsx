interface ProfileFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function ProfileField({ value, onChange, disabled }: ProfileFieldProps) {
  return (
    <label className="settings-label">
      Profile
      <textarea
        className="settings-textarea"
        rows={5}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="Character background and profile description..."
        disabled={disabled}
      />
    </label>
  );
}
