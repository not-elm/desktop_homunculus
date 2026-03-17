---
title: "repeat.count"
sidebar_position: 26
---

# repeat.count

```typescript
import { repeat } from "@hmcs/sdk";
```

`repeat.count(n)` はアニメーションをちょうど `n` 回再生してから停止する `VrmaRepeat` 値を返します。

```typescript
await character.playVrma({
  asset: "my-mod:nod",
  repeat: repeat.count(3),
});
```

:::warning
`repeat.count(n)` には正の整数が必要です。0、負の数、非整数を渡すと `RangeError` がスローされます。
:::

[`repeat.forever`](./repeat-forever) と [`repeat.never`](./repeat-never) も参照してください。
