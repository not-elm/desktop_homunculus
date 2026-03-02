---
title: "Tweening"
sidebar_position: 4
---

# Tweening

Smooth property animations using easing functions. Tweening is accessed through the `entities` module and supports animating position, rotation, and scale independently.

## Import

```typescript
import { entities } from "@hmcs/sdk";
```

## Position Tween

`entities.tweenPosition(entityId, request)` smoothly animates an entity's position to a target `[x, y, z]` value over a given duration.

```typescript
await entities.tweenPosition(myEntity, {
  target: [100, 50, 0],
  durationMs: 1000,
  easing: "quadraticInOut",
});
```

Set `wait: true` to block until the animation completes:

```typescript
await entities.tweenPosition(myEntity, {
  target: [0, 200, 0],
  durationMs: 500,
  easing: "cubicOut",
  wait: true,
});
// Execution continues only after the tween finishes
```

## Rotation Tween

`entities.tweenRotation(entityId, request)` animates rotation to a target quaternion `[x, y, z, w]`.

```typescript
await entities.tweenRotation(myEntity, {
  target: [0, 0, 0.7071, 0.7071], // 90 degrees around Z axis
  durationMs: 500,
  easing: "elasticOut",
});
```

## Scale Tween

`entities.tweenScale(entityId, request)` animates scale to a target `[x, y, z]` value.

```typescript
await entities.tweenScale(myEntity, {
  target: [2, 2, 2],
  durationMs: 800,
  easing: "bounceOut",
});
```

## Easing Functions

The `easing` parameter controls the acceleration curve of the animation. If omitted, it defaults to `"linear"`.

| Family | In | Out | InOut |
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

Plus `linear` (constant speed, no acceleration).

- **In** -- starts slow, accelerates
- **Out** -- starts fast, decelerates
- **InOut** -- slow at both ends, fast in the middle

## Types

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

```typescript
type EasingFunction =
  | "linear"
  | "quadraticIn" | "quadraticOut" | "quadraticInOut"
  | "cubicIn" | "cubicOut" | "cubicInOut"
  // ... (37 total values, see table above)
  | "smootherStepIn" | "smootherStepOut" | "smootherStep";
```

## Examples

### Slide in from offscreen

```typescript
const entity = await entities.findByName("MyCharacter");

// Start offscreen to the left, then slide in
await entities.setTransform(entity, { translation: [-500, 0, 0] });
await entities.tweenPosition(entity, {
  target: [0, 0, 0],
  durationMs: 800,
  easing: "cubicOut",
  wait: true,
});
```

### Bounce scale effect

```typescript
// Quick scale-up with bounce
await entities.tweenScale(entity, {
  target: [1.5, 1.5, 1.5],
  durationMs: 300,
  easing: "bounceOut",
  wait: true,
});

// Return to normal
await entities.tweenScale(entity, {
  target: [1, 1, 1],
  durationMs: 200,
  easing: "quadraticOut",
  wait: true,
});
```

### Parallel tweens

Omit `wait` (or set `wait: false`) to run multiple tweens simultaneously:

```typescript
// Position and rotation animate at the same time
entities.tweenPosition(entity, {
  target: [100, 100, 0],
  durationMs: 1000,
  easing: "sineInOut",
});

entities.tweenRotation(entity, {
  target: [0, 0, 0.3827, 0.9239], // 45 degrees around Z
  durationMs: 1000,
  easing: "sineInOut",
});
```

## Next Steps

- **[SDK Overview](./)** -- Full module map and quick example
