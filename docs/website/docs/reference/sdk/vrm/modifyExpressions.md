---
title: "modifyExpressions"
sidebar_position: 29
---

# modifyExpressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.modifyExpressions(weights)` performs a **partial update** -- only the specified expressions change. Existing overrides not mentioned in the call remain intact.

```typescript
// First, set happy
await character.modifyExpressions({ happy: 1.0 });

// Later, add a blink without removing the happy override
await character.modifyExpressions({ blink: 1.0 });
// Result: happy=1.0, blink=1.0
```

:::tip
Use [`setExpressions`](./setExpressions) when you want full control over which expressions are overridden. Use `modifyExpressions` when you want to layer changes without disturbing other overrides.
:::
