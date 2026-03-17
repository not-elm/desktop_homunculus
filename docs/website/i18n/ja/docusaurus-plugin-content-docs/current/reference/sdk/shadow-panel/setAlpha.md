---
sidebar_position: 3
---

# setAlpha

シャドウパネルの透明度を設定します。

`alpha` の範囲は `0`（完全に透明 / 非表示）から `1`（完全に不透明）です。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `alpha` | `number` | 0（非表示）から 1（不透明）の透明度の値 |

## 戻り値

`Promise<void>`

## 例

```typescript
import { shadowPanel } from "@hmcs/sdk";

// 背景を暗くする
await shadowPanel.setAlpha(0.7);

// オーバーレイを削除
await shadowPanel.setAlpha(0);
```
