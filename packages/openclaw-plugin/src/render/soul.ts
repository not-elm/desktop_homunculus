import type { PersonaCacheEntry } from '../persona-cache.js';

const PROFILE_MAX_LINES = 5;

export function renderSoul(persona: PersonaCacheEntry, maxChars: number): string {
  const toneSection =
    persona.personality && persona.personality.trim()
      ? `# Tone\n${persona.personality.trim()}\n`
      : null;

  const profileLines =
    persona.profile && persona.profile.trim()
      ? persona.profile
          .split('\n')
          .map((l) => l.trim())
          .filter(Boolean)
          .slice(0, PROFILE_MAX_LINES)
      : [];
  const profileSection =
    profileLines.length > 0 ? `# Profile note\n${profileLines.join('\n')}\n` : null;

  const pronounSection =
    persona.firstPersonPronoun && persona.firstPersonPronoun.trim()
      ? `# First-person pronoun\n${persona.firstPersonPronoun.trim()}\n`
      : null;

  // Priority order for truncation: tone > pronoun > profile (drop profile first)
  const sections = [toneSection, profileSection, pronounSection].filter(Boolean) as string[];
  let out = sections.join('\n');
  if (out.length <= maxChars) return out;

  // Over budget: drop profile first, then truncate tone
  const essentials = [toneSection, pronounSection].filter(Boolean) as string[];
  out = essentials.join('\n');
  if (out.length <= maxChars) return out;

  // Still over: hard truncate
  return out.slice(0, Math.max(0, maxChars));
}
