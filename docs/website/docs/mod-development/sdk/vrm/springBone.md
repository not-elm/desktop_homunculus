---
title: "springBone"
sidebar_position: 22
---

# springBone

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.springBone(chainId)` returns a single spring bone chain by its entity ID.

```typescript
const { chains } = await character.springBones();
const chain = await character.springBone(chains[0].entity);
console.log(`Stiffness: ${chain.props.stiffness}`);
console.log(`Drag: ${chain.props.dragForce}`);
```

Returns a [`SpringBoneChain`](./types#springbonechain) object. Use [`springBones`](./springBones) to list all chains and find the chain ID you want.
