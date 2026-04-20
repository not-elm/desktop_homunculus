import { describe, expect, test } from 'vitest';
import type { PersonaCacheEntry } from '../persona-cache.js';
import { renderIdentity } from './identity.js';

function makePersona(overrides: Partial<PersonaCacheEntry> = {}): PersonaCacheEntry {
  return {
    personaId: 'alice',
    name: 'Alice',
    personality: null,
    profile: null,
    age: null,
    gender: 'unknown',
    firstPersonPronoun: null,
    ttsModName: '@hmcs/voicevox',
    spawned: true,
    hasWarnedNoAgent: false,
    lastRenderedHash: null,
    ...overrides,
  };
}

describe('renderIdentity', () => {
  test('renders all fields when present', () => {
    const out = renderIdentity(
      makePersona({
        name: 'Alice',
        age: 20,
        gender: 'female',
      }),
      10000,
    );
    expect(out).toContain('# Name');
    expect(out).toContain('Alice');
    expect(out).toContain('# Age');
    expect(out).toContain('20');
    expect(out).toContain('# Gender');
    expect(out).toContain('female');
    expect(out).toContain('# HMCS persona id: alice');
  });

  test('uses "unknown" placeholders when age is null and gender is unknown', () => {
    const out = renderIdentity(makePersona({ age: null, gender: 'unknown' }), 10000);
    expect(out).toContain('unknown');
  });

  test('falls back to personaId when name is null', () => {
    const out = renderIdentity(makePersona({ name: null, personaId: 'elmer' }), 10000);
    expect(out).toContain('elmer');
  });

  test('falls back to personaId when name is whitespace only', () => {
    const out = renderIdentity(makePersona({ name: '   ', personaId: 'elmer' }), 10000);
    expect(out).toContain('elmer');
  });

  test('idempotent', () => {
    const p = makePersona({ age: 30, gender: 'other' });
    expect(renderIdentity(p, 10000)).toBe(renderIdentity(p, 10000));
  });
});
