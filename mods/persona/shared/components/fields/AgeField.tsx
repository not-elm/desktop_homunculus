import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import { useRef } from 'react';
import type { AgeValue } from '../PersonaFields';

interface AgeFieldProps {
  value: AgeValue;
  onChange: (age: AgeValue) => void;
  disabled?: boolean;
}

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
    <fieldset className="settings-label settings-age-field" disabled={disabled}>
      <legend className="settings-age-legend">Age</legend>
      <RadioGroupPrimitive.Root
        className="settings-age-segments"
        value={mode}
        onValueChange={handleModeChange}
        data-mode={mode}
        disabled={disabled}
      >
        <RadioGroupPrimitive.Item
          value="specify"
          className="settings-age-segment"
          aria-label="Specify age"
          data-mode="specify"
        >
          Specify
        </RadioGroupPrimitive.Item>
        <RadioGroupPrimitive.Item
          value="unknown"
          className="settings-age-segment"
          aria-label="Age unknown"
          data-mode="unknown"
        >
          Unknown
        </RadioGroupPrimitive.Item>
      </RadioGroupPrimitive.Root>
      <div className="settings-age-value-area" role="status" aria-live="polite" data-mode={mode}>
        {mode === 'unknown' ? (
          <span className="settings-age-unknown-readout">Unknown</span>
        ) : (
          <input
            ref={inputRef}
            type="text"
            inputMode="numeric"
            pattern="[0-9]*"
            className="settings-age-input"
            value={String(value.age)}
            onChange={(e) => handleInput(e.target.value)}
            aria-label="Age value"
            disabled={disabled}
          />
        )}
      </div>
    </fieldset>
  );
}
