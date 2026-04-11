import { beforeEach, describe, expect, it } from 'vitest';
import { KeyboardHookService } from './keyboard-hook.ts';
import { MockAgentRuntime } from './runtime/mock-agent-runtime.ts';
import { SessionManager } from './session-manager.ts';
import { SessionPersistence } from './session-persistence.ts';

function createManager(): SessionManager {
  return new SessionManager(new SessionPersistence(), new KeyboardHookService());
}

describe('SessionManager', () => {
  let manager: SessionManager;

  beforeEach(() => {
    manager = createManager();
  });

  it('tracks an active Frontman per persona', () => {
    const runtime = new MockAgentRuntime([{ type: 'completed', sessionId: 'abc' }]);
    manager.startFrontman('alice', runtime);

    expect(manager.hasFrontman('alice')).toBe(true);
    expect(manager.hasFrontman('bob')).toBe(false);
  });

  it('rejects starting a second Frontman for the same persona', () => {
    const r1 = new MockAgentRuntime([{ type: 'completed', sessionId: 'a' }]);
    const r2 = new MockAgentRuntime([{ type: 'completed', sessionId: 'b' }]);
    manager.startFrontman('alice', r1);

    expect(() => manager.startFrontman('alice', r2)).toThrow(/already/i);
  });

  it('stops a Frontman and clears tracked state', async () => {
    const runtime = new MockAgentRuntime([{ type: 'completed', sessionId: 'abc' }]);
    manager.startFrontman('alice', runtime);

    await manager.stopPersonaSessions('alice');
    expect(manager.hasFrontman('alice')).toBe(false);
  });
});
