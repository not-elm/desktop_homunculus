---
title: "Signals"
sidebar_position: 9
---

# Signals

Cross-process pub/sub messaging via Server-Sent Events (SSE). Signals let MOD services, bin commands, and WebViews communicate in real time.

## Import

```typescript
import { signals } from "@hmcs/sdk";
```

## Subscribing

`signals.stream<V>(signal, callback)` opens a persistent SSE connection and calls `callback` each time a message arrives on the given channel. Returns an `EventSource` instance you must close when done.

```typescript
interface ChatMessage {
  sender: string;
  text: string;
}

const es = signals.stream<ChatMessage>("my-mod:chat", (msg) => {
  console.log(`${msg.sender}: ${msg.text}`);
});

// Close when no longer needed
es.close();
```

The callback can be `async` -- errors inside the callback are caught and logged to the console.

```typescript
const es = signals.stream<{ url: string }>("my-mod:fetch", async (payload) => {
  const res = await fetch(payload.url);
  console.log(await res.text());
});
```

## Publishing

`signals.send<V>(signal, payload)` broadcasts a JSON payload to all active subscribers on that channel.

```typescript
await signals.send("my-mod:chat", {
  sender: "bot",
  text: "Hello from the server!",
});
```

If no subscribers are listening, the message is silently dropped.

## Listing Channels

`signals.list()` returns all active signal channels with their subscriber counts. Useful for debugging or checking if anyone is listening before sending.

```typescript
const channels = await signals.list();
for (const ch of channels) {
  console.log(`${ch.signal}: ${ch.subscribers} subscribers`);
}
```

## Example: Real-Time Sync

A common pattern is using signals to synchronize a MOD's service with its WebView UI.

**Service** (runs in Node.js):

```typescript
import { signals, Vrm } from "@hmcs/sdk";

// Listen for commands from the WebView
signals.stream<{ action: string }>("my-mod:ui-cmd", async (cmd) => {
  if (cmd.action === "wave") {
    const vrm = await Vrm.findByName("MyAvatar");
    await vrm.playVrma({ asset: "my-mod:wave" });
  }
});
```

**WebView code** (runs in the browser):

```typescript
import { signals } from "@hmcs/sdk";

// Send a command to the service
document.getElementById("wave-btn")?.addEventListener("click", () => {
  signals.send("my-mod:ui-cmd", { action: "wave" });
});
```

## Types

### `SignalChannelInfo`

Returned by `signals.list()`.

| Field | Type | Description |
|-------|------|-------------|
| `signal` | `string` | The signal channel name |
| `subscribers` | `number` | Number of active subscribers |

## Next Steps

- **[WebViews](./webviews)** -- Create and control embedded HTML interfaces
- **[Audio](./audio)** -- Sound effects and background music
