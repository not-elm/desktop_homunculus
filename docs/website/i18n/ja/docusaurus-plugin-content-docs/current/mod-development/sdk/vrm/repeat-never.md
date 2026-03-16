---
title: "repeat.never"
sidebar_position: 25
---

# repeat.never

```typescript
import { repeat } from "@hmcs/sdk";
```

`repeat.never()` はアニメーションをちょうど1回再生してから停止する `VrmaRepeat` 値を返します。

```typescript
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
  waitForCompletion: true,
});
```

[`playVrma`](./playVrma) の `waitForCompletion: true` と組み合わせて、ワンショットアニメーションの完了をブロックすることができます。[`repeat.forever`](./repeat-forever) と [`repeat.count`](./repeat-count) も参照してください。
