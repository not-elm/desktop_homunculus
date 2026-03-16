---
sidebar_position: 1
---

# math

SDK 全体で使用される 3D 数学の型定義です。これらは純粋な TypeScript の型で、ランタイムメソッドはありません -- エンジンとやり取りするデータの形状を定義します。すべての型は Bevy の数学シリアライゼーション形式と互換性があります。

## インポート

```typescript
import {
  type Transform,
  type TransformArgs,
  type Vec2,
  type Vec3,
  type Quat,
  type Rect,
} from "@hmcs/sdk";
```

関連: [型定義](./types)
