import { Label, Textarea } from '@hmcs/ui';

interface ProfileFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function ProfileField({ value, onChange, disabled }: ProfileFieldProps) {
  return (
    <Label className="flex flex-col items-start gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
      Profile
      <Textarea
        rows={5}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="Character background and profile description..."
        disabled={disabled}
      />
    </Label>
  );
}
