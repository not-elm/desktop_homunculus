---
sidebar_position: 1
---

# app

Application lifecycle, health checks, and platform information. Use `app` to verify the engine is running, query its version and features, or shut down the application.

## Import

```typescript
import { app } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [health](./health) | Returns `true` if the engine is reachable and healthy |
| [info](./info) | Returns metadata about the running engine instance |
| [exit](./exit) | Shuts down the Desktop Homunculus application gracefully |

See also: [Type Definitions](./types)
