---
sidebar_position: 3
---

# stream

`signals.stream<V>(signal, callback)` opens a persistent SSE connection and calls `callback` each time a message arrives on the given channel. Returns an `EventSource` instance you must close when done.

The callback can be `async` -- errors inside the callback are caught and logged to the console.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `signal` | `string` | The signal channel name to subscribe to |
| `callback` | `(payload: V) => void \| Promise<void>` | Callback function to handle received payloads |

## Returns

`EventSource` -- the SSE connection instance. Call `.close()` to unsubscribe.

## Example

```typescript
import { signals } from "@hmcs/sdk";

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

The callback can also be `async`:

```typescript
const es = signals.stream<{ url: string }>("my-mod:fetch", async (payload) => {
  const res = await fetch(payload.url);
  console.log(await res.text());
});
```
