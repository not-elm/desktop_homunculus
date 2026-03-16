---
sidebar_position: 11
---

# postStream

Sends a POST request and returns an async generator that yields parsed NDJSON objects as they arrive. Throws [`HomunculusApiError`](./types#homunculusapierror) on non-OK responses, and [`HomunculusStreamError`](./types#homunculusstreamerror) if an NDJSON line cannot be parsed.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `URL` | The URL to send the POST request to |
| `body` | `unknown` (optional) | Request body that will be JSON-serialized |
| `signal` | `AbortSignal` (optional) | Signal for cancellation |

## Returns

`AsyncGenerator<T>`

## Example

```typescript
import { host, type HomunculusStreamError } from "@hmcs/sdk";

const stream = host.postStream<{ type: string; data: string }>(
  host.createUrl("commands/execute"),
  { command: "build" },
);

for await (const event of stream) {
  console.log(event);
}
```
