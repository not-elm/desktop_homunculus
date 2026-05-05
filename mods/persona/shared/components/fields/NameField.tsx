import { Input, Label } from '@hmcs/ui';

interface NameFieldProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

export function NameField({ value, onChange, disabled }: NameFieldProps) {
  return (
    <Label className="flex flex-col items-start gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
      Name
      <Input
        value={value}
        placeholder="Name"
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
      />
    </Label>
  );
}
