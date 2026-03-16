---
title: "setSpringBone"
sidebar_position: 23
---

# setSpringBone

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setSpringBone(chainId, props)` updates the physics properties of a spring bone chain. All properties are optional -- only the specified fields are updated.

```typescript
const { chains } = await character.springBones();
const hairChain = chains[0];

// Make hair bouncier
await character.setSpringBone(hairChain.entity, {
  stiffness: 0.5,
  dragForce: 0.2,
});

// Change gravity direction
await character.setSpringBone(hairChain.entity, {
  gravityPower: 1.0,
  gravityDir: [0, -1, 0],
});
```

See [`SpringBoneProps`](./types) for all available properties.
