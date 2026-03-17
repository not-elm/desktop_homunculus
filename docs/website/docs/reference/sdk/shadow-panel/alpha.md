---
sidebar_position: 2
---

# alpha

Gets the current transparency level of the shadow panel.

## Parameters

None.

## Returns

`Promise<number>` -- the current alpha value (0--1).

## Example

```typescript
import { shadowPanel } from "@hmcs/sdk";

const current = await shadowPanel.alpha();
console.log(current); // e.g. 0.7
```
