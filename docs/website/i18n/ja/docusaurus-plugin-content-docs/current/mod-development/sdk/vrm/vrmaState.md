---
title: "vrmaState"
sidebar_position: 19
---

# vrmaState

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.vrmaState(asset)` は特定のアニメーションの再生状態を返します。

```typescript
const state = await character.vrmaState("vrma:idle-maid");
console.log(`Playing: ${state.playing}`);
console.log(`Speed: ${state.speed}x`);
console.log(`Elapsed: ${state.elapsedSecs}s`);
console.log(`Repeat: ${state.repeat}`);
```

`playing`、`speed`、`elapsedSecs`、`repeat` フィールドを持つ [`VrmaState`](./types) オブジェクトを返します。
