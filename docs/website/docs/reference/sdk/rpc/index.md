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
| [serve](./serve)   | Start the RPC server and register methods with the engine |
| [method](./method) | Define an RPC method with optional Zod validation         |
| [call](./call)     | Call an RPC method on another MOD's service               |
