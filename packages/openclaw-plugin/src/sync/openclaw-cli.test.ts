import { beforeEach, describe, expect, test, vi } from 'vitest';
import type { OpenClawConfig } from 'openclaw/plugin-sdk/plugin-entry';
import { createOpenClawCli, type OpenClawAgentSource } from './openclaw-cli.js';

function makeLogger() {
  return { debug: vi.fn(), info: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

// Tests exercise partial OpenClawConfig shapes; cast through `unknown` to
// avoid reconstructing the full 100+ field type.
function asConfig(value: unknown): OpenClawConfig {
  return value as OpenClawConfig;
}

function makeSource(
  agents: Array<{ id: string; workspace?: string }>,
): OpenClawAgentSource {
  return {
    config: asConfig({ agents: { list: agents } }),
    resolveAgentWorkspaceDir: vi.fn((_cfg, id) => `/ws/${id}`),
  };
}

describe('openclaw-cli', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test('agentsList returns entries resolved via SDK resolvers', async () => {
    const logger = makeLogger();
    const source = makeSource([
      { id: 'alice' },
      { id: 'bob', workspace: '/custom/bob' },
    ]);
    const cli = createOpenClawCli(logger, source);
    const result = await cli.agentsList();

    expect(result).toHaveLength(2);
    expect(result[0]).toEqual({ id: 'alice', workspace: '/ws/alice' });
    expect(result[1]).toEqual({ id: 'bob', workspace: '/ws/bob' });
    expect(source.resolveAgentWorkspaceDir).toHaveBeenCalledWith(source.config, 'alice');
    expect(source.resolveAgentWorkspaceDir).toHaveBeenCalledWith(source.config, 'bob');
  });

  test('agentsList returns [] and warns once when agents.list is empty', async () => {
    const logger = makeLogger();
    const source = makeSource([]);
    const cli = createOpenClawCli(logger, source);

    const r1 = await cli.agentsList();
    const r2 = await cli.agentsList();

    expect(r1).toEqual([]);
    expect(r2).toEqual([]);
    expect(logger.warn).toHaveBeenCalledTimes(1);
    expect(logger.warn).toHaveBeenCalledWith(expect.stringContaining('no agents.list entries'));
  });

  test('agentsList returns [] when config is missing or malformed', async () => {
    const logger = makeLogger();
    const source: OpenClawAgentSource = {
      config: asConfig(null),
      resolveAgentWorkspaceDir: vi.fn(() => '/never'),
    };
    const cli = createOpenClawCli(logger, source);
    const result = await cli.agentsList();
    expect(result).toEqual([]);
  });

  test('agentsList filters out entries with missing/empty/non-string id', async () => {
    const logger = makeLogger();
    const source: OpenClawAgentSource = {
      config: asConfig({
        agents: { list: [{ id: 'alice' }, { id: '' }, { id: '   ' }, { id: 42 }, {}, null] },
      }),
      resolveAgentWorkspaceDir: vi.fn((_cfg, id) => `/ws/${id}`),
    };
    const cli = createOpenClawCli(logger, source);
    const result = await cli.agentsList();
    expect(result).toHaveLength(1);
    expect(result[0]!.id).toBe('alice');
  });

  test('agentsList dedupes duplicate ids', async () => {
    const logger = makeLogger();
    const source = makeSource([{ id: 'alice' }, { id: 'alice' }, { id: 'bob' }]);
    const cli = createOpenClawCli(logger, source);
    const result = await cli.agentsList();
    expect(result.map((e) => e.id)).toEqual(['alice', 'bob']);
  });

  test('agentsList singleflights concurrent calls', async () => {
    const logger = makeLogger();
    const source = makeSource([{ id: 'alice' }]);
    const cli = createOpenClawCli(logger, source);
    const [a, b] = await Promise.all([cli.agentsList(), cli.agentsList()]);
    expect(a).toEqual(b);
    // resolver called once for the single agent (shared readOnce)
    const mock = vi.mocked(source.resolveAgentWorkspaceDir);
    expect(mock.mock.calls.length).toBe(1);
  });

  test('agentsList returns [] and warns when resolver throws', async () => {
    const logger = makeLogger();
    const source: OpenClawAgentSource = {
      config: asConfig({ agents: { list: [{ id: 'alice' }] } }),
      resolveAgentWorkspaceDir: vi.fn(() => {
        throw new Error('resolver bang');
      }),
    };
    const cli = createOpenClawCli(logger, source);
    const result = await cli.agentsList();
    expect(result).toEqual([]);
    expect(logger.warn).toHaveBeenCalledWith(
      expect.stringContaining('Failed to read OpenClaw agents'),
    );
    expect(logger.warn).toHaveBeenCalledWith(expect.stringContaining('resolver bang'));
  });
});
