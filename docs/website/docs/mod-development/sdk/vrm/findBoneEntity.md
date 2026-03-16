---
title: "findBoneEntity"
sidebar_position: 15
---

# findBoneEntity

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.findBoneEntity(bone)` returns the Bevy entity ID of the named bone. Use this to attach objects to a specific bone or to set look-at targets.

```typescript
const character = await Vrm.findByName("MyAvatar");
const headEntity = await character.findBoneEntity("head");
const leftHandEntity = await character.findBoneEntity("leftHand");
```

Available bone names: `hips`, `spine`, `chest`, `neck`, `head`, `leftShoulder`, `leftArm`, `leftForeArm`, `leftHand`, `rightShoulder`, `rightArm`, `rightForeArm`, `rightHand`, `leftUpLeg`, `leftLeg`, `leftFoot`, `rightUpLeg`, `rightLeg`, `rightFoot`.

The `Bones` type is the union of all valid bone name strings. See [`lookAtTarget`](./lookAtTarget) for an example using bone entities as look-at targets.
