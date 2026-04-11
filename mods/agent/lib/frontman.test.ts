import { describe, expect, it } from 'vitest';
import { buildFrontmanPrompt } from './frontman.ts';
import type { Persona } from './types.ts';

const testPersona: Persona = {
  name: 'Alice',
  age: 22,
  gender: 'female',
  firstPersonPronoun: 'watashi',
  profile: 'Helpful.',
  personality: null,
};

describe('buildFrontmanPrompt', () => {
  it('instructs Frontman to delegate implementation work', () => {
    const prompt = buildFrontmanPrompt(testPersona);
    expect(prompt).toMatch(/delegate/i);
  });

  it('instructs Frontman to stay conversational', () => {
    const prompt = buildFrontmanPrompt(testPersona);
    expect(prompt).toMatch(/1.*3.*sentence/i);
  });

  it('includes persona identity', () => {
    const prompt = buildFrontmanPrompt(testPersona);
    expect(prompt).toContain('Alice');
  });

  it('mentions delegate-task as the implementation tool', () => {
    const prompt = buildFrontmanPrompt(testPersona);
    expect(prompt).toContain('delegate-task');
  });
});
