---
title: "findBoneEntity"
sidebar_position: 15
---

# findBoneEntity

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.findBoneEntity(bone)` は名前付きボーンの Bevy エンティティ ID を返します。オブジェクトを特定のボーンにアタッチしたり、視線追従ターゲットに設定したりするために使用します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const headEntity = await character.findBoneEntity("head");
const leftHandEntity = await character.findBoneEntity("leftHand");
```

利用可能なボーン名：`hips`、`spine`、`chest`、`neck`、`head`、`leftShoulder`、`leftArm`、`leftForeArm`、`leftHand`、`rightShoulder`、`rightArm`、`rightForeArm`、`rightHand`、`leftUpLeg`、`leftLeg`、`leftFoot`、`rightUpLeg`、`rightLeg`、`rightFoot`。

視線追従ターゲットとしてボーンエンティティを使用する例は [`lookAtTarget`](./lookAtTarget) を参照してください。
