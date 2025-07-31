# Transform

The transform defines where the entity is positioned in 3D space, how it's rotated, and its scale factor. This is
fundamental for understanding entity positioning and for making positioning decisions.

## entities.transform()

Get the current transform (position, rotation, scale) of an entity.

### Parameters

- `entity`: The entity ID to get the transform for

### Returns

Promise that resolves to a `Transform` object containing:

- `translation`: Position in world space [x, y, z]
- `rotation`: Rotation as quaternion [x, y, z, w]
- `scale`: Scale factor [x, y, z]

## entities.setTransform()

Update the transform (position, rotation, scale) of an entity.

### Parameters

- `entity`: The entity ID to update
- `transform`: Partial transform data with the values to update

## Examples

### Basic Transform Retrieval

```typescript
// Get VRM character transform
const vrmEntity = await entities.findByName("MyCharacter");
const transform = await entities.transform(vrmEntity);

console.log("Position:", transform.translation);
console.log("Rotation:", transform.rotation);
console.log("Scale:", transform.scale);

// Extract individual components
const [x, y, z] = transform.translation;
console.log(`Character is at (${x}, ${y}, ${z})`);
```

## Related Functions

- [`findByName()`](./findByName.md) - Find entities to get transforms for
- [`cameras.worldToGlobalViewport()`](../cameras/worldToGlobalViewport.md) - Convert world positions to screen
  coordinates
