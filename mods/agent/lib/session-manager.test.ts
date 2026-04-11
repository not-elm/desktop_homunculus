import { beforeEach, describe, expect, it } from 'vitest';
import type { AgentRuntime } from './runtime/agent-runtime.ts';
import { KeyboardHookService } from './keyboard-hook.ts';
import { MockAgentRuntime } from './runtime/mock-agent-runtime.ts';
import { SessionManager } from './session-manager.ts';
import { SessionPersistence } from './session-persistence.ts';
import { DEFAULT_WORKER_LIMIT } from './types.ts';

/** A runtime that never yields any events, keeping the worker in 'running' state. */
class HangingAgentRuntime implements AgentRuntime {
  async *execute(): ReturnType<AgentRuntime['execute']> {
    await new Promise(() => {}); // hang forever
  }
}

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

describe('SessionManager worker delegation', () => {
  let manager: SessionManager;
  beforeEach(() => {
    manager = createManager();
  });

  function makeRuntimeFactory(sessionId: string) {
    return () => new MockAgentRuntime([{ type: 'completed', sessionId }]);
  }

  it('spawns a Worker and returns a taskId', async () => {
    const { taskId } = await manager.delegateTask({
      personaId: 'alice',
      description: 'refactor login',
      worktreeName: null,
      createRuntime: makeRuntimeFactory('s1'),
    });

    expect(taskId).toMatch(/^task-/);
    const status = manager.getTaskStatus('alice', taskId);
    expect(status).toBeDefined();
    expect(status?.description).toBe('refactor login');
  });

  it('enforces the default Worker limit per persona', async () => {
    const makeHanging = () => new HangingAgentRuntime();
    for (let i = 0; i < DEFAULT_WORKER_LIMIT; i++) {
      await manager.delegateTask({
        personaId: 'alice',
        description: `task-${i}`,
        worktreeName: null,
        createRuntime: makeHanging,
      });
    }

    await expect(
      manager.delegateTask({
        personaId: 'alice',
        description: 'overflow',
        worktreeName: null,
        createRuntime: makeHanging,
      }),
    ).rejects.toThrow(/limit/i);
  });

  it('cancels a running Worker', async () => {
    const { taskId } = await manager.delegateTask({
      personaId: 'alice',
      description: 't',
      worktreeName: null,
      createRuntime: () => new HangingAgentRuntime(),
    });

    manager.cancelTask('alice', taskId);
    const status = manager.getTaskStatus('alice', taskId);
    expect(status?.status).toBe('cancelled');
  });

  it('returns undefined for unknown taskId', () => {
    expect(manager.getTaskStatus('alice', 'nope')).toBeUndefined();
  });
});
