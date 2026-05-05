import type { Gender } from '@hmcs/sdk';
import { Label, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@hmcs/ui';

const GENDER_OPTIONS: { value: Gender; label: string }[] = [
  { value: 'unknown', label: 'Unknown' },
  { value: 'male', label: 'Male' },
  { value: 'female', label: 'Female' },
  { value: 'other', label: 'Other' },
];

interface GenderFieldProps {
  value: Gender;
  onChange: (value: Gender) => void;
  disabled?: boolean;
}

export function GenderField({ value, onChange, disabled }: GenderFieldProps) {
  return (
    <Label className="flex flex-col items-start gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
      Gender
      <Select value={value} onValueChange={(v) => onChange(v as Gender)} disabled={disabled}>
        <SelectTrigger className="w-full">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {GENDER_OPTIONS.map((opt) => (
            <SelectItem key={opt.value} value={opt.value}>
              {opt.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </Label>
  );
}
