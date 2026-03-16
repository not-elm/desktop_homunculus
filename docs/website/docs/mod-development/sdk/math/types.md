---
sidebar_position: 100
---

# Type Definitions

## Transform

A full 3D transformation with position, rotation, and scale. Returned by `entities.transform()` and used throughout the VRM and entity APIs.

```typescript
interface Transform {
  /** Position in world space: [x, y, z]. Y is up. */
  translation: [number, number, number];
  /** Rotation as a quaternion: [x, y, z, w]. Identity is [0, 0, 0, 1]. */
  rotation: [number, number, number, number];
  /** Scale factor: [x, y, z]. Normal size is [1, 1, 1]. */
  scale: [number, number, number];
}
```

Example identity transform:

```typescript
const identity: Transform = {
  translation: [0, 0, 0],
  rotation: [0, 0, 0, 1],
  scale: [1, 1, 1],
};
```

## TransformArgs

A partial version of `Transform` for update operations. Only the fields you include are changed -- the rest stay at their current values.

```typescript
interface TransformArgs {
  translation?: Vec3;
  rotation?: Quat;
  scale?: Vec3;
}
```

```typescript
// Move an entity up without changing rotation or scale
const args: TransformArgs = {
  translation: [0, 100, 0],
};
```

## Vec2

A 2-element tuple for screen coordinates, UI positions, and 2D math.

```typescript
type Vec2 = [number, number]; // [x, y]
```

## Vec3

A 3-element tuple for 3D positions, directions, and scale values.

```typescript
type Vec3 = [number, number, number]; // [x, y, z]
```

## Quat

A 4-element tuple representing a rotation. `[0, 0, 0, 1]` is the identity (no rotation).

```typescript
type Quat = [number, number, number, number]; // [x, y, z, w]
```

Common values:

| Rotation | Quat |
|---|---|
| Identity (none) | `[0, 0, 0, 1]` |
| 90 degrees around Y | `[0, 0.7071, 0, 0.7071]` |
| 180 degrees around Y | `[0, 1, 0, 0]` |

## Rect

A 2D rectangle defined by its minimum and maximum corner points.

```typescript
interface Rect {
  min: Vec2;
  max: Vec2;
}
```

```typescript
const bounds: Rect = {
  min: [0, 0],
  max: [1920, 1080],
};
```
