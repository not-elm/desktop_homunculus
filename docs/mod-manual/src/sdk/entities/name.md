# Name

Gets the human-readable name of an entity.

Most entities in Desktop Homunculus have names that make them easier to identify and work with. VRM models use their
character names, bones use standard bone names like "head", "leftHand", etc., and UI elements have descriptive names for
their functions.

## Parameters

- `entity`: The entity ID to get the name for

## Returns

Promise that resolves to the entity's name as a string.

## Examples

### Basic Name Retrieval

```typescript
// Get VRM character name
const vrmEntity = await entities.findByName("MyCharacter");
const name = await entities.name(vrmEntity);
console.log("Entity name:", name); // "MyCharacter"

// Get bone names
const headBone = await entities.findByName("head", {root: vrmEntity});
const boneName = await entities.name(headBone);
console.log("Bone name:", boneName); // "head"

// Get UI element names
const settingsPanel = await entities.findByName("SettingsPanel");
const panelName = await entities.name(settingsPanel);
console.log("Panel name:", panelName); // "SettingsPanel"
```

## Related Functions

- [`findByName()`](./findByName.md) - Find entities by their names
- [`transform()`](./transform.md) - Get entity position/rotation/scale
- [`setTransform()`](./setTransform.md) - Update entity transform
- [`Vrm.name()`](../vrm/name.md) - Get VRM character names specifically