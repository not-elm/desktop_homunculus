import * as RadioGroupPrimitive from '@radix-ui/react-radio-group';
import { useEffect, useRef, useState } from 'react';

type Mode = 'specify' | 'unknown';

interface AgeFieldProps {
  value: number | null;
  onChange: (age: number | null) => void;
  disabled?: boolean;
}

export function AgeField({ value, onChange, disabled }: AgeFieldProps) {
  const preservedAge = useRef<number | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const prevValue = useRef(value);
  const [mode, setMode] = useState<Mode>(value == null ? 'unknown' : 'specify');

  useEffect(() => {
    if (prevValue.current !== value) {
      setMode(value == null ? 'unknown' : 'specify');
      prevValue.current = value;
    }
  }, [value]);

  function handleModeChange(newMode: string) {
    if (disabled) return;
    if (newMode === 'unknown') {
      if (value != null) preservedAge.current = value;
      setMode('unknown');
      onChange(null);
    } else {
      setMode('specify');
      const restored = preservedAge.current;
      if (restored != null) onChange(restored);
      requestAnimationFrame(() => inputRef.current?.focus());
    }
  }

  function handleInput(raw: string) {
    const digits = raw.replace(/[^0-9]/g, '');
    if (digits === '') {
      onChange(null);
      return;
    }
    const age = Math.min(parseInt(digits, 10), 999);
    preservedAge.current = age;
    onChange(age);
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
            value={value ?? ''}
            onChange={(e) => handleInput(e.target.value)}
            aria-label="Age value"
            placeholder="&#x2014;"
            disabled={disabled}
          />
        )}
      </div>
    </fieldset>
  );
}
