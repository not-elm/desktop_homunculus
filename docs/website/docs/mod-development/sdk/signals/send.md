---
sidebar_position: 4
---

# send

`signals.send<V>(signal, payload)` broadcasts a JSON payload to all active subscribers on that channel.

If no subscribers are listening, the message is silently dropped.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `signal` | `string` | The signal channel name to broadcast to |
| `payload` | `V` | The data to send to all subscribers |

## Returns

`Promise<void>`

## Example

```typescript
import { signals } from "@hmcs/sdk";

await signals.send("my-mod:chat", {
  sender: "bot",
  text: "Hello from the server!",
});
```
