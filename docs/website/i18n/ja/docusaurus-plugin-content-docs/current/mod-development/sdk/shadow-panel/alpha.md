---
sidebar_position: 2
---

# alpha

シャドウパネルの現在の透明度を取得します。

## パラメータ

なし。

## 戻り値

`Promise<number>` -- 現在のアルファ値（0--1）。

## 例

```typescript
import { shadowPanel } from "@hmcs/sdk";

const current = await shadowPanel.alpha();
console.log(current); // 例: 0.7
```
