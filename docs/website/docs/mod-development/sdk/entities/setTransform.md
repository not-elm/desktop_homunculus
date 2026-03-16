---
sidebar_position: 5
---

# setTransform

Updates the transform (position, rotation, scale) of an entity. Accepts a **partial** transform -- only the fields you provide are updated.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entity` | `number` | The entity ID to update |
| `transform` | `Partial<Transform>` | Partial transform data with the values to update |

## Returns

`Promise<void>`

## Example

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
