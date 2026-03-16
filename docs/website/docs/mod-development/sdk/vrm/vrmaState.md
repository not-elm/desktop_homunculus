---
title: "vrmaState"
sidebar_position: 19
---

# vrmaState

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.vrmaState(asset)` returns the playback state of a specific animation.

```typescript
const state = await character.vrmaState("vrma:idle-maid");
console.log(`Playing: ${state.playing}`);
console.log(`Speed: ${state.speed}x`);
console.log(`Elapsed: ${state.elapsedSecs}s`);
console.log(`Repeat: ${state.repeat}`);
```

Returns a [`VrmaState`](./types) object with `playing`, `speed`, `elapsedSecs`, and `repeat` fields.
