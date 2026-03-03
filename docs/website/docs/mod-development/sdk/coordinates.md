---
title: "Coordinates"
sidebar_position: 10
---

# Coordinates

Convert between screen-space (viewport) and 3D world-space positions. This is essential for placing effects, positioning webviews relative to characters, and mapping mouse/touch input to world locations.

## Import

```typescript
import { coordinates } from "@hmcs/sdk";
```

## Coordinate Systems

Desktop Homunculus uses two primary coordinate systems:

| System | Description | Example |
|---|---|---|
| **Global Viewport** | Screen pixels relative to the full desktop area | Mouse position, UI element placement |
| **World** | 3D scene coordinates (Y is up) | Entity transforms, character positions |

## Viewport to World

Convert screen-pixel coordinates to 2D world-space coordinates. Returns a `Vec2` representing the position in the 3D scene.

```typescript
const worldPos = await coordinates.toWorld({ x: 150, y: 200 });
console.log("World position:", worldPos); // [x, y]
```

Both `x` and `y` are optional -- omit either to use the screen center for that axis:

```typescript
// Convert only the x coordinate (y defaults to center)
const pos = await coordinates.toWorld({ x: 500 });
```

**Signature:**

```typescript
coordinates.toWorld(
  viewport?: { x?: number; y?: number }
): Promise<Vec2>
```

## World to Viewport

Project a 3D world position onto screen coordinates. Useful for positioning HTML overlays or effects relative to a character or scene object.

```typescript
const screenPos = await coordinates.toViewport({ x: 0, y: 1.5, z: 0 });
console.log("Screen position:", screenPos); // [x, y]
```

All fields are optional -- omit any to default to the world origin on that axis:

```typescript
// Only specify y (x and z default to 0)
const pos = await coordinates.toViewport({ y: 2.0 });
```

**Signature:**

```typescript
coordinates.toViewport(
  world?: { x?: number; y?: number; z?: number }
): Promise<GlobalViewport>
```

## Types

| Type | Definition | Description |
|---|---|---|
| `World2d` | `Vec2` (`[number, number]`) | Alias for 2D world coordinates |
| `World3d` | `Vec3` (`[number, number, number]`) | Alias for 3D world coordinates |
| `GlobalViewport` | `[number, number]` | Screen-space coordinates |
| `GlobalDisplay` | `interface` | Information about a connected display/monitor |

See [Math Types](./math) for `Vec2`, `Vec3`, and `Rect`.

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

See [Math Types](./math) for the `Rect` definition.

## Next Steps

- **[Entities](./entities)** -- Position and animate entities using world-space transforms.
- **[Displays](./displays)** -- Query connected monitors and their screen-space rectangles.
