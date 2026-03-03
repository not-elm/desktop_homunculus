---
title: "entities"
sidebar_position: 3
---

# entities

Query and manipulate Bevy ECS entities by name. Entities are the building blocks of the 3D scene -- VRM characters, cameras, webviews, and spawned objects are all entities with a numeric ID, an optional name, and a transform (position, rotation, scale).

## Import

```typescript
import { entities } from "@hmcs/sdk";
```

## Finding Entities

### findByName

Look up an entity by its human-readable name. Throws if no match is found.

```typescript
const vrmEntity = await entities.findByName("MyCharacter");
```

Pass a `root` option to search only within the children of a specific entity -- useful for finding bones inside a VRM hierarchy:

```typescript
const headBone = await entities.findByName("head", {
  root: vrmEntity,
});
```

**Signature:**

```typescript
entities.findByName(name: string, options?: FindOptions): Promise<number>
```

### Entity Names

Retrieve the name attached to an entity ID:

```typescript
const entityName = await entities.name(vrmEntity);
console.log(entityName); // "MyCharacter"
```

## Transform

Every entity has a **transform** describing its position, rotation, and scale in 3D space.

### Reading

```typescript
const t = await entities.transform(vrmEntity);
console.log("Position:", t.translation); // [x, y, z]
console.log("Rotation:", t.rotation);    // [x, y, z, w] quaternion
console.log("Scale:", t.scale);          // [x, y, z]
```

### Writing

`setTransform` accepts a **partial** transform -- only the fields you provide are updated:

```typescript
// Move the entity up 100 units (leave rotation and scale unchanged)
await entities.setTransform(vrmEntity, {
  translation: [0, 100, 0],
});

// Update all three components at once
await entities.setTransform(vrmEntity, {
  translation: [50, 0, -25],
  rotation: [0, 0.707, 0, 0.707], // 90-degree Y rotation
  scale: [1.5, 1.5, 1.5],
});
```

## Movement

`entities.move` repositions an entity using either **world** or **viewport** (screen-space) coordinates.

### World Coordinates

Set the entity's position directly in 3D world space. The `z` field is optional -- if omitted, the entity keeps its current z value.

```typescript
await entities.move(vrmEntity, {
  type: "world",
  position: [0, 1.5],
  z: -2,
});

// Keep current z
await entities.move(vrmEntity, {
  type: "world",
  position: [0, 1.5],
});
```

### Viewport Coordinates

Pass screen-pixel coordinates and the engine converts them to world space automatically:

```typescript
await entities.move(vrmEntity, {
  type: "viewport",
  position: [500, 300],
});
```

## Tweening

Smoothly animate an entity's position, rotation, or scale over time with easing functions. Each tween function accepts a `TweenPositionRequest`, `TweenRotationRequest`, or `TweenScaleRequest`.

### tweenPosition

```typescript
await entities.tweenPosition(vrmEntity, {
  target: [100, 50, 0],
  durationMs: 1000,
  easing: "quadraticInOut",
  wait: true, // block until animation finishes
});
```

### tweenRotation

```typescript
await entities.tweenRotation(vrmEntity, {
  target: [0, 0, 0.7071, 0.7071], // 90 degrees around Z
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
  wait: false, // fire-and-forget
});
```

All three share the same option fields:

| Field | Type | Default | Description |
|---|---|---|---|
| `target` | `[number, number, number]` (or 4 for rotation) | -- | Target value |
| `durationMs` | `number` | -- | Animation duration in milliseconds |
| `easing` | `EasingFunction` | `"linear"` | Acceleration curve |
| `wait` | `boolean` | `false` | Whether to block until the tween completes |

## Easing Functions

The `EasingFunction` type defines the acceleration curve for tweens. Import it as a type:

```typescript
import { type EasingFunction } from "@hmcs/sdk";
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

## Types

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

See [Math Types](./math) for `Transform`, `Vec2`, `Vec3`, and `Quat`.

## Next Steps

- **[Math Types](./math)** -- Type definitions for vectors, quaternions, and transforms.
- **[Coordinates](./coordinates)** -- Convert between screen-space and world-space positions.
- **[VRM Module](./vrm/)** -- Spawn and control VRM characters (which are entities under the hood).
