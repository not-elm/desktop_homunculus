import { describe, expect, it, vi } from 'vitest';
import { buildFrontmanPrompt, createFrontmanRuntime } from './frontman.ts';
import { DEFAULT_SETTINGS, type AgentSettings, type Persona } from './types.ts';

vi.mock('./runtime/claude-agent-runtime.ts', () => ({
  ClaudeAgentRuntime: vi.fn(),
}));
vi.mock('./runtime/codex-appserver-runtime.ts', () => ({
  CodexAppServerRuntime: vi.fn(),
}));

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

describe('createFrontmanRuntime', () => {
  it('uses lightweight Claude model for SDK runtime regardless of user setting', async () => {
    const { ClaudeAgentRuntime } = await import('./runtime/claude-agent-runtime.ts');
    const settings: AgentSettings = { ...DEFAULT_SETTINGS, runtime: 'sdk', claudeModel: 'claude-sonnet-4-20250514' };
    createFrontmanRuntime({
      settings,
      prompt: 'test',
      apiKey: 'sk-test',
      workDir: '/tmp',
      appServerProcess: {} as never,
    });
    expect(ClaudeAgentRuntime).toHaveBeenCalledWith(
      'test',
      expect.objectContaining({ claudeModel: 'claude-haiku-4-5-20251001' }),
      'sk-test',
      '/tmp',
    );
  });

  it('uses lightweight Codex model for codex runtime regardless of user setting', async () => {
    const { CodexAppServerRuntime } = await import('./runtime/codex-appserver-runtime.ts');
    const settings: AgentSettings = { ...DEFAULT_SETTINGS, runtime: 'codex', claudeModel: 'gpt-5.3-codex' };
    const mockProcess = {} as never;
    createFrontmanRuntime({
      settings,
      prompt: 'test',
      apiKey: null,
      workDir: '/tmp',
      appServerProcess: mockProcess,
    });
    expect(CodexAppServerRuntime).toHaveBeenCalledWith(
      'test',
      expect.objectContaining({ claudeModel: 'gpt-5.1-codex-mini' }),
      '/tmp',
      mockProcess,
    );
  });
});
