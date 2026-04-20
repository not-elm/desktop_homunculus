import type { PersonaCacheEntry } from '../persona-cache.js';

export function renderIdentity(persona: PersonaCacheEntry, maxChars: number): string {
  const name = persona.name.trim() || persona.personaId;
  const age = persona.age === null ? 'unknown' : String(persona.age);
  const gender = persona.gender && persona.gender.trim() ? persona.gender : 'unknown';

  const lines = [
    '# Name',
    name,
    '',
    '# Age',
    age,
    '',
    '# Gender',
    gender,
    '',
    `# DH persona id: ${persona.personaId}`,
  ];
  const out = lines.join('\n') + '\n';
  if (out.length <= maxChars) return out;
  return out.slice(0, Math.max(0, maxChars));
}
