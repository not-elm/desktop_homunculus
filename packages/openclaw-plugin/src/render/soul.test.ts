import { describe, expect, test } from 'vitest';
import type { PersonaCacheEntry } from '../persona-cache.js';
import { renderSoul } from './soul.js';

function makePersona(overrides: Partial<PersonaCacheEntry> = {}): PersonaCacheEntry {
  return {
    personaId: 'alice',
    name: 'Alice',
    personality: null,
    profile: null,
    age: null,
    gender: null,
    firstPersonPronoun: null,
    ttsModName: null,
    spawned: true,
    hasWarnedNoAgent: false,
    lastRenderedHash: null,
    ...overrides,
  };
}

describe('renderSoul', () => {
  test('renders personality/profile/pronoun when all present', () => {
    const out = renderSoul(
      makePersona({
        personality: 'Cheerful and curious.',
        profile: 'Lives on a desktop.\nLoves coffee.',
        firstPersonPronoun: '私',
      }),
      10000,
    );
    expect(out).toContain('# Tone');
    expect(out).toContain('Cheerful and curious.');
    expect(out).toContain('# Profile note');
    expect(out).toContain('Lives on a desktop.');
    expect(out).toContain('# First-person pronoun');
    expect(out).toContain('私');
  });

  test('omits empty sections', () => {
    const out = renderSoul(
      makePersona({ personality: 'Chirpy.', profile: null, firstPersonPronoun: null }),
      10000,
    );
    expect(out).toContain('# Tone');
    expect(out).toContain('Chirpy.');
    expect(out).not.toContain('# Profile note');
    expect(out).not.toContain('# First-person pronoun');
  });

  test('caps profile to 5 lines', () => {
    const profile = Array.from({ length: 10 }, (_, i) => `line${i + 1}`).join('\n');
    const out = renderSoul(makePersona({ profile }), 10000);
    const body = out.split('# Profile note\n')[1]!;
    const lineCount = body.split('\n').filter((l) => l.startsWith('line')).length;
    expect(lineCount).toBeLessThanOrEqual(5);
  });

  test('truncates when exceeding soulMaxChars, preserves personality', () => {
    const bigProfile = 'x'.repeat(20000);
    const out = renderSoul(
      makePersona({
        personality: 'Keep me.',
        profile: bigProfile,
      }),
      200,
    );
    expect(out.length).toBeLessThanOrEqual(200);
    expect(out).toContain('Keep me.');
  });

  test('idempotent: same persona → same output', () => {
    const persona = makePersona({
      personality: 'Kind.',
      profile: 'A\nB\nC',
      firstPersonPronoun: 'we',
    });
    expect(renderSoul(persona, 10000)).toBe(renderSoul(persona, 10000));
  });
});
