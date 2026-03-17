---
sidebar_position: 2
---

# serve

Starts an HTTP server on the engine-allocated port, registers methods with the engine, and handles graceful shutdown.

## Signature

```typescript
rpc.serve(options: RpcServeOptions): Promise<RpcServer>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options.methods` | `Record<string, `[`RpcMethodEntry`](./types#rpcmethodentry)`>` | Map of method names to definitions or handler functions |

## Returns

[`RpcServer`](./types#rpcserver)

## Behavior

1. Reads `HMCS_RPC_PORT`, `HMCS_MOD_NAME`, and `HMCS_PORT` from environment variables (set by the engine when spawning the MOD service).
2. Creates an HTTP server listening on `127.0.0.1:{HMCS_RPC_PORT}`.
3. Registers methods with the engine via `POST /rpc/register` with exponential backoff (up to 10 retries, 100 ms → 5 s).
4. Installs a `SIGTERM` handler for graceful shutdown.

## Error Responses

When a request arrives at the MOD's RPC server, it may return:

| Status | Condition |
|--------|-----------|
| 400 | Invalid JSON or Zod validation failure |
| 404 | Unknown method name |
| 405 | Non-POST request |
| 500 | Handler threw an error |

## Example

```typescript
import { rpc } from "@hmcs/sdk/rpc";
import { z } from "zod";

const server = await rpc.serve({
  methods: {
    greet: rpc.method({
      input: z.object({ name: z.string() }),
      handler: async ({ name }) => ({ message: `Hello, ${name}!` }),
    }),
    ping: async () => ({ pong: true }),
  },
});
```
