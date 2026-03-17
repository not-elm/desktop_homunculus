---
title: "lookAtCursor"
sidebar_position: 35
---

# lookAtCursor

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.lookAtCursor()` makes the character's eyes follow the mouse cursor across the screen.

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.lookAtCursor();
```

A typical pattern is to enable cursor tracking when the character is idle and disable it during drag or other interactions:

```typescript
character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    await character.unlook();
  }
});
```

Use [`unlook`](./unlook) to disable look-at behavior, or [`lookAtTarget`](./lookAtTarget) to track a specific entity instead.
