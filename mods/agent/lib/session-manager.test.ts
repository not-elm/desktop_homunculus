import { signals } from '@hmcs/sdk';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { MessageRouter } from './coordination/message-router.ts';
import { KeyboardHookService } from './keyboard-hook.ts';
import type { AgentRuntime } from './runtime/agent-runtime.ts';
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

describe('SessionManager worker-event emission', () => {
  let manager: SessionManager;
  let sendSpy: ReturnType<typeof vi.spyOn>;
  const sent: { channel: string; payload: unknown }[] = [];

  beforeEach(() => {
    sent.length = 0;
    manager = createManager();
    sendSpy = vi.spyOn(signals, 'send').mockImplementation(async (channel: string, payload: unknown) => {
      sent.push({ channel, payload });
    });
  });

  afterEach(() => {
    sendSpy.mockRestore();
  });

  it('emits agent:worker-event for each Worker AgentEvent', async () => {
    const runtime = new MockAgentRuntime([
      { type: 'assistant_message', text: 'working' },
      { type: 'tool_use', tool: 'bash', summary: '$ ls' },
      { type: 'completed', sessionId: 's1' },
    ]);

    const { taskId } = await manager.delegateTask({
      personaId: 'alice',
      description: 'test task',
      worktreeName: null,
      createRuntime: () => runtime,
    });

    // Wait for the async Worker loop to finish
    await new Promise((r) => setTimeout(r, 50));

    const workerEvents = sent.filter((s) => s.channel === 'agent:worker-event');
    expect(workerEvents.length).toBeGreaterThanOrEqual(3);
    expect(workerEvents[0].payload).toMatchObject({
      personaId: 'alice',
      taskId,
      event: { type: 'assistant_message' },
    });
  });
});

describe('Worker permission routing', () => {
  let manager: SessionManager;
  let sendSpy: ReturnType<typeof vi.spyOn>;
  const sent: { channel: string; payload: unknown }[] = [];

  beforeEach(() => {
    sent.length = 0;
    manager = createManager();
    sendSpy = vi.spyOn(signals, 'send').mockImplementation(async (channel: string, payload: unknown) => {
      sent.push({ channel, payload });
    });
  });

  afterEach(() => {
    sendSpy.mockRestore();
  });

  it('Worker permission_request emits agent:permission with taskId and resolves via resolveWorkerPermission', async () => {
    const runtime = new MockAgentRuntime([
      { type: 'permission_request', requestId: 'req-1', tool: 'bash', input: { command: 'rm -rf /' } },
      { type: 'completed', sessionId: 's1' },
    ]);

    const { taskId } = await manager.delegateTask({
      personaId: 'alice',
      description: 'needs permission',
      worktreeName: null,
      createRuntime: () => runtime,
    });

    // Wait for permission signal
    await new Promise((r) => setTimeout(r, 50));

    const permEvents = sent.filter((s) => s.channel === 'agent:permission');
    expect(permEvents).toHaveLength(1);
    expect(permEvents[0].payload).toMatchObject({
      personaId: 'alice',
      taskId,
      requestId: 'req-1',
    });

    // Resolve the permission
    const resolved = manager.resolveWorkerPermission('req-1', { type: 'permission', approved: true });
    expect(resolved).toBe(true);

    // Wait for Worker to complete
    await new Promise((r) => setTimeout(r, 50));
    expect(manager.getTaskStatus('alice', taskId)?.status).toBe('completed');
  });

  it('resolveWorkerPermission returns false for unknown requestId', () => {
    const resolved = manager.resolveWorkerPermission('unknown', { type: 'permission', approved: true });
    expect(resolved).toBe(false);
  });
});

describe('permission timeout behavior', () => {
  let manager: SessionManager;
  let sendSpy: ReturnType<typeof vi.spyOn>;
  const sent: { channel: string; payload: unknown }[] = [];

  beforeEach(() => {
    sent.length = 0;
    manager = createManager();
    sendSpy = vi.spyOn(signals, 'send').mockImplementation(async (channel: string, payload: unknown) => {
      sent.push({ channel, payload });
    });
  });

  afterEach(() => {
    sendSpy.mockRestore();
  });

  it('does not auto-decline permission requests within 60s', async () => {
    // This test verifies the 60s auto-decline is gone.
    // We can't actually wait 60s in a test, but we verify that after a short
    // delay the permission is still pending (not auto-declined).

    // Use a runtime that emits a permission_request then waits
    const runtime = new MockAgentRuntime([
      { type: 'permission_request', requestId: 'r1', tool: 'bash', input: {} },
      { type: 'completed', sessionId: 's1' },
    ]);

    const { taskId } = await manager.delegateTask({
      personaId: 'alice',
      description: 'test permission',
      worktreeName: null,
      createRuntime: () => runtime,
    });

    // Wait a bit — old code would have set a 60s timer
    await new Promise((r) => setTimeout(r, 200));

    // Worker should still be running (blocked on permission)
    const task = manager.getTaskStatus('alice', taskId);
    expect(task?.status).toBe('running');

    // Now resolve it manually
    manager.resolveWorkerPermission('r1', { type: 'permission', approved: true });
    await new Promise((r) => setTimeout(r, 50));

    expect(manager.getTaskStatus('alice', taskId)?.status).toBe('completed');
  });
});

describe('peer message delivery', () => {
  let manager: SessionManager;
  let messageRouter: MessageRouter;
  let sendSpy: ReturnType<typeof vi.spyOn>;
  const sent: { channel: string; payload: unknown }[] = [];

  beforeEach(() => {
    sent.length = 0;
    messageRouter = new MessageRouter();
    manager = new SessionManager(new SessionPersistence(), new KeyboardHookService(), messageRouter);
    sendSpy = vi.spyOn(signals, 'send').mockImplementation(async (channel: string, payload: unknown) => {
      sent.push({ channel, payload });
    });
  });

  afterEach(() => {
    sendSpy.mockRestore();
  });

  it('delivered PeerMessage results in agent:peer-message signal', async () => {
    // Start bob's Frontman so he's subscribed to the router
    const bobRuntime = new MockAgentRuntime([]);
    manager.startFrontman('bob', bobRuntime);

    // Send a message from alice to bob via the router
    await messageRouter.send({ from: 'alice', to: 'bob', message: 'hi bob' });

    // Allow async delivery
    await new Promise((r) => setTimeout(r, 10));

    const peerMsgs = sent.filter((m) => m.channel === 'agent:peer-message');
    expect(peerMsgs).toHaveLength(1);
    expect(peerMsgs[0].payload).toMatchObject({
      personaId: 'bob',
    });
  });
});
