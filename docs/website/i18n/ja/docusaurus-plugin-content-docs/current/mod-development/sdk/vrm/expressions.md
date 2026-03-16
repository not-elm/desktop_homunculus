---
title: "expressions"
sidebar_position: 27
---

# expressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.expressions()` はすべての表情の現在の状態（ウェイトとメタデータを含む）を返します。

```typescript
const { expressions } = await character.expressions();
for (const expr of expressions) {
  if (expr.weight > 0) {
    console.log(`${expr.name}: weight=${expr.weight}, binary=${expr.isBinary}`);
  }
}
```

各 [`ExpressionInfo`](./types) には以下が含まれます：
- `name` -- 表情名（例：`"happy"`、`"aa"`）
- `weight` -- 現在のウェイト値（0.0--1.0）
- `isBinary` -- 表情が 0 か 1 にスナップするかどうか（中間値なし）
- `overrideBlink` -- この表情がまばたきとどう相互作用するか（`"none"`、`"blend"`、または `"block"`）
- `overrideLookAt` -- この表情が視線追従とどう相互作用するか
- `overrideMouth` -- この表情が口の表情とどう相互作用するか

## 利用可能な表情

ほとんどのモデルで利用可能な標準 VRM 表情：

| カテゴリ | 表情 |
|---|---|
| **感情** | `happy`、`angry`、`sad`、`relaxed`、`surprised`、`neutral` |
| **口** | `aa`、`ih`、`ou`、`ee`、`oh` |
| **目** | `blink`、`blinkLeft`、`blinkRight` |
| **視線** | `lookUp`、`lookDown`、`lookLeft`、`lookRight` |

:::note
利用可能な表情は VRM モデルによって異なります。すべてのモデルがすべての表情を含んでいるわけではありません。特定のモデルがサポートする表情を確認するには `expressions()` を使用してください。
:::
