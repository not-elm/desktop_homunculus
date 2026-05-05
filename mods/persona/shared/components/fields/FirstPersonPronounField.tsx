import { Input, Label } from '@hmcs/ui';

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
    <Label className="flex flex-col items-start gap-1.5 text-xs uppercase tracking-[0.1em] text-primary/70">
      First Person Pronoun
      <Input
        value={value}
        placeholder="e.g. watashi, boku, ore"
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
      />
    </Label>
  );
}
