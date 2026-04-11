import { signals } from '@hmcs/sdk';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { MessageRouter } from './coordination/message-router.ts';
import { KeyboardHookService } from './keyboard-hook.ts';
import { MockAgentRuntime } from './runtime/mock-agent-runtime.ts';
import { SessionManager } from './session-manager.ts';
import { SessionPersistence } from './session-persistence.ts';

describe('multi-persona integration', () => {
  let sendSpy: ReturnType<typeof vi.spyOn>;
  let sent: { channel: string; payload: unknown }[];
  let router: MessageRouter;
  let manager: SessionManager;

  beforeEach(() => {
    sent = [];
    sendSpy = vi.spyOn(signals, 'send').mockImplementation(async (channel: string, payload: unknown) => {
      sent.push({ channel, payload });
    });
    router = new MessageRouter();
    manager = new SessionManager(new SessionPersistence(), new KeyboardHookService(), router);
  });

  afterEach(() => {
    sendSpy.mockRestore();
  });

  it('Worker delegation completes and emits events', async () => {
    const workerRuntime = new MockAgentRuntime([
      { type: 'tool_use', tool: 'bash', summary: '$ ls' },
      { type: 'completed', sessionId: 'w1' },
    ]);
    const { taskId } = await manager.delegateTask({
      personaId: 'alice',
      description: 'list files',
      worktreeName: null,
      createRuntime: () => workerRuntime,
    });

    await new Promise((r) => setTimeout(r, 50));

    const task = manager.getTaskStatus('alice', taskId);
    expect(task?.status).toBe('completed');

    const workerEvents = sent.filter((s) => s.channel === 'agent:worker-event');
    expect(workerEvents.length).toBeGreaterThanOrEqual(1);
  });

  it('parallel Workers operate independently', async () => {
    const t1 = await manager.delegateTask({
      personaId: 'alice',
      description: 'task 1',
      worktreeName: null,
      createRuntime: () => new MockAgentRuntime([{ type: 'completed', sessionId: 'a' }]),
    });
    const t2 = await manager.delegateTask({
      personaId: 'alice',
      description: 'task 2',
      worktreeName: null,
      createRuntime: () => new MockAgentRuntime([{ type: 'completed', sessionId: 'b' }]),
    });

    await new Promise((r) => setTimeout(r, 50));

    expect(manager.getTaskStatus('alice', t1.taskId)?.status).toBe('completed');
    expect(manager.getTaskStatus('alice', t2.taskId)?.status).toBe('completed');
  });

  it('cross-persona message is delivered to target Frontman', async () => {
    manager.startFrontman('bob', new MockAgentRuntime([]));

    await router.send({ from: 'alice', to: 'bob', message: 'hi bob' });

    const peerMsgs = sent.filter((s) => s.channel === 'agent:peer-message');
    expect(peerMsgs).toHaveLength(1);
    expect((peerMsgs[0].payload as { personaId: string }).personaId).toBe('bob');
  });
});
