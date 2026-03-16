---
sidebar_position: 100
---

# 型定義

## Transform

位置、回転、スケールを含む完全な 3D トランスフォームです。`entities.transform()` から返され、VRM やエンティティ API 全体で使用されます。

```typescript
interface Transform {
  /** ワールド空間での位置: [x, y, z]。Y が上。 */
  translation: [number, number, number];
  /** クォータニオンとしての回転: [x, y, z, w]。単位元は [0, 0, 0, 1]。 */
  rotation: [number, number, number, number];
  /** スケール係数: [x, y, z]。通常サイズは [1, 1, 1]。 */
  scale: [number, number, number];
}
```

単位元トランスフォームの例：

```typescript
const identity: Transform = {
  translation: [0, 0, 0],
  rotation: [0, 0, 0, 1],
  scale: [1, 1, 1],
};
```

## TransformArgs

更新操作用の `Transform` の部分バージョンです。含めたフィールドのみが変更され、残りは現在の値が維持されます。

```typescript
interface TransformArgs {
  translation?: Vec3;
  rotation?: Quat;
  scale?: Vec3;
}
```

```typescript
// 回転やスケールを変更せずにエンティティを上方向に移動
const args: TransformArgs = {
  translation: [0, 100, 0],
};
```

## Vec2

スクリーン座標、UI の位置、2D 数学用の 2 要素タプルです。

```typescript
type Vec2 = [number, number]; // [x, y]
```

## Vec3

3D の位置、方向、スケール値用の 3 要素タプルです。

```typescript
type Vec3 = [number, number, number]; // [x, y, z]
```

## Quat

回転を表す 4 要素タプルです。`[0, 0, 0, 1]` が単位元（回転なし）です。

```typescript
type Quat = [number, number, number, number]; // [x, y, z, w]
```

一般的な値：

| 回転 | Quat |
|---|---|
| 単位元（なし） | `[0, 0, 0, 1]` |
| Y 軸 90 度 | `[0, 0.7071, 0, 0.7071]` |
| Y 軸 180 度 | `[0, 1, 0, 0]` |

## Rect

最小角と最大角の点で定義される 2D 矩形です。

```typescript
interface Rect {
  min: Vec2;
  max: Vec2;
}
```

```typescript
const bounds: Rect = {
  min: [0, 0],
  max: [1920, 1080],
};
```
