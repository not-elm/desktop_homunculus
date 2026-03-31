import type { Gender } from "@hmcs/sdk";
import { useRef } from "react";
import * as RadioGroupPrimitive from "@radix-ui/react-radio-group";
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from "@hmcs/ui";

interface PersonaTabProps {
  name: string;
  displayName: string;
  onDisplayNameChange: (displayName: string) => void;
  age: number | null;
  onAgeChange: (age: number | null) => void;
  gender: Gender;
  onGenderChange: (gender: Gender) => void;
  firstPersonPronoun: string;
  onFirstPersonPronounChange: (pronoun: string) => void;
  profile: string;
  onProfileChange: (profile: string) => void;
}

const GENDER_OPTIONS: { value: Gender; label: string }[] = [
  { value: "unknown", label: "Unknown" },
  { value: "male", label: "Male" },
  { value: "female", label: "Female" },
  { value: "other", label: "Other" },
];

export function PersonaTab({
  name,
  displayName,
  onDisplayNameChange,
  age,
  onAgeChange,
  gender,
  onGenderChange,
  firstPersonPronoun,
  onFirstPersonPronounChange,
  profile,
  onProfileChange,
}: PersonaTabProps) {
  return (
    <div className="settings-section">
      <label className="settings-label">
        Display Name
        <input
          type="text"
          className="settings-input"
          value={displayName}
          placeholder={name}
          onChange={(e) => onDisplayNameChange(e.target.value)}
        />
      </label>

      <AgeField value={age} onChange={onAgeChange} />

      <div className="settings-label">
        Gender
        <Select value={gender} onValueChange={(v) => onGenderChange(v as Gender)}>
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

      <label className="settings-label">
        First Person Pronoun
        <input
          type="text"
          className="settings-input"
          value={firstPersonPronoun}
          placeholder="e.g. watashi, boku, ore"
          onChange={(e) => onFirstPersonPronounChange(e.target.value)}
        />
      </label>

      <label className="settings-label">
        Profile
        <textarea
          className="settings-textarea"
          rows={5}
          value={profile}
          onChange={(e) => onProfileChange(e.target.value)}
          placeholder="Character background and profile description..."
        />
      </label>

    </div>
  );
}

interface AgeFieldProps {
  value: number | null;
  onChange: (age: number | null) => void;
}

function AgeField({ value, onChange }: AgeFieldProps) {
  const preservedAge = useRef<number | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const isUnknown = value == null && preservedAge.current != null;
  const radioValue = isUnknown ? "unknown" : "specify";

  function handleModeChange(newMode: string) {
    if (newMode === "unknown") {
      if (value != null) preservedAge.current = value;
      onChange(null);
    } else {
      const restored = preservedAge.current;
      if (restored != null) onChange(restored);
      requestAnimationFrame(() => inputRef.current?.focus());
    }
  }

  function handleInput(raw: string) {
    const digits = raw.replace(/[^0-9]/g, "");
    if (digits === "") {
      onChange(null);
      return;
    }
    onChange(Math.min(parseInt(digits, 10), 999));
  }

  return (
    <fieldset className="settings-label settings-age-field">
      <legend className="settings-age-legend">Age</legend>
      <RadioGroupPrimitive.Root
        className="settings-age-segments"
        value={radioValue}
        onValueChange={handleModeChange}
        data-mode={radioValue === "unknown" ? "unknown" : "specify"}
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
      <div
        className="settings-age-value-area"
        role="status"
        aria-live="polite"
        data-mode={radioValue === "unknown" ? "unknown" : "specify"}
      >
        {radioValue === "unknown" ? (
          <span className="settings-age-unknown-readout">Unknown</span>
        ) : (
          <>
            <input
              ref={inputRef}
              type="text"
              inputMode="numeric"
              pattern="[0-9]*"
              className="settings-age-input"
              value={value ?? ""}
              onChange={(e) => handleInput(e.target.value)}
              aria-label="Age value"
              placeholder="—"
            />
          </>
        )}
      </div>
    </fieldset>
  );
}
