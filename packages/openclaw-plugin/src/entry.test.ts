import { rpc } from '@hmcs/sdk/rpc';
import type { OpenClawPluginApi } from 'openclaw/plugin-sdk/plugin-entry';
import { afterEach, describe, expect, test, vi } from 'vitest';
import pluginEntry from './entry.js';

interface EntryShape {
  register?: (api: OpenClawPluginApi) => void;
  default?: { register?: (api: OpenClawPluginApi) => void };
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe('entry', () => {
  test('registers message hooks + bootstrap hook + persona-sync service', () => {
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
    register?.(api);

    expect(registerTool).not.toHaveBeenCalled();
    const hookNames = on.mock.calls.map((call) => call[0]);
    expect(hookNames).toEqual(['reply_dispatch', 'session_end']);

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

describe('speakViaTts', () => {
  test('skips rpc.call when resolveTtsModName returns null', async () => {
    const resolverModule = await import('./tts-resolver.js');
    vi.spyOn(resolverModule, 'resolveTtsModName').mockResolvedValue(null);
    const callSpy = vi.spyOn(rpc, 'call').mockResolvedValue(undefined);

    const { speakViaTts } = await import('./entry.js');
    await speakViaTts('alice', 'Hello, world!');

    expect(callSpy).not.toHaveBeenCalled();
  });

  test('forwards sanitized sentences to the resolved MOD', async () => {
    const resolverModule = await import('./tts-resolver.js');
    vi.spyOn(resolverModule, 'resolveTtsModName').mockResolvedValue('@hmcs/voicevox');
    const callSpy = vi.spyOn(rpc, 'call').mockResolvedValue(undefined);

    const { speakViaTts } = await import('./entry.js');
    await speakViaTts('alice', 'Hello, world!');

    expect(callSpy).toHaveBeenCalledTimes(1);
    const options = callSpy.mock.calls[0]?.[0];
    expect(options.modName).toBe('@hmcs/voicevox');
    expect(options.method).toBe('speak');
    const body = options.body as { personaId: string; text: string[] };
    expect(body.personaId).toBe('alice');
    expect(Array.isArray(body.text)).toBe(true);
    expect(body.text.length).toBeGreaterThan(0);
  });

  test('routes to a different MOD when the resolver changes', async () => {
    const resolverModule = await import('./tts-resolver.js');
    vi.spyOn(resolverModule, 'resolveTtsModName').mockResolvedValue('@hmcs/some-other-tts');
    const callSpy = vi.spyOn(rpc, 'call').mockResolvedValue(undefined);

    const { speakViaTts } = await import('./entry.js');
    await speakViaTts('alice', 'Hello');

    expect(callSpy.mock.calls[0]?.[0].modName).toBe('@hmcs/some-other-tts');
  });

  test('skips rpc.call when sanitizer yields zero sentences', async () => {
    const resolverModule = await import('./tts-resolver.js');
    vi.spyOn(resolverModule, 'resolveTtsModName').mockResolvedValue('@hmcs/voicevox');
    const callSpy = vi.spyOn(rpc, 'call').mockResolvedValue(undefined);

    const { speakViaTts } = await import('./entry.js');
    await speakViaTts('alice', '');

    expect(callSpy).not.toHaveBeenCalled();
  });
});
