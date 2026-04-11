import { describe, expect, it } from 'vitest';
import { buildWorkerPrompt } from './worker.ts';
import type { Persona } from './types.ts';

const testPersona: Persona = {
  name: 'Alice',
  age: 22,
  gender: 'female',
  firstPersonPronoun: 'watashi',
  profile: 'A helpful assistant.',
  personality: null,
};

describe('buildWorkerPrompt', () => {
  it('includes the task description in the prompt', () => {
    const prompt = buildWorkerPrompt(testPersona, {
      taskDescription: 'Refactor the login page',
    });
    expect(prompt).toContain('Refactor the login page');
  });

  it('includes the persona identity so Workers stay in character', () => {
    const prompt = buildWorkerPrompt(testPersona, {
      taskDescription: 'Write tests',
    });
    expect(prompt).toContain('Alice');
  });

  it('includes worktree context when provided', () => {
    const prompt = buildWorkerPrompt(testPersona, {
      taskDescription: 'Fix bug',
      worktree: {
        worktreeName: 'feature-x',
        baseBranch: 'main',
        worktreePath: '/tmp/wt',
      },
    });
    expect(prompt).toContain('feature-x');
  });
});
