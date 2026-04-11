import { describe, expect, it } from 'vitest';
import { MessageRouter } from './message-router.ts';

describe('MessageRouter', () => {
  it('delivers a message to the target persona subscriber', async () => {
    const router = new MessageRouter();
    const received: unknown[] = [];
    router.subscribe('bob', (msg) => received.push(msg));

    await router.send({ from: 'alice', to: 'bob', message: 'hello' });

    expect(received).toHaveLength(1);
    expect(received[0]).toMatchObject({ from: 'alice', to: 'bob', message: 'hello' });
  });

  it('rejects sending to an unknown persona', async () => {
    const router = new MessageRouter();
    await expect(
      router.send({ from: 'alice', to: 'ghost', message: 'hi' }),
    ).rejects.toThrow(/not.*subscribed/i);
  });

  it('unsubscribing stops delivery', async () => {
    const router = new MessageRouter();
    const received: unknown[] = [];
    const unsub = router.subscribe('bob', (msg) => received.push(msg));
    unsub();

    await expect(
      router.send({ from: 'alice', to: 'bob', message: 'hi' }),
    ).rejects.toThrow();
    expect(received).toHaveLength(0);
  });
});
