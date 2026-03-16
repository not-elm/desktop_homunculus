---
title: "springBone"
sidebar_position: 22
---

# springBone

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.springBone(chainId)` はエンティティ ID で指定した単一のスプリングボーンチェーンを返します。

```typescript
const { chains } = await character.springBones();
const chain = await character.springBone(chains[0].entity);
console.log(`Stiffness: ${chain.props.stiffness}`);
console.log(`Drag: ${chain.props.dragForce}`);
```

[`SpringBoneChain`](./types) オブジェクトを返します。すべてのチェーンを一覧表示してチェーン ID を取得するには [`springBones`](./springBones) を使用してください。
