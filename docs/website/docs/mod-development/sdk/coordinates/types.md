---
sidebar_position: 100
---

# Type Definitions

### GlobalViewport

```typescript
type GlobalViewport = [number, number];
```

Screen-space coordinates as `[x, y]`.

### GlobalDisplay

```typescript
interface GlobalDisplay {
  /** Unique display identifier. */
  id: number;
  /** Human-readable display name. */
  title: string;
  /** Display frame rectangle in screen coordinates. */
  frame: Rect;
}
```

See [Math Types](../math) for the `Rect` definition.

### World2d

```typescript
type World2d = Vec2; // [number, number]
```

Alias for 2D world coordinates.

### World3d

```typescript
type World3d = Vec3; // [number, number, number]
```

Alias for 3D world coordinates.

See [Math Types](../math) for `Vec2` and `Vec3`.
