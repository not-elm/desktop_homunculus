import { cn, Input } from '@hmcs/ui';
import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import { useRef } from 'react';
import type { AgeValue } from '../PersonaFields';

interface AgeFieldProps {
  value: AgeValue;
  onChange: (age: AgeValue) => void;
  disabled?: boolean;
}

const segmentsClasses = cn(
  'flex h-8 gap-0.5 overflow-hidden rounded-md border bg-input',
  'border-primary/20',
  '[&[data-mode=unknown]]:border-holo-violet/20',
);

const segmentClasses = cn(
  'flex flex-1 cursor-pointer items-center justify-center border-b-2 border-transparent bg-transparent py-1.5 text-xs font-medium uppercase tracking-[0.08em] transition-colors',
  'text-muted-foreground',
  '[&[data-mode=specify]]:hover:text-primary',
  '[&[data-mode=unknown]]:hover:text-holo-violet',
  '[&[data-state=checked][data-mode=specify]]:[border-bottom-color:color-mix(in_oklab,var(--primary),transparent_20%)] [&[data-state=checked][data-mode=specify]]:bg-hud-surface-elevated [&[data-state=checked][data-mode=specify]]:text-primary [&[data-state=checked][data-mode=specify]]:[text-shadow:0_0_12px_color-mix(in_oklab,var(--primary),transparent_70%)]',
  '[&[data-state=checked][data-mode=unknown]]:[border-bottom-color:color-mix(in_oklab,var(--holo-violet),transparent_20%)] [&[data-state=checked][data-mode=unknown]]:bg-hud-surface-elevated [&[data-state=checked][data-mode=unknown]]:text-holo-violet [&[data-state=checked][data-mode=unknown]]:[text-shadow:0_0_12px_color-mix(in_oklab,var(--holo-violet),transparent_70%)]',
  'focus-visible:outline-2 focus-visible:outline-primary focus-visible:outline-offset-2',
  'disabled:cursor-not-allowed disabled:opacity-50',
);

const unknownDisplayClasses =
  'flex h-9 w-full items-center justify-center rounded-lg border border-holo-violet/20 bg-input text-center font-mono text-[0.95rem] text-holo-violet tabular-nums';

const ageInputClasses = cn(
  'h-9 text-center font-mono text-[0.95rem] tabular-nums',
  '[&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none',
);

export function AgeField({ value, onChange, disabled }: AgeFieldProps) {
  const preservedAge = useRef(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const mode = value.type;

  function handleModeChange(newMode: string) {
    if (disabled) return;
    if (newMode === 'unknown') {
      if (value.type === 'specify') preservedAge.current = value.age;
      onChange({ type: 'unknown' });
    } else {
      onChange({ type: 'specify', age: preservedAge.current });
      requestAnimationFrame(() => inputRef.current?.focus());
    }
  }

  function handleInput(raw: string) {
    const digits = raw.replace(/[^0-9]/g, '');
    if (digits === '') {
      onChange({ type: 'specify', age: 0 });
      return;
    }
    const age = parseInt(digits, 10);
    preservedAge.current = age;
    onChange({ type: 'specify', age });
  }

  return (
    <fieldset
      className="m-0 flex min-w-0 flex-col gap-1.5 border-0 p-0 text-xs uppercase tracking-[0.1em] text-primary/70"
      disabled={disabled}
    >
      <legend className="sr-only">Age</legend>
      <RadioGroupPrimitive.Root
        className={segmentsClasses}
        value={mode}
        onValueChange={handleModeChange}
        data-mode={mode}
        disabled={disabled}
      >
        <RadioGroupPrimitive.Item
          value="specify"
          className={segmentClasses}
          aria-label="Specify age"
          data-mode="specify"
        >
          Specify
        </RadioGroupPrimitive.Item>
        <RadioGroupPrimitive.Item
          value="unknown"
          className={segmentClasses}
          aria-label="Age unknown"
          data-mode="unknown"
        >
          Unknown
        </RadioGroupPrimitive.Item>
      </RadioGroupPrimitive.Root>
      {mode === 'unknown' ? (
        <div className={unknownDisplayClasses} role="status" aria-live="polite">
          Unknown
        </div>
      ) : (
        <Input
          ref={inputRef}
          type="text"
          inputMode="numeric"
          pattern="[0-9]*"
          className={ageInputClasses}
          value={String(value.age)}
          onChange={(e) => handleInput(e.target.value)}
          aria-label="Age value"
          disabled={disabled}
        />
      )}
    </fieldset>
  );
}
