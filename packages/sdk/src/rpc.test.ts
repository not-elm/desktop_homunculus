import { afterEach, beforeEach, describe, expect, it, type Mock, vi } from 'vitest';
import { z } from 'zod';
import { HomunculusApiError } from './host';

// ---------------------------------------------------------------------------
// rpc.method() — validation logic (no server needed)
// ---------------------------------------------------------------------------

describe('rpc.method()', () => {
  it('returns a method def with handler that resolves for valid input', async () => {
    const { rpc } = await import('./rpc');
    const def = rpc.method({
      description: 'Double a number',
      timeout: 5000,
      input: z.object({ n: z.number() }),
      handler: async ({ n }) => ({ result: n * 2 }),
    });

    const result = await def.handler({ n: 7 });
    expect(result).toEqual({ result: 14 });
  });

  it('preserves description and timeout on the returned def', async () => {
    const { rpc } = await import('./rpc');
    const def = rpc.method({
      description: 'My method',
      timeout: 1234,
      input: z.object({ x: z.string() }),
      handler: async ({ x }) => x,
    });

    expect(def.description).toBe('My method');
    expect(def.timeout).toBe(1234);
  });

  it('returns undefined description and timeout when not provided', async () => {
    const { rpc } = await import('./rpc');
    const def = rpc.method({
      input: z.object({ x: z.number() }),
      handler: async ({ x }) => x,
    });

    expect(def.description).toBeUndefined();
    expect(def.timeout).toBeUndefined();
  });

  it('validates input with safeParse — returns success for valid data', async () => {
    const { rpc } = await import('./rpc');
    const def = rpc.method({
      input: z.object({ name: z.string(), count: z.number() }),
      handler: async (params) => params,
    });

    // biome-ignore lint/style/noNonNullAssertion: input is defined in this test
    const result = def.input!.safeParse({ name: 'alice', count: 3 });
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data).toEqual({ name: 'alice', count: 3 });
    }
  });

  it('validates input with safeParse — returns failure for invalid data', async () => {
    const { rpc } = await import('./rpc');
    const def = rpc.method({
      input: z.object({ name: z.string() }),
      handler: async (params) => params,
    });

    // biome-ignore lint/style/noNonNullAssertion: input is defined in this test
    const result = def.input!.safeParse({ name: 123 });
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.errors.length).toBeGreaterThan(0);
    }
  });

  it('supports method without input schema', async () => {
    const { rpc } = await import('./rpc');
    const def = rpc.method({
      description: 'Ping',
      handler: async () => ({ pong: true }),
    });

    expect(def.input).toBeUndefined();
    expect(def.description).toBe('Ping');
    const result = await def.handler(undefined as never);
    expect(result).toEqual({ pong: true });
  });

  it('handler is called and result returned for valid input', async () => {
    const { rpc } = await import('./rpc');
    const handlerFn = vi.fn(async ({ value }: { value: string }) => ({
      upper: value.toUpperCase(),
    }));

    const def = rpc.method({
      input: z.object({ value: z.string() }),
      handler: handlerFn,
    });

    // biome-ignore lint/style/noNonNullAssertion: input is defined in this test
    const parseResult = def.input!.safeParse({ value: 'hello' });
    expect(parseResult.success).toBe(true);
    if (parseResult.success) {
      const output = await def.handler(parseResult.data);
      expect(output).toEqual({ upper: 'HELLO' });
      expect(handlerFn).toHaveBeenCalledWith({ value: 'hello' });
    }
  });
});

// ---------------------------------------------------------------------------
// rpc.serve() — env var checks
// ---------------------------------------------------------------------------

describe('rpc.serve() — env var validation', () => {
  const originalEnv = { ...process.env };

  beforeEach(() => {
    vi.resetModules();
    // Restore original env before each test
    for (const key of Object.keys(process.env)) {
      if (!(key in originalEnv)) {
        delete process.env[key];
      }
    }
    Object.assign(process.env, originalEnv);
  });

  afterEach(() => {
    // Clean up env vars set during tests
    delete process.env.HMCS_RPC_PORT;
    delete process.env.HMCS_MOD_NAME;
    delete process.env.HMCS_PORT;
  });

  it('throws when HMCS_RPC_PORT is missing', async () => {
    delete process.env.HMCS_RPC_PORT;
    process.env.HMCS_MOD_NAME = 'test-mod';

    const { rpc } = await import('./rpc');
    await expect(rpc.serve({ methods: {} })).rejects.toThrow(
      'HMCS_RPC_PORT environment variable is required',
    );
  });

  it('throws when HMCS_MOD_NAME is missing', async () => {
    process.env.HMCS_RPC_PORT = '9999';
    delete process.env.HMCS_MOD_NAME;

    const { rpc } = await import('./rpc');
    await expect(rpc.serve({ methods: {} })).rejects.toThrow(
      'HMCS_MOD_NAME environment variable is required',
    );
  });

  it('throws when both HMCS_RPC_PORT and HMCS_MOD_NAME are missing', async () => {
    delete process.env.HMCS_RPC_PORT;
    delete process.env.HMCS_MOD_NAME;

    const { rpc } = await import('./rpc');
    // HMCS_RPC_PORT is checked first
    await expect(rpc.serve({ methods: {} })).rejects.toThrow(
      'HMCS_RPC_PORT environment variable is required',
    );
  });
});

// ---------------------------------------------------------------------------
// rpc.call() — browser-safe RPC client (via rpc-client.ts)
// ---------------------------------------------------------------------------

describe('rpc.call()', () => {
  let postMock: Mock;

  beforeEach(async () => {
    vi.resetModules();
    const { host } = await import('./host');
    postMock = vi.fn();
    vi.spyOn(host, 'post').mockImplementation(postMock);
    vi.spyOn(host, 'createUrl').mockImplementation(
      (path: string) => new URL(`http://localhost:3100/${path}`),
    );
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('sends POST to rpc/call with modName, method, and body', async () => {
    postMock.mockResolvedValue({
      json: () => Promise.resolve({ greeting: 'Hello!' }),
    });

    const { rpc } = await import('./rpc-client');
    const result = await rpc.call<{ greeting: string }>({
      modName: 'voicevox',
      method: 'speak',
      body: { text: 'Hello!' },
    });

    expect(result).toEqual({ greeting: 'Hello!' });
    expect(postMock).toHaveBeenCalledWith(new URL('http://localhost:3100/rpc/call'), {
      modName: 'voicevox',
      method: 'speak',
      body: { text: 'Hello!' },
    });
  });

  it('omits body field when body is undefined', async () => {
    postMock.mockResolvedValue({
      json: () => Promise.resolve({ running: true }),
    });

    const { rpc } = await import('./rpc-client');
    await rpc.call({ modName: 'voicevox', method: 'status' });

    expect(postMock).toHaveBeenCalledWith(new URL('http://localhost:3100/rpc/call'), {
      modName: 'voicevox',
      method: 'status',
    });
  });

  it('propagates HomunculusApiError from host.post', async () => {
    postMock.mockRejectedValue(new HomunculusApiError(503, '/rpc/call', 'MOD not registered'));

    const { rpc } = await import('./rpc-client');
    await expect(rpc.call({ modName: 'unknown', method: 'foo' })).rejects.toThrow(
      HomunculusApiError,
    );
  });

  it('is re-exported from rpc.ts (Node.js entry)', async () => {
    postMock.mockResolvedValue({
      json: () => Promise.resolve({ ok: true }),
    });

    const { rpc } = await import('./rpc');
    const result = await rpc.call({ modName: 'test', method: 'ping' });

    expect(result).toEqual({ ok: true });
  });
});
