---
sidebar_position: 3
---

# toViewport

3D ワールド位置をスクリーン座標に投影します。キャラクターやシーンオブジェクトに対して相対的に HTML オーバーレイやエフェクトを配置する場合に便利です。

すべてのフィールドはオプションです -- いずれかを省略すると、その軸のワールド原点がデフォルトになります。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| `world` | `{ x?: number; y?: number; z?: number }`（オプション） | 変換する 3D ワールド座標。省略した場合は原点を使用 |

## Returns

`Promise<GlobalViewport>`

## Example

```typescript
const screenPos = await coordinates.toViewport({ x: 0, y: 1.5, z: 0 });
console.log("スクリーン位置:", screenPos); // [x, y]
```

```typescript
// y のみ指定（x と z はデフォルトで 0）
const pos = await coordinates.toViewport({ y: 2.0 });
```
