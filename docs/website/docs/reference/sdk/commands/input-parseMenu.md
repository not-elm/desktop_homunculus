---
sidebar_position: 3
---

# input.parseMenu

Parse menu command stdin and return a `Vrm` instance for the linked character. Menu commands receive `{ "linkedVrm": <entityId> }` on stdin from the menu UI.

## Parameters

None.

## Returns

`Promise<Vrm>`

## Example

```typescript
import { input } from "@hmcs/sdk/commands";

const vrm = await input.parseMenu();
await vrm.setExpressions({ happy: 1.0 });
```
