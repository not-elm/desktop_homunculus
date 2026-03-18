---
sidebar_position: 1
---

# rpc

Define and serve RPC methods from a MOD service. Other MODs, the engine, and AI agents can call these methods through the engine's RPC proxy.

:::note
This module uses Node.js APIs (`node:http`, `process`) and is not browser-compatible. It is intentionally not re-exported from the main `@hmcs/sdk` entry point.
:::

## Import

```typescript
import { rpc } from "@hmcs/sdk/rpc";
```

## Functions

| Function           | Description                                               |
| ------------------ | --------------------------------------------------------- |
| [serve](/reference/sdk/rpc/serve)   | Start the RPC server and register methods with the engine |
| [method](/reference/sdk/rpc/method) | Define an RPC method with optional Zod validation         |
| [call](/reference/sdk/rpc/call)     | Call an RPC method on another MOD's service               |

## Type Definitions

| Type | Description |
|------|-------------|
| [RpcServer](/reference/sdk/rpc/types#rpcserver) | Server instance returned by `serve()` |
| [RpcMethodEntry](/reference/sdk/rpc/types#rpcmethodentry) | Method definition used in `serve({ methods })` |
| [RpcCallOptions](/reference/sdk/rpc/types#rpccalloptions) | Options for `call()` |
