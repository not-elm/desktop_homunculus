---
sidebar_position: 2
---

# sleep

Resolves after `ms` milliseconds (non-blocking delay).

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `ms` | `number` | Number of milliseconds to wait |

## Returns

`Promise<void>`

## Example

```typescript
import { sleep } from "@hmcs/sdk";

await sleep(1000); // wait 1 second
```
