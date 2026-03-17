---
sidebar_position: 100
---

# Type Definitions

### FindOptions

```typescript
interface FindOptions {
  /** Limit search to children of this entity. */
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
  target: [number, number, number, number]; // quaternion [x, y, z, w]
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

Controls the acceleration curve for tween animations. Import it as a type:

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

Available values:

| Group | In | Out | InOut |
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

- **In** -- starts slow, accelerates
- **Out** -- starts fast, decelerates
- **InOut** -- slow at both ends, fast in the middle

See [Math Types](../math) for `Transform`, `Vec2`, `Vec3`, and `Quat`.
