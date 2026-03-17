---
title: "springBones"
sidebar_position: 21
---

# springBones

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.springBones()` はこのキャラクターのすべてのスプリングボーンチェーンを返します。スプリングボーンは髪、衣服、アクセサリーの物理シミュレーションを行います。

```typescript
const { chains } = await character.springBones();
for (const chain of chains) {
  console.log(`Chain ${chain.entity}: ${chain.joints.length} joints`);
  console.log(`  Stiffness: ${chain.props.stiffness}`);
  console.log(`  Drag: ${chain.props.dragForce}`);
}
```

[`SpringBoneChain`](./types#springbonechain) オブジェクトの配列を含む [`SpringBoneChainsResponse`](./types#springbonechainsresponse) を返します。各チェーンの `entity` ID を使用して、[`springBone`](./springBone) と [`setSpringBone`](./setSpringBone) で個々のチェーンを照会・変更できます。
