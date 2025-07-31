# Finding Entities by Name

Finds an entity by its name, optionally within a specific parent entity.

This is the primary method for locating entities in the ECS system. Names are unique within their scope (global or under
a specific parent). Every VRM model, bone, UI element, and other game objects can be found using their human-readable
names.

## Parameters

- `name`: The name of the entity to find
- `options` (optional): Search parameters object
  - `root` (optional): Parent entity to search within

## Returns

Promise that resolves to the entity ID (number). Throws an error if no entity with the given name is found.

## Examples

### Basic Entity Finding

```typescript
// Find a VRM character globally
const vrmEntity = await entities.findByName("MyCharacter");
console.log("Found VRM entity:", vrmEntity);

// Find UI elements
const settingsPanel = await entities.findByName("SettingsPanel");
const chatWindow = await entities.findByName("ChatWindow");
```

### Finding Child Entities

```typescript
// Find a bone within a specific VRM
const vrmEntity = await entities.findByName("MyCharacter");
const headBone = await entities.findByName("head", {
    root: vrmEntity
});
console.log("Found head bone:", headBone);

// Find hand bones
const leftHand = await entities.findByName("leftHand", { root: vrmEntity });
const rightHand = await entities.findByName("rightHand", { root: vrmEntity });
```

## Related Functions

- [`name()`](./name.md) - Get the name of an entity
- [`transform()`](./transform.md) - Get entity position/rotation/scale
- [`setTransform()`](./setTransform.md) - Update entity transform
- [`Vrm.findByName()`](../vrm/findByName.md) - Find VRM models specifically