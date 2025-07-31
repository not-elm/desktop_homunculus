# Transform

Represents a complete 3D transformation containing position, rotation, and scale components. This is the core type for
positioning and orienting objects in 3D space throughout the Desktop Homunculus ecosystem.

## Type Definition

```typescript
interface Transform {
    translation: [number, number, number];
    rotation: [number, number, number, number];
    scale: [number, number, number];
}
```

## Properties

- **translation**: Position in world space as [x, y, z] coordinates
- **rotation**: Orientation as a quaternion in [x, y, z, w] format
- **scale**: Size multiplier as [x, y, z] values (1.0 = normal size)

## Examples

### Identity Transform

```typescript
// Default transform - no translation, rotation, or scaling
const identity: Transform = {
    translation: [0, 0, 0],
    rotation: [0, 0, 0, 1],
    scale: [1, 1, 1]
};
```

### Positioning VRM Characters

```typescript
// Place VRM 50 units forward and 100 units up
const characterPosition: Transform = {
    translation: [0, 100, 50],
    rotation: [0, 0, 0, 1],    // No rotation
    scale: [1, 1, 1]           // Normal size
};

await Deno.api.entities.setTransform(vrmEntity, characterPosition);
```

## Partial Updates

You can provide partial transforms to update only specific components:

```typescript
// Update only position
const newPosition: Partial<Transform> = {
    translation: [100, 0, 0]
};
await Deno.api.entities.setTransform(entity, newPosition);

// Update only rotation
const newRotation: Partial<Transform> = {
    rotation: [0, 0.707, 0, 0.707]
};
await Deno.api.entities.setTransform(entity, newRotation);

// Update position and scale, keep rotation
const positionAndScale: Partial<Transform> = {
    translation: [50, 100, 0],
    scale: [1.5, 1.5, 1.5]
};
await Deno.api.entities.setTransform(entity, positionAndScale);
```

## Working with Entity Transforms

### Getting Current Transform

```typescript
// Get the current transform of an entity
const currentTransform = await Deno.api.entities.transform(entityId);
console.log('Position:', currentTransform.translation);
console.log('Rotation:', currentTransform.rotation);
console.log('Scale:', currentTransform.scale);
```

### Modifying Existing Transforms

```typescript
// Move entity relative to current position
const current = await Deno.api.entities.transform(entityId);
const moved: Transform = {
    translation: [
        current.translation[0] + 50,  // Move 50 units right
        current.translation[1],       // Keep Y position
        current.translation[2] + 25   // Move 25 units forward
    ],
    rotation: current.rotation,     // Keep rotation
    scale: current.scale           // Keep scale
};

await Deno.api.entities.setTransform(entityId, moved);
```

## Related Types

- **[Vec3](./Vec3.md)** - Used internally for translation and scale vectors
- **[Vec2](./Vec2.md)** - Used for 2D positioning operations
- **[Rect](./Rect.md)** - Used for 2D bounds and regions

## Related Documentation

- [Entities](../entities/index.md) - Primary interface for manipulating transforms
- [VRM](../vrm/index.md) - VRM characters use transforms for positioning
- [Cameras](../cameras/index.md) - Convert between coordinate systems