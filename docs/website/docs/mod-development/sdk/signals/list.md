---
sidebar_position: 2
---

# list

`signals.list()` returns all active signal channels with their subscriber counts. Useful for debugging or checking if anyone is listening before sending.

## Parameters

None.

## Returns

`Promise<SignalChannelInfo[]>`

## Example

```typescript
import { signals } from "@hmcs/sdk";

const channels = await signals.list();
for (const ch of channels) {
  console.log(`${ch.signal}: ${ch.subscribers} subscribers`);
}
```
