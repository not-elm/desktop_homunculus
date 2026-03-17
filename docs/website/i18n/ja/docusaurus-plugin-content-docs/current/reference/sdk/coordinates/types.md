---
sidebar_position: 100
---

# 型定義

### GlobalViewport

```typescript
type GlobalViewport = [number, number];
```

スクリーン空間座標を `[x, y]` で表します。

### GlobalDisplay

```typescript
interface GlobalDisplay {
  /** 一意なディスプレイ識別子。 */
  id: number;
  /** 人間が読めるディスプレイ名。 */
  title: string;
  /** スクリーン座標でのディスプレイフレーム矩形。 */
  frame: Rect;
}
```

`Rect` の定義は [Math Types](../math) を参照してください。

### World2d

```typescript
type World2d = Vec2; // [number, number]
```

2D ワールド座標のエイリアス。

### World3d

```typescript
type World3d = Vec3; // [number, number, number]
```

3D ワールド座標のエイリアス。

`Vec2`、`Vec3` については [Math Types](../math) を参照してください。
