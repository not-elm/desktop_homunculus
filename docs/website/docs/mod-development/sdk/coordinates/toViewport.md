---
sidebar_position: 3
---

# toViewport

Project a 3D world position onto screen coordinates. Useful for positioning HTML overlays or effects relative to a character or scene object.

All fields are optional -- omit any to default to the world origin on that axis.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `world` | `{ x?: number; y?: number; z?: number }` (optional) | 3D world coordinates to convert; uses origin if not provided |

## Returns

`Promise<`[`GlobalViewport`](./types#globalviewport)`>`

## Example

```typescript
const screenPos = await coordinates.toViewport({ x: 0, y: 1.5, z: 0 });
console.log("Screen position:", screenPos); // [x, y]
```

```typescript
// Only specify y (x and z default to 0)
const pos = await coordinates.toViewport({ y: 2.0 });
```
