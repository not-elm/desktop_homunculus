---
title: "listVrma"
sidebar_position: 18
---

# listVrma

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.listVrma()` returns all VRMA animations currently attached to this character.

```typescript
const animations = await character.listVrma();
for (const anim of animations) {
  console.log(`${anim.name}: entity=${anim.entity}, playing=${anim.playing}`);
}
```

Returns an array of [`VrmaInfo`](./types) objects, each containing the animation's entity ID, name, and playing status.
