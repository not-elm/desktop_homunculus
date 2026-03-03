---
title: "Coordinates"
sidebar_position: 10
---

# Coordinates

スクリーン空間（ビューポート）と 3D ワールド空間の座標変換を行います。エフェクトの配置、キャラクターに対する相対的な WebView の位置決め、マウス/タッチ入力からワールド位置へのマッピングに不可欠です。

## インポート

```typescript
import { coordinates } from "@hmcs/sdk";
```

## 座標系

Desktop Homunculus は 2 つの主要な座標系を使用します：

| 座標系 | 説明 | 例 |
|---|---|---|
| **グローバルビューポート** | デスクトップ全体に対するスクリーンピクセル | マウス位置、UI 要素の配置 |
| **ワールド** | 3D シーン座標（Y が上） | エンティティのトランスフォーム、キャラクターの位置 |

## ビューポートからワールドへの変換

スクリーンピクセル座標を 2D ワールド空間座標に変換します。3D シーン内の位置を表す `Vec2` を返します。

```typescript
const worldPos = await coordinates.toWorld({ x: 150, y: 200 });
console.log("ワールド位置:", worldPos); // [x, y]
```

`x` と `y` はどちらもオプションです -- いずれかを省略すると、その軸にはスクリーンの中心が使用されます：

```typescript
// x 座標のみ変換（y はデフォルトで中心）
const pos = await coordinates.toWorld({ x: 500 });
```

**シグネチャ：**

```typescript
coordinates.toWorld(
  viewport?: { x?: number; y?: number }
): Promise<Vec2>
```

## ワールドからビューポートへの変換

3D ワールド位置をスクリーン座標に投影します。キャラクターやシーンオブジェクトに対して相対的に HTML オーバーレイやエフェクトを配置する場合に便利です。

```typescript
const screenPos = await coordinates.toViewport({ x: 0, y: 1.5, z: 0 });
console.log("スクリーン位置:", screenPos); // [x, y]
```

すべてのフィールドはオプションです -- いずれかを省略すると、その軸のワールド原点がデフォルトになります：

```typescript
// y のみ指定（x と z はデフォルトで 0）
const pos = await coordinates.toViewport({ y: 2.0 });
```

**シグネチャ：**

```typescript
coordinates.toViewport(
  world?: { x?: number; y?: number; z?: number }
): Promise<GlobalViewport>
```

## 型

| 型 | 定義 | 説明 |
|---|---|---|
| `World2d` | `Vec2` (`[number, number]`) | 2D ワールド座標のエイリアス |
| `World3d` | `Vec3` (`[number, number, number]`) | 3D ワールド座標のエイリアス |
| `GlobalViewport` | `[number, number]` | スクリーン空間座標 |
| `GlobalDisplay` | `interface` | 接続されたディスプレイ/モニターの情報 |

`Vec2`、`Vec3`、`Rect` については [Math Types](./math) を参照してください。

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

`Rect` の定義は [Math Types](./math) を参照してください。

## 次のステップ

- **[Entities](./entities)** -- ワールド空間のトランスフォームを使用してエンティティの位置決めとアニメーション。
- **[Displays](./displays)** -- 接続されたモニターとそのスクリーン空間矩形の照会。
