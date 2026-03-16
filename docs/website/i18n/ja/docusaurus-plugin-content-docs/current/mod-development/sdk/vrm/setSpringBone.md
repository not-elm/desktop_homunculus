---
title: "setSpringBone"
sidebar_position: 23
---

# setSpringBone

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setSpringBone(chainId, props)` はスプリングボーンチェーンの物理プロパティを更新します。すべてのプロパティはオプションです -- 指定したフィールドのみが更新されます。

```typescript
const { chains } = await character.springBones();
const hairChain = chains[0];

// 髪をより弾力的に
await character.setSpringBone(hairChain.entity, {
  stiffness: 0.5,
  dragForce: 0.2,
});

// 重力方向を変更
await character.setSpringBone(hairChain.entity, {
  gravityPower: 1.0,
  gravityDir: [0, -1, 0],
});
```

利用可能なすべてのプロパティについては [`SpringBoneProps`](./types) を参照してください。
