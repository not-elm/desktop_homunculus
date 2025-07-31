# VRM Transform Persistence

VRM Transform Persistence provides specialized functions for saving and loading VRM character positions, rotations, and
scales. These convenience functions simplify the process of remembering where users positioned their characters across
application sessions.

## Overview

The VRM transform system uses the character's name as a key to store and retrieve transform data. This makes it easy to
restore character positions when the application starts, or save them when characters are moved.

## Functions

#### loadVrmTransform

Loads a previously saved VRM character transform (position, rotation, scale) by character name.

Vrm's transform is automatically saved when the app exits, and this function retrieves the saved it.

This convenience function retrieves the transform (position, rotation, scale) for a specific VRM character. Unlike the
generic `load()` function, this method provides a default identity transform instead of throwing an error when no saved
data exists, making it safe to use without error handling.

## Parameters

- `vrmName` (string) - The name of the VRM character

## Returns

`Promise<Transform>` - A promise that resolves to the saved transform, or default identity transform if none exists

### saveVrmTransform

Saves a VRM character's transform (position, rotation, scale) for later restoration.

This convenience function stores the position, rotation, and scale of a VRM character so it can be restored in future
sessions. This is useful for remembering where users positioned their characters after dragging or programmatic
movement.

#### Parameters

- `vrmName` (string) - The name of the VRM character
- `transform` (Transform) - The transform data to save

#### Returns

`Promise<void>` - A promise that resolves when the transform has been saved

## Quick Example

```typescript
import {preferences, Vrm, entities} from '@homunculus/sdk';

// Save a character's current position
const vrm = await Vrm.findByName('MyCharacter');
const currentTransform = await entities.transform(vrm.entity);
await preferences.saveVrmTransform('MyCharacter', currentTransform);

// Later, restore the character's position
const savedTransform = await preferences.loadVrmTransform('MyCharacter');
await entities.setTransform(vrm.entity, savedTransform);
```

## Common Patterns

### Startup Restoration

Restore all character positions when the application starts:

```typescript
const characterNames = ['Alice', 'Bob', 'Charlie'];

for (const name of characterNames) {
    try {
        const vrm = await Vrm.findByName(name);
        const savedTransform = await preferences.loadVrmTransform(name);
        await entities.setTransform(vrm.entity, savedTransform);
    } catch (error) {
        console.log(`Could not restore ${name}`);
    }
}
```

### Auto-save on Movement

Save character positions automatically when they're moved:

```typescript
const vrm = await Vrm.spawn('characters/assistant.vrm');
const events = vrm.events();

events.on('drag-end', async () => {
    const name = await vrm.name();
    const transform = await entities.transform(vrm.entity);
    await preferences.saveVrmTransform(name, transform);
});
```

## Related Functions

- [`load()`](./load.md) - Load stored preferences with type safety
- [`save()`](./save.md) - Save data to the preferences store
