interface PersonaTabProps {
  profile: string;
  personality: string;
  onProfileChange: (profile: string) => void;
  onPersonalityChange: (personality: string) => void;
}

export function PersonaTab({
  profile,
  personality,
  onProfileChange,
  onPersonalityChange,
}: PersonaTabProps) {
  return (
    <div className="settings-section">
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
