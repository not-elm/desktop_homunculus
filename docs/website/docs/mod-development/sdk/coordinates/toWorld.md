---
sidebar_position: 2
---

# toWorld

Convert screen-pixel coordinates to 2D world-space coordinates. Returns a `Vec2` representing the position in the 3D scene.

Both `x` and `y` are optional -- omit either to use the screen center for that axis.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `viewport` | `{ x?: number; y?: number }` (optional) | Screen coordinates to convert; uses center if not provided |

## Returns

`Promise<Vec2>`

## Example

```typescript
const worldPos = await coordinates.toWorld({ x: 150, y: 200 });
console.log("World position:", worldPos); // [x, y]
```

```typescript
// Convert only the x coordinate (y defaults to center)
const pos = await coordinates.toWorld({ x: 500 });
```
