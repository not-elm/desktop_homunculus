---
title: "トゥイーン"
sidebar_position: 4
---

# トゥイーン

イージング関数を使用したスムーズなプロパティアニメーションです。トゥイーンは `entities` モジュールを通じてアクセスし、位置、回転、スケールを個別にアニメーションできます。

## インポート

```typescript
import { entities } from "@hmcs/sdk";
```

## 位置トゥイーン

`entities.tweenPosition(entityId, request)` は、エンティティの位置をターゲットの `[x, y, z]` 値まで指定した時間でスムーズにアニメーションします。

```typescript
await entities.tweenPosition(myEntity, {
  target: [100, 50, 0],
  durationMs: 1000,
  easing: "quadraticInOut",
});
```

`wait: true` を設定するとアニメーション完了までブロックします：

```typescript
await entities.tweenPosition(myEntity, {
  target: [0, 200, 0],
  durationMs: 500,
  easing: "cubicOut",
  wait: true,
});
// トゥイーン完了後に処理が続行されます
```

## 回転トゥイーン

`entities.tweenRotation(entityId, request)` は、回転をターゲットのクォータニオン `[x, y, z, w]` までアニメーションします。

```typescript
await entities.tweenRotation(myEntity, {
  target: [0, 0, 0.7071, 0.7071], // Z軸周りに90度
  durationMs: 500,
  easing: "elasticOut",
});
```

## スケールトゥイーン

`entities.tweenScale(entityId, request)` は、スケールをターゲットの `[x, y, z]` 値までアニメーションします。

```typescript
await entities.tweenScale(myEntity, {
  target: [2, 2, 2],
  durationMs: 800,
  easing: "bounceOut",
});
```

## イージング関数

`easing` パラメータはアニメーションの加速カーブを制御します。省略した場合、デフォルトは `"linear"` です。

| ファミリー | In | Out | InOut |
|--------|-----|------|-------|
| **Quadratic** | `quadraticIn` | `quadraticOut` | `quadraticInOut` |
| **Cubic** | `cubicIn` | `cubicOut` | `cubicInOut` |
| **Quartic** | `quarticIn` | `quarticOut` | `quarticInOut` |
| **Quintic** | `quinticIn` | `quinticOut` | `quinticInOut` |
| **Sine** | `sineIn` | `sineOut` | `sineInOut` |
| **Circular** | `circularIn` | `circularOut` | `circularInOut` |
| **Exponential** | `exponentialIn` | `exponentialOut` | `exponentialInOut` |
| **Elastic** | `elasticIn` | `elasticOut` | `elasticInOut` |
| **Back** | `backIn` | `backOut` | `backInOut` |
| **Bounce** | `bounceIn` | `bounceOut` | `bounceInOut` |
| **Smooth Step** | `smoothStepIn` | `smoothStepOut` | `smoothStep` |
| **Smoother Step** | `smootherStepIn` | `smootherStepOut` | `smootherStep` |

加えて `linear`（一定速度、加速なし）があります。

- **In** -- ゆっくり始まり、加速します
- **Out** -- 速く始まり、減速します
- **InOut** -- 両端でゆっくり、中間で速くなります

## 型定義

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
  target: [number, number, number, number]; // クォータニオン [x, y, z, w]
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

### EasingFunction

```typescript
type EasingFunction =
  | "linear"
  | "quadraticIn" | "quadraticOut" | "quadraticInOut"
  | "cubicIn" | "cubicOut" | "cubicInOut"
  // ... (全37種類、上記の表を参照)
  | "smootherStepIn" | "smootherStepOut" | "smootherStep";
```

## 使用例

### 画面外からスライドイン

```typescript
const entity = await entities.findByName("MyCharacter");

// 画面左の外から開始し、スライドイン
await entities.setTransform(entity, { translation: [-500, 0, 0] });
await entities.tweenPosition(entity, {
  target: [0, 0, 0],
  durationMs: 800,
  easing: "cubicOut",
  wait: true,
});
```

### バウンススケールエフェクト

```typescript
// バウンス付きの素早いスケールアップ
await entities.tweenScale(entity, {
  target: [1.5, 1.5, 1.5],
  durationMs: 300,
  easing: "bounceOut",
  wait: true,
});

// 元のサイズに戻す
await entities.tweenScale(entity, {
  target: [1, 1, 1],
  durationMs: 200,
  easing: "quadraticOut",
  wait: true,
});
```

### 並列トゥイーン

`wait` を省略（または `wait: false` に設定）すると、複数のトゥイーンを同時に実行できます：

```typescript
// 位置と回転が同時にアニメーション
entities.tweenPosition(entity, {
  target: [100, 100, 0],
  durationMs: 1000,
  easing: "sineInOut",
});

entities.tweenRotation(entity, {
  target: [0, 0, 0.3827, 0.9239], // Z軸周りに45度
  durationMs: 1000,
  easing: "sineInOut",
});
```

## 次のステップ

- **[SDK 概要](./)** -- 全モジュールマップとクイック例
