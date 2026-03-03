---
title: "Entities"
sidebar_position: 3
---

# Entities

名前による Bevy ECS エンティティ（entity）の検索と操作を行います。エンティティは 3D シーンの構成要素です -- VRM キャラクター、カメラ、WebView、スポーンされたオブジェクトはすべて、数値 ID、オプションの名前、トランスフォーム（位置、回転、スケール）を持つエンティティです。

## インポート

```typescript
import { entities } from "@hmcs/sdk";
```

## エンティティの検索

### findByName

人間が読める名前でエンティティを検索します。一致するものが見つからない場合はエラーをスローします。

```typescript
const vrmEntity = await entities.findByName("MyCharacter");
```

`root` オプションを渡すと、特定のエンティティの子要素内のみを検索します -- VRM 階層内のボーンを見つける場合に便利です：

```typescript
const headBone = await entities.findByName("head", {
  root: vrmEntity,
});
```

**シグネチャ：**

```typescript
entities.findByName(name: string, options?: FindOptions): Promise<number>
```

### エンティティ名

エンティティ ID に紐づけられた名前を取得します：

```typescript
const entityName = await entities.name(vrmEntity);
console.log(entityName); // "MyCharacter"
```

## トランスフォーム

すべてのエンティティは、3D 空間での位置、回転、スケールを記述する **トランスフォーム** を持っています。

### 読み取り

```typescript
const t = await entities.transform(vrmEntity);
console.log("位置:", t.translation); // [x, y, z]
console.log("回転:", t.rotation);    // [x, y, z, w] クォータニオン
console.log("スケール:", t.scale);   // [x, y, z]
```

### 書き込み

`setTransform` は **部分的な** トランスフォームを受け付けます -- 指定したフィールドのみが更新されます：

```typescript
// エンティティを上方向に 100 ユニット移動（回転とスケールは変更なし）
await entities.setTransform(vrmEntity, {
  translation: [0, 100, 0],
});

// 3 つのコンポーネントを一度に更新
await entities.setTransform(vrmEntity, {
  translation: [50, 0, -25],
  rotation: [0, 0.707, 0, 0.707], // Y 軸 90 度回転
  scale: [1.5, 1.5, 1.5],
});
```

## 移動

`entities.move` は、**ワールド** 座標または **ビューポート**（スクリーン空間）座標を使用してエンティティを再配置します。

### ワールド座標

3D ワールド空間でエンティティの位置を直接設定します。`z` フィールドはオプションです -- 省略した場合、エンティティは現在の z 値を保持します。

```typescript
await entities.move(vrmEntity, {
  type: "world",
  position: [0, 1.5],
  z: -2,
});

// 現在の z を保持
await entities.move(vrmEntity, {
  type: "world",
  position: [0, 1.5],
});
```

### ビューポート座標

スクリーンピクセル座標を渡すと、エンジンが自動的にワールド空間に変換します：

```typescript
await entities.move(vrmEntity, {
  type: "viewport",
  position: [500, 300],
});
```

## トゥイーン

イージング関数を使用して、エンティティの位置、回転、スケールを時間経過でスムーズにアニメーションさせます。各トゥイーン関数は `TweenPositionRequest`、`TweenRotationRequest`、`TweenScaleRequest` を受け付けます。

### tweenPosition

```typescript
await entities.tweenPosition(vrmEntity, {
  target: [100, 50, 0],
  durationMs: 1000,
  easing: "quadraticInOut",
  wait: true, // アニメーション完了までブロック
});
```

### tweenRotation

```typescript
await entities.tweenRotation(vrmEntity, {
  target: [0, 0, 0.7071, 0.7071], // Z 軸 90 度
  durationMs: 500,
  easing: "elasticOut",
});
```

### tweenScale

```typescript
await entities.tweenScale(vrmEntity, {
  target: [2, 2, 2],
  durationMs: 800,
  easing: "bounceOut",
  wait: false, // ファイアアンドフォーゲット
});
```

3 つすべてが同じオプションフィールドを共有しています：

| フィールド | 型 | デフォルト | 説明 |
|---|---|---|---|
| `target` | `[number, number, number]`（回転の場合は 4 要素） | -- | ターゲット値 |
| `durationMs` | `number` | -- | ミリ秒単位のアニメーション持続時間 |
| `easing` | `EasingFunction` | `"linear"` | 加速カーブ |
| `wait` | `boolean` | `false` | トゥイーン完了までブロックするかどうか |

## イージング関数

`EasingFunction` 型は、トゥイーンの加速カーブを定義します。型としてインポートしてください：

```typescript
import { type EasingFunction } from "@hmcs/sdk";
```

利用可能な値：

| グループ | In | Out | InOut |
|---|---|---|---|
| Linear | `"linear"` | -- | -- |
| Quadratic | `"quadraticIn"` | `"quadraticOut"` | `"quadraticInOut"` |
| Cubic | `"cubicIn"` | `"cubicOut"` | `"cubicInOut"` |
| Quartic | `"quarticIn"` | `"quarticOut"` | `"quarticInOut"` |
| Quintic | `"quinticIn"` | `"quinticOut"` | `"quinticInOut"` |
| Sine | `"sineIn"` | `"sineOut"` | `"sineInOut"` |
| Circular | `"circularIn"` | `"circularOut"` | `"circularInOut"` |
| Exponential | `"exponentialIn"` | `"exponentialOut"` | `"exponentialInOut"` |
| Elastic | `"elasticIn"` | `"elasticOut"` | `"elasticInOut"` |
| Back | `"backIn"` | `"backOut"` | `"backInOut"` |
| Bounce | `"bounceIn"` | `"bounceOut"` | `"bounceInOut"` |
| Smooth Step | `"smoothStepIn"` | `"smoothStepOut"` | `"smoothStep"` |
| Smoother Step | `"smootherStepIn"` | `"smootherStepOut"` | `"smootherStep"` |

## 型

### FindOptions

```typescript
interface FindOptions {
  /** この指定エンティティの子要素に検索を制限します。 */
  root?: number;
}
```

### MoveTarget

```typescript
type MoveTarget =
  | { type: "world"; position: Vec2; z?: number }
  | { type: "viewport"; position: Vec2 };
```

### TweenPositionRequest

```typescript
interface TweenPositionRequest {
  target: [number, number, number];
  durationMs: number;
  easing?: EasingFunction;
  wait?: boolean;
}
```

### TweenRotationRequest

```typescript
interface TweenRotationRequest {
  target: [number, number, number, number];
  durationMs: number;
  easing?: EasingFunction;
  wait?: boolean;
}
```

### TweenScaleRequest

```typescript
interface TweenScaleRequest {
  target: [number, number, number];
  durationMs: number;
  easing?: EasingFunction;
  wait?: boolean;
}
```

`Transform`、`Vec2`、`Vec3`、`Quat` については [Math Types](./math) を参照してください。

## 次のステップ

- **[Math Types](./math)** -- ベクトル、クォータニオン、トランスフォームの型定義。
- **[Coordinates](./coordinates)** -- スクリーン空間とワールド空間の座標変換。
- **[VRM モジュール](./vrm/)** -- VRM キャラクターのスポーンと制御（内部的にはエンティティです）。
