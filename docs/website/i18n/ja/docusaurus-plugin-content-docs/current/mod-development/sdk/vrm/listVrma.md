---
title: "listVrma"
sidebar_position: 18
---

# listVrma

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.listVrma()` はこのキャラクターに現在アタッチされているすべての VRMA アニメーションを返します。

```typescript
const animations = await character.listVrma();
for (const anim of animations) {
  console.log(`${anim.name}: entity=${anim.entity}, playing=${anim.playing}`);
}
```

アニメーションのエンティティ ID、名前、再生状態を含む [`VrmaInfo`](./types) オブジェクトの配列を返します。
