# Vrm.spawn()

Creates and spawns a new VRM character instance from a mod asset, making it visible and interactive in the application.

```typescript
import {Vrm} from '@homunculus/sdk';

// Spawn a character with default settings
const character = await Vrm.spawn('characters/assistant.vrm');
console.log('Character spawned successfully');

// Character is ready for interaction
const name = await character.name();
console.log(`Spawned character: ${name}`);
```

```typescript
import {Vrm} from '@homunculus/sdk';

// Spawn character at a specific position
const character = await Vrm.spawn('characters/guide.vrm', {
    transform: {
        translation: [2.0, 0.0, 1.0],  // Move to position (2, 0, 1)
        scale: [1.2, 1.2, 1.2]         // Make 20% larger
    }
});

console.log('Character spawned at custom position');
```

## Parameters

- `asset` (string) - The mod asset path of the VRM model relative to `assets/mods/`.
- `options` (SpawnVrmOptions, optional) - Configuration options for spawning

### SpawnVrmOptions

```typescript
interface SpawnVrmOptions {
    transform?: Partial<Transform>;  // Initial position, rotation, and scale
}
```

### Transform

```typescript
interface Transform {
    translation: [number, number, number];  // Position [x, y, z]
    rotation: [number, number, number, number];  // Quaternion [x, y, z, w]
    scale: [number, number, number];  // Scale [x, y, z]
}
```

## Returns

`Promise<Vrm>` - A new VRM character instance that can be controlled and interacted with

## Description

The `spawn()` static method creates a new VRM character from a model asset and places it in the application environment.
The character starts in the `idle` state and is immediately available for interaction. This is the primary method for
creating character instances in mods.

## Common Use Cases

### Character Creation

Spawn main characters, NPCs, and interactive avatars for applications.

### Scene Population

Create multiple characters to populate environments and scenes.

### User Avatars

Allow users to select and spawn their preferred character representations.

### Dynamic Content

Spawn characters based on user actions, story progression, or application state.

### Multi-Character Experiences

Create group interactions, conversations, and collaborative scenarios.

## Related Documentation

- **[VRM Character Management](index.md)** - Overall character system
- **[Vrm.findByName()](findByName.md)** - Finding spawned characters
- **[Vrm.findAll()](./findAll.md)** - Getting all spawned characters
- **[Transform System](../math/index.md)** - Position, rotation, and scale management
- **[Mod Asset System](../mods/index.md)** - Managing VRM model assets