# Entities API

The Entities API provides core functionality for managing ECS (Entity Component System) entities.
In Desktop Homunculus, everything is represented as entities in Bevy's ECS system - including VRM models, bones, UI
elements, and other game objects.

## Key Concepts

- **Entity**: A unique identifier in the ECS system
- **Name**: Human-readable identifier for entities
- **Transform**: Position, rotation, and scale data in 3D space
- **Hierarchy**: Entities can have parent-child relationships

## Functions

- [`findByName()`](./findByName.md) - Find entities by their name
- [`name()`](./name.md) - Get the human-readable name of an entity
- [`transform()`](./transform.md) - Get entity transform (position, rotation, scale)
- [`setTransform()`](./setTransform.md) - Update entity transform

## Quick Example

```typescript
// Find a VRM entity by name
const vrmEntity = await entities.findByName("MyCharacter");

// Get the current transform (position, rotation, scale)
const transform = await entities.transform(vrmEntity);
console.log("Position:", transform.translation);

// Move the VRM to a new position
await entities.setTransform(vrmEntity, {
    translation: [100, 0, 50]
});

// Find a bone within a specific VRM
const headBone = await entities.findByName("head", {root: vrmEntity});
```

## Common Use Cases

- **VRM Management**: Control character position and orientation
- **Bone Manipulation**: Access and modify VRM bone transforms
- **Object Positioning**: Place and move objects in 3D space
- **Hierarchy Navigation**: Find children and parent entities
- **Animation Control**: Modify transforms for custom animations
