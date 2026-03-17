---
sidebar_position: 1
---

# coordinates

スクリーン空間（ビューポート）と 3D ワールド空間の座標変換を行います。エフェクトの配置、キャラクターに対する相対的な WebView の位置決め、マウス/タッチ入力からワールド位置へのマッピングに不可欠です。

## インポート

```typescript
import { coordinates } from "@hmcs/sdk";
```

## Functions

| Function | 説明 |
|----------|-------------|
| [toWorld](./toWorld) | スクリーンピクセル座標を 2D ワールド空間座標に変換する |
| [toViewport](./toViewport) | 3D ワールド位置をスクリーン座標に投影する |

See also: [型定義](./types)
