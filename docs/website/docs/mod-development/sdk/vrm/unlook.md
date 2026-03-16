---
title: "unlook"
sidebar_position: 37
---

# unlook

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.unlook()` turns off look-at behavior entirely. The character's eyes return to their default animation-driven state.

```typescript
await character.unlook();
```

Use [`lookAtCursor`](./lookAtCursor) to re-enable cursor tracking, or [`lookAtTarget`](./lookAtTarget) to track a specific entity.
