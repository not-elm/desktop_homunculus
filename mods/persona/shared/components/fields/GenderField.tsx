import type { Gender } from '@hmcs/sdk';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@hmcs/ui';

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
    <div className="settings-label">
      Gender
      <Select value={value} onValueChange={(v) => onChange(v as Gender)} disabled={disabled}>
        <SelectTrigger className="settings-input">
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
    </div>
  );
}
