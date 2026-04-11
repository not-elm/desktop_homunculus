interface FirstPersonPronounFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function FirstPersonPronounField({
  value,
  onChange,
  disabled,
}: FirstPersonPronounFieldProps) {
  return (
    <label className="settings-label">
      First Person Pronoun
      <input
        type="text"
        className="settings-input"
        value={value}
        placeholder="e.g. watashi, boku, ore"
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
      />
    </label>
  );
}
