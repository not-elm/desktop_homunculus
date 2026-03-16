---
sidebar_position: 6
---

# move

Repositions an entity using either **world** or **viewport** (screen-space) coordinates.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entity` | `number` | The entity ID to move |
| `target` | [`MoveTarget`](./types#movetarget) | The target position (world or viewport coordinates) |

## Returns

`Promise<void>`

## Example

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
