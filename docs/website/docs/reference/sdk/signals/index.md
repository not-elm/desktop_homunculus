---
sidebar_position: 1
---

# signals

Cross-process pub/sub messaging via Server-Sent Events (SSE). Signals let MOD services, MOD commands, and WebViews communicate in real time.

## Import

```typescript
import { signals } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [list](./list) | Returns all active signal channels with their subscriber counts |
| [stream](./stream) | Opens a persistent SSE connection and calls a callback on each message |
| [send](./send) | Broadcasts a JSON payload to all active subscribers on a channel |

See also: [Type Definitions](./types)
