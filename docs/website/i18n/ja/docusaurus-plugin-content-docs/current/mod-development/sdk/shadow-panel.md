---
title: "Shadow Panel"
sidebar_position: 16.5
---

# Shadow Panel

`shadowPanel` モジュールは、シャドウオーバーレイパネルを制御します -- 雰囲気演出やフォーカス暗転に使用される、フルスクリーンの透明レイヤーです。

```typescript
import { shadowPanel } from "@hmcs/sdk";

// 背景を暗くする
await shadowPanel.setAlpha(0.7);

// 現在のアルファ値を読み取り
const current = await shadowPanel.alpha();

// オーバーレイを削除
await shadowPanel.setAlpha(0);
```

`alpha` の範囲は `0`（完全に透明 / 非表示）から `1`（完全に不透明）です。
