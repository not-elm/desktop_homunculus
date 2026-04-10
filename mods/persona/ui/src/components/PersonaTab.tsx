import { useMemo } from "react";
import { Persona } from "@hmcs/sdk";
import type { Gender } from "@hmcs/sdk";
import { ThumbnailCard } from "@persona/shared/components/ThumbnailCard";
import { useThumbnailImport } from "@persona/shared/hooks/useThumbnailImport";
import { NameField } from "@persona/shared/components/fields/NameField";
import { AgeField } from "@persona/shared/components/fields/AgeField";
import { GenderField } from "@persona/shared/components/fields/GenderField";
import { FirstPersonPronounField } from "@persona/shared/components/fields/FirstPersonPronounField";
import { ProfileField } from "@persona/shared/components/fields/ProfileField";
import { PersonalityField } from "@persona/shared/components/fields/PersonalityField";

interface PersonaTabProps {
  personaId: string;
  thumbnail: string | null;
  onThumbnailChange: (id: string | null) => void;
  name: string;
  onNameChange: (name: string) => void;
  age: number | null;
  onAgeChange: (age: number | null) => void;
  gender: Gender;
  onGenderChange: (gender: Gender) => void;
  firstPersonPronoun: string;
  onFirstPersonPronounChange: (pronoun: string) => void;
  profile: string;
  onProfileChange: (profile: string) => void;
  personality: string;
  onPersonalityChange: (personality: string) => void;
}

export function PersonaTab({
  personaId,
  thumbnail,
  onThumbnailChange,
  name,
  onNameChange,
  age,
  onAgeChange,
  gender,
  onGenderChange,
  firstPersonPronoun,
  onFirstPersonPronounChange,
  profile,
  onProfileChange,
  personality,
  onPersonalityChange,
}: PersonaTabProps) {
  const { importThumbnail } = useThumbnailImport();
  const persona = useMemo(() => new Persona(personaId), [personaId]);

  async function handleThumbnailClick() {
    const assetId = await importThumbnail(personaId);
    if (assetId) {
      onThumbnailChange(assetId);
    }
  }

  const thumbnailUrl = persona.thumbnailUrl(thumbnail);

  return (
    <div>
      <div className="persona-tab-header">
        <ThumbnailCard
          className="thumb-inline"
          thumbnailUrl={thumbnailUrl}
          onThumbnailChange={handleThumbnailClick}
        />
        <div className="persona-tab-header-fields">
          <NameField value={name} onChange={onNameChange} />
          <AgeField value={age} onChange={onAgeChange} />
        </div>
      </div>
      <div className="settings-section">
        <GenderField value={gender} onChange={onGenderChange} />
        <FirstPersonPronounField value={firstPersonPronoun} onChange={onFirstPersonPronounChange} />
        <ProfileField value={profile} onChange={onProfileChange} />
        <PersonalityField value={personality} onChange={onPersonalityChange} />
      </div>
    </div>
  );
}
