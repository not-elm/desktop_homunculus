---
sidebar_position: 100
---

# 型定義

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

### MoveTargetWorld

```typescript
interface MoveTargetWorld {
  type: "world";
  position: Vec2;
  z?: number;
}
```

### MoveTargetViewport

```typescript
interface MoveTargetViewport {
  type: "viewport";
  position: Vec2;
}
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

トゥイーンアニメーションの加速カーブを制御します。型としてインポートしてください：

```typescript
import { type EasingFunction } from "@hmcs/sdk";
```

```typescript
type EasingFunction =
  | "linear"
  | "quadraticIn" | "quadraticOut" | "quadraticInOut"
  | "cubicIn" | "cubicOut" | "cubicInOut"
  | "quarticIn" | "quarticOut" | "quarticInOut"
  | "quinticIn" | "quinticOut" | "quinticInOut"
  | "sineIn" | "sineOut" | "sineInOut"
  | "circularIn" | "circularOut" | "circularInOut"
  | "exponentialIn" | "exponentialOut" | "exponentialInOut"
  | "elasticIn" | "elasticOut" | "elasticInOut"
  | "backIn" | "backOut" | "backInOut"
  | "bounceIn" | "bounceOut" | "bounceInOut"
  | "smoothStepIn" | "smoothStepOut" | "smoothStep"
  | "smootherStepIn" | "smootherStepOut" | "smootherStep";
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

- **In** -- ゆっくり始まり、加速します
- **Out** -- 速く始まり、減速します
- **InOut** -- 両端でゆっくり、中間で速くなります

`Transform`、`Vec2`、`Vec3`、`Quat` については [Math Types](../math) を参照してください。
