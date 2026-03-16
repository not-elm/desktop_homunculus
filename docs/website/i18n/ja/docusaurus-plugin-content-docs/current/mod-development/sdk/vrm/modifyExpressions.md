---
title: "modifyExpressions"
sidebar_position: 29
---

# modifyExpressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.modifyExpressions(weights)` は**部分的な更新**を行います -- 指定された表情のみが変更されます。呼び出しに記載されていない既存のオーバーライドはそのまま維持されます。

```typescript
// まず happy を設定
await character.modifyExpressions({ happy: 1.0 });

// 次に happy のオーバーライドを除去せずに blink を追加
await character.modifyExpressions({ blink: 1.0 });
// 結果：happy=1.0、blink=1.0
```

:::tip
どの表情をオーバーライドするか完全に制御したい場合は [`setExpressions`](./setExpressions) を使用してください。他のオーバーライドを変更せずに変更をレイヤリングしたい場合は `modifyExpressions` を使用してください。
:::
