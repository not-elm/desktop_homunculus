/**
 * Signal pub/sub namespace for real-time event streaming.
 *
 * Provides multiplexed WebSocket-based signal streaming. Each JS runtime
 * (WebView or Node.js process) maintains a single WebSocket connection
 * to the server, with all channel subscriptions multiplexed over it.
 *
 * @example
 * ```typescript
 * import { signals } from "@hmcs/sdk";
 *
 * // Subscribe to a channel
 * const sub = signals.stream<{ state: string }>("openclaw:status", (data) => {
 *   console.log("State:", data.state);
 * });
 *
 * // Later, unsubscribe
 * sub.close();
 *
 * // Send a signal (uses HTTP POST, not WebSocket)
 * await signals.send("openclaw:status", { state: "idle" });
 * ```
 *
 * @packageDocumentation
 */

import { host } from './host';

/** Information about an active signal channel. */
export interface SignalChannelInfo {
  /** The signal channel name. */
  signal: string;
  /** The number of active subscribers. */
  subscribers: number;
}

/** A handle to an active signal subscription. Call `.close()` to unsubscribe. */
export interface Subscription {
  /** Unsubscribe from the channel and stop receiving events. */
  close(): void;
}

export namespace signals {
  // --- Connection state ---

  type Callback = (payload: unknown) => void | Promise<void>;

  let ws: WebSocket | null = null;
  let connectPromise: Promise<void> | null = null;
  const listeners = new Map<string, Set<Callback>>();
  const pendingSubscribes: string[] = [];
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  let reconnectDelay = 1000;
  const MAX_RECONNECT_DELAY = 5000;

  function wsUrl(): string {
    const base = host.base().replace(/^http/, 'ws');
    return `${base}/signals/ws`;
  }

  function ensureConnection(): void {
    if (ws && (ws.readyState === WebSocket.OPEN || ws.readyState === WebSocket.CONNECTING)) {
      return;
    }
    connect();
  }

  function connect(): void {
    if (connectPromise) return;

    const url = wsUrl();

    // Node.js: use `ws` package; Browser: use native WebSocket
    const WS =
      typeof globalThis.WebSocket !== 'undefined'
        ? globalThis.WebSocket
        : // eslint-disable-next-line @typescript-eslint/no-require-imports
          (require('ws') as typeof WebSocket);

    ws = new WS(url);

    connectPromise = new Promise<void>((resolve) => {
      ws?.addEventListener('open', () => {
        reconnectDelay = 1000;
        connectPromise = null;

        // Flush pending subscribes
        for (const ch of pendingSubscribes) {
          sendFrame({ type: 'subscribe', channel: ch });
        }
        pendingSubscribes.length = 0;

        // Re-subscribe all active channels
        for (const channel of listeners.keys()) {
          sendFrame({ type: 'subscribe', channel });
        }

        resolve();
      });
    });

    ws.addEventListener('message', (event: MessageEvent) => {
      try {
        const msg = JSON.parse(typeof event.data === 'string' ? event.data : event.data.toString());
        if (msg.channel && 'data' in msg) {
          dispatch(msg.channel, msg.data);
        }
      } catch (e) {
        console.error('signals: failed to parse WS message', e);
      }
    });

    ws.addEventListener('close', () => {
      ws = null;
      connectPromise = null;
      scheduleReconnect();
    });

    ws.addEventListener('error', () => {
      // Error is followed by close event, which handles reconnection
    });
  }

  function scheduleReconnect(): void {
    if (listeners.size === 0) return;
    if (reconnectTimer) return;

    const jitter = Math.random() * 500;
    reconnectTimer = setTimeout(() => {
      reconnectTimer = null;
      reconnectDelay = Math.min(reconnectDelay * 2, MAX_RECONNECT_DELAY);
      connect();
    }, reconnectDelay + jitter);
  }

  function sendFrame(msg: object): void {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(msg));
    }
  }

  function dispatch(channel: string, data: unknown): void {
    const cbs = listeners.get(channel);
    if (!cbs) return;
    for (const cb of cbs) {
      try {
        const result = cb(data);
        if (result instanceof Promise) {
          result.catch((e) => console.error(`Error processing signal ${channel}:`, e));
        }
      } catch (e) {
        console.error(`Error processing signal ${channel}:`, e);
      }
    }
  }

  // --- Public API ---

  /**
   * List all active signal channels.
   *
   * @returns Array of active signal channels with subscriber counts
   *
   * @example
   * ```typescript
   * const channels = await signals.list();
   * for (const ch of channels) {
   *   console.log(`${ch.signal}: ${ch.subscribers} subscribers`);
   * }
   * ```
   */
  export async function list(): Promise<SignalChannelInfo[]> {
    const response = await host.get(host.createUrl('signals'));
    return (await response.json()) as SignalChannelInfo[];
  }

  /**
   * Subscribe to a signal channel.
   *
   * Returns a {@link Subscription} handle. Call `.close()` to unsubscribe.
   * Internally multiplexed over a single WebSocket connection per JS runtime.
   *
   * @typeParam V - Expected payload type (documentation-level safety)
   * @param signal - Signal channel name to subscribe to
   * @param f - Callback invoked for each received event
   * @returns A subscription handle with `.close()` method
   *
   * @example
   * ```typescript
   * const sub = signals.stream<{ state: string }>("openclaw:status", (data) => {
   *   console.log("State:", data.state);
   * });
   *
   * // Later, unsubscribe
   * sub.close();
   * ```
   */
  export function stream<V>(signal: string, f: (payload: V) => void | Promise<void>): Subscription {
    const callback = f as Callback;
    let closed = false;

    // Add callback to listeners
    let cbs = listeners.get(signal);
    const isNewChannel = !cbs || cbs.size === 0;
    if (!cbs) {
      cbs = new Set();
      listeners.set(signal, cbs);
    }
    cbs.add(callback);

    // Subscribe on server if this is a new channel
    if (isNewChannel) {
      ensureConnection();
      if (ws && ws.readyState === WebSocket.OPEN) {
        sendFrame({ type: 'subscribe', channel: signal });
      } else {
        pendingSubscribes.push(signal);
      }
    }

    return {
      close() {
        if (closed) return;
        closed = true;

        const channelCbs = listeners.get(signal);
        if (channelCbs) {
          channelCbs.delete(callback);
          if (channelCbs.size === 0) {
            listeners.delete(signal);
            sendFrame({ type: 'unsubscribe', channel: signal });
          }
        }
      },
    };
  }

  /**
   * Send a signal to all subscribers.
   *
   * Uses HTTP POST (not WebSocket) for request-response semantics.
   *
   * @typeParam V - Payload type to send
   * @param signal - Signal channel name
   * @param payload - Data to broadcast to all subscribers
   *
   * @example
   * ```typescript
   * await signals.send("openclaw:status", { state: "thinking" });
   * ```
   */
  export async function send<V>(signal: string, payload: V): Promise<void> {
    await host.post(host.createUrl(`signals/${signal}`), payload);
  }
}
