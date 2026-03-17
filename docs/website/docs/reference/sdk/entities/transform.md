---
sidebar_position: 4
---

# transform

Gets the current transform (position, rotation, scale) of an entity.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entity` | `number` | The entity ID to get the transform for |

## Returns

`Promise<Transform>`

## Example

```typescript
const t = await entities.transform(vrmEntity);
console.log("Position:", t.translation); // [x, y, z]
console.log("Rotation:", t.rotation);    // [x, y, z, w] quaternion
console.log("Scale:", t.scale);          // [x, y, z]
```
