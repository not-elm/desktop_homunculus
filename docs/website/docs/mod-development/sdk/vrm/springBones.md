---
title: "springBones"
sidebar_position: 21
---

# springBones

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.springBones()` returns all spring bone chains for this character. Spring bones simulate physics on hair, clothing, and accessories.

```typescript
const { chains } = await character.springBones();
for (const chain of chains) {
  console.log(`Chain ${chain.entity}: ${chain.joints.length} joints`);
  console.log(`  Stiffness: ${chain.props.stiffness}`);
  console.log(`  Drag: ${chain.props.dragForce}`);
}
```

Returns a [`SpringBoneChainsResponse`](./types#springbonechainsresponse) containing an array of [`SpringBoneChain`](./types#springbonechain) objects. Use the `entity` ID of each chain to query or modify individual chains with [`springBone`](./springBone) and [`setSpringBone`](./setSpringBone).
