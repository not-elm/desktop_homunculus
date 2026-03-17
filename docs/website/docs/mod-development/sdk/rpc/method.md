---
sidebar_position: 3
---

# method

Creates a typed RPC method definition. Supports optional Zod input validation.

## Signatures

**With input validation:**

```typescript
rpc.method<I, O>(def: {
  description?: string;
  timeout?: number;
  input: ZodType<I>;
  handler: (params: I) => Promise<O>;
}): RpcMethodDef<I, O>
```

**Without input:**

```typescript
rpc.method<O>(def: {
  description?: string;
  timeout?: number;
  handler: () => Promise<O>;
}): RpcMethodDef<unknown, O>
```

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `description` | `string` | No | Human-readable description (exposed in the RPC registry) |
| `timeout` | `number` | No | Timeout in milliseconds (default: 30000) |
| `input` | `ZodType<I>` | No | Zod schema for input validation |
| `handler` | `(params: I) => Promise<O>` or `() => Promise<O>` | Yes | Async function that processes the request. For the no-input overload, the handler takes no arguments. |

## Returns

[`RpcMethodDef<I, O>`](./types#rpcmethoddefi-o)

## Example

```typescript
import { rpc } from "@hmcs/sdk/rpc";
import { z } from "zod";

// With input validation
const speak = rpc.method({
  description: "Speak a message",
  timeout: 10000,
  input: z.object({ text: z.string() }),
  handler: async ({ text }) => {
    return { spoken: true };
  },
});

// Without input
const status = rpc.method({
  description: "Get current status",
  handler: async () => {
    return { ready: true };
  },
});
```
