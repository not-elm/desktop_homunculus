import type { Gender } from "@hmcs/sdk";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@hmcs/ui";

interface PersonaTabProps {
  name: string;
  displayName: string;
  onDisplayNameChange: (displayName: string) => void;
  age: number | null;
  ageUnknown: boolean;
  onAgeChange: (age: number | null) => void;
  onAgeUnknownChange: (unknown: boolean) => void;
  gender: Gender;
  onGenderChange: (gender: Gender) => void;
  firstPersonPronoun: string;
  onFirstPersonPronounChange: (pronoun: string) => void;
  profile: string;
  personality: string;
  onProfileChange: (profile: string) => void;
  onPersonalityChange: (personality: string) => void;
}

const GENDER_OPTIONS: { value: Gender; label: string }[] = [
  { value: "unknown", label: "不明" },
  { value: "male", label: "男" },
  { value: "female", label: "女" },
  { value: "other", label: "その他" },
];

export function PersonaTab({
  name,
  displayName,
  onDisplayNameChange,
  age,
  ageUnknown,
  onAgeChange,
  onAgeUnknownChange,
  gender,
  onGenderChange,
  firstPersonPronoun,
  onFirstPersonPronounChange,
  profile,
  personality,
  onProfileChange,
  onPersonalityChange,
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

      <AgeField
        age={age}
        ageUnknown={ageUnknown}
        onAgeChange={onAgeChange}
        onAgeUnknownChange={onAgeUnknownChange}
      />

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
          placeholder="例: わたし、僕、俺"
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

      <label className="settings-label">
        Personality
        <textarea
          className="settings-textarea"
          rows={5}
          value={personality}
          onChange={(e) => onPersonalityChange(e.target.value)}
          placeholder="Personality traits in natural language..."
        />
      </label>
    </div>
  );
}

interface AgeFieldProps {
  age: number | null;
  ageUnknown: boolean;
  onAgeChange: (age: number | null) => void;
  onAgeUnknownChange: (unknown: boolean) => void;
}

function AgeField({ age, ageUnknown, onAgeChange, onAgeUnknownChange }: AgeFieldProps) {
  function handleUnknownToggle(checked: boolean) {
    onAgeUnknownChange(checked);
    if (checked) onAgeChange(null);
  }

  function handleAgeInput(value: string) {
    const num = parseInt(value, 10);
    onAgeChange(isNaN(num) ? null : Math.min(Math.max(num, 0), 999));
  }

  return (
    <div className="settings-label">
      Age
      <div className="settings-age-row">
        <input
          type="number"
          className="settings-input settings-age-input"
          min={0}
          max={999}
          value={ageUnknown ? "" : (age ?? "")}
          disabled={ageUnknown}
          onChange={(e) => handleAgeInput(e.target.value)}
          placeholder="—"
        />
        <label className="settings-checkbox-label">
          <input
            type="checkbox"
            checked={ageUnknown}
            onChange={(e) => handleUnknownToggle(e.target.checked)}
          />
          年齢不詳
        </label>
      </div>
    </div>
  );
}
