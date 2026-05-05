import { Label, Textarea } from '@hmcs/ui';

interface PersonalityFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function PersonalityField({ value, onChange, disabled }: PersonalityFieldProps) {
  return (
    <Label className="flex flex-col items-start gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
      Personality
      <Textarea
        rows={3}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="e.g. Sarcastic but caring, uses formal speech"
        disabled={disabled}
      />
    </Label>
  );
}
