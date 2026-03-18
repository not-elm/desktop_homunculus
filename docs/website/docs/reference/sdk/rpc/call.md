---
sidebar_position: 4
---

# call

Call an RPC method on a MOD service via the engine's HTTP API. Works in both browser (WebView) and Node.js environments.

## Signature

```typescript
rpc.call<T = unknown>(options: RpcCallOptions): Promise<T>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options.modName` | `string` | Name of the target MOD |
| `options.method` | `string` | RPC method name to invoke |
| `options.body` | `unknown` | Optional request body passed to the method handler |

## Returns

`Promise<T>` — The parsed JSON response from the MOD method handler.

## Error Responses

The engine proxy may return the following errors:

| Status | Condition |
|--------|-----------|
| 404 | Method not found on the target MOD |
| 502 | MOD service unreachable |
| 503 | MOD not registered (service not running) |
| 504 | Request timed out |

## Example

```typescript
import { rpc } from "@hmcs/sdk/rpc";

// Call with a request body
const result = await rpc.call<{ greeting: string }>({
  modName: "voicevox",
  method: "speak",
  body: { text: "Hello!" },
});
console.log(result.greeting);

// Call without a body
const status = await rpc.call<{ running: boolean }>({
  modName: "voicevox",
  method: "status",
});
```
