---
title: "repeat.forever"
sidebar_position: 24
---

# repeat.forever

```typescript
import { repeat } from "@hmcs/sdk";
```

`repeat.forever()` はアニメーションを無限にループする `VrmaRepeat` 値を返します。

```typescript
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

[`playVrma`](./playVrma) と一緒に使用してループアニメーションを作成します。ワンショットアニメーションには [`repeat.never`](./repeat-never)、固定回数のループには [`repeat.count`](./repeat-count) を参照してください。
