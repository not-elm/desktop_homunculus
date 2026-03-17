---
sidebar_position: 2
---

# toWorld

スクリーンピクセル座標を 2D ワールド空間座標に変換します。3D シーン内の位置を表す `Vec2` を返します。

`x` と `y` はどちらもオプションです -- いずれかを省略すると、その軸にはスクリーンの中心が使用されます。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| `viewport` | `{ x?: number; y?: number }`（オプション） | 変換するスクリーン座標。省略した場合は中心を使用 |

## Returns

`Promise<Vec2>`

## Example

```typescript
const worldPos = await coordinates.toWorld({ x: 150, y: 200 });
console.log("ワールド位置:", worldPos); // [x, y]
```

```typescript
// x 座標のみ変換（y はデフォルトで中心）
const pos = await coordinates.toWorld({ x: 500 });
```
