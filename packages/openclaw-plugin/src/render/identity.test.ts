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
    gender: null,
    firstPersonPronoun: null,
    ttsModName: null,
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
    expect(out).toContain('# DH persona id: alice');
  });

  test('uses "unknown" placeholders when fields are null', () => {
    const out = renderIdentity(makePersona({ age: null, gender: null }), 10000);
    expect(out).toContain('unknown');
  });

  test('idempotent', () => {
    const p = makePersona({ age: 30, gender: 'nb' });
    expect(renderIdentity(p, 10000)).toBe(renderIdentity(p, 10000));
  });
});
