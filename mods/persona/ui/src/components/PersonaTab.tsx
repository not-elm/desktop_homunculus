import { useMemo } from "react";
import { Persona, type Gender } from "@hmcs/sdk";
import { PersonaFields, type PersonaFormValues } from "@persona/shared/components/PersonaFields";
import { ThumbnailCard } from "@persona/shared/components/ThumbnailCard";
import { useThumbnailImport } from "@persona/shared/hooks/useThumbnailImport";

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

  const values: PersonaFormValues = {
    name,
    age,
    gender,
    firstPersonPronoun,
    profile,
    personality,
  };

  function handleChange(updated: PersonaFormValues) {
    if (updated.name !== name) onNameChange(updated.name);
    if (updated.age !== age) onAgeChange(updated.age);
    if (updated.gender !== gender) onGenderChange(updated.gender);
    if (updated.firstPersonPronoun !== firstPersonPronoun) onFirstPersonPronounChange(updated.firstPersonPronoun);
    if (updated.profile !== profile) onProfileChange(updated.profile);
    if (updated.personality !== personality) onPersonalityChange(updated.personality);
  }

  return (
    <div>
      <ThumbnailCard thumbnailUrl={thumbnailUrl} onThumbnailChange={handleThumbnailClick} />
      <PersonaFields values={values} onChange={handleChange} />
    </div>
  );
}
