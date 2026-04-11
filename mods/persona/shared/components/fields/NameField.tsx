interface NameFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function NameField({ value, onChange, disabled }: NameFieldProps) {
  return (
    <label className="settings-label">
      Name
      <input
        type="text"
        className="settings-input"
        value={value}
        placeholder="Name"
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
      />
    </label>
  );
}
