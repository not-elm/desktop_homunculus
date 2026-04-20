import type { OpenClawPluginApi } from 'openclaw/plugin-sdk/plugin-entry';
import { describe, expect, test, vi } from 'vitest';
import pluginEntry from './entry.js';

interface EntryShape {
  register?: (api: OpenClawPluginApi) => void;
  default?: { register?: (api: OpenClawPluginApi) => void };
}

describe('entry', () => {
  test('registers bootstrap hook + persona-sync service', () => {
    const registerTool = vi.fn();
    const registerHook = vi.fn();
    const on = vi.fn();
    const registerService = vi.fn();
    const logger = { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
    const api = {
      id: 'hmcs-openclaw',
      name: 'Desktop Homunculus Bridge',
      source: 'test',
      registrationMode: 'full',
      config: {},
      runtime: {
        agent: {
          resolveAgentWorkspaceDir: vi.fn(),
          resolveAgentDir: vi.fn(),
        },
      },
      logger,
      registerTool,
      registerHook,
      registerService,
      on,
    } as unknown as OpenClawPluginApi;

    const entry = pluginEntry as EntryShape;
    const register = entry.register ?? entry.default?.register;
    expect(typeof register).toBe('function');
    register!(api);

    expect(registerTool).not.toHaveBeenCalled();
    // No api.on subscriptions in this stage — message hooks arrive in a follow-up PR.
    expect(on).not.toHaveBeenCalled();

    expect(registerHook).toHaveBeenCalledWith('agent:bootstrap', expect.any(Function), {
      name: 'hmcs-openclaw.bootstrap',
    });
    expect(registerService).toHaveBeenCalledWith(
      expect.objectContaining({
        id: 'persona-sync',
        start: expect.any(Function),
        stop: expect.any(Function),
      }),
    );
  });
});
