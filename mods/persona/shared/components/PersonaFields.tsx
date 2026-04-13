import type { Gender } from '@hmcs/sdk';
import { AgeField } from './fields/AgeField';
import { FirstPersonPronounField } from './fields/FirstPersonPronounField';
import { GenderField } from './fields/GenderField';
import { NameField } from './fields/NameField';
import { PersonalityField } from './fields/PersonalityField';
import { ProfileField } from './fields/ProfileField';

export type AgeValue = { type: 'specify'; age: number } | { type: 'unknown' };

export interface PersonaFormValues {
  name: string;
  age: AgeValue;
  gender: Gender;
  firstPersonPronoun: string;
  profile: string;
  personality: string;
}

interface PersonaFieldsProps {
  values: PersonaFormValues;
  onChange: (values: PersonaFormValues) => void;
  disabled?: boolean;
}

export function PersonaFields({ values, onChange, disabled }: PersonaFieldsProps) {
  function update<K extends keyof PersonaFormValues>(key: K, value: PersonaFormValues[K]) {
    onChange({ ...values, [key]: value });
  }

  return (
    <div className="settings-section">
      <NameField value={values.name} onChange={(v) => update('name', v)} disabled={disabled} />
      <AgeField value={values.age} onChange={(v) => update('age', v)} disabled={disabled} />
      <GenderField
        value={values.gender}
        onChange={(v) => update('gender', v)}
        disabled={disabled}
      />
      <FirstPersonPronounField
        value={values.firstPersonPronoun}
        onChange={(v) => update('firstPersonPronoun', v)}
        disabled={disabled}
      />
      <ProfileField
        value={values.profile}
        onChange={(v) => update('profile', v)}
        disabled={disabled}
      />
      <PersonalityField
        value={values.personality}
        onChange={(v) => update('personality', v)}
        disabled={disabled}
      />
    </div>
  );
}
