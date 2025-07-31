# Vrm.findByName()

Finds and returns an existing VRM character instance by its name. This method allows you to access characters that have
already been spawned by other parts of the application or by different mods.

## Parameters

- `vrmName` (string) - The name of the VRM character to find

## Returns

`Promise<Vrm>` - The VRM character instance with the specified name

## Description

The `findByName()` static method searches for an already-spawned VRM character by its name and returns a reference to
that character. This is useful for accessing characters created by other mods or parts of the application, enabling
cross-mod character interactions and shared character management.

## Character Names

Character names are typically derived from:

- The VRM model's metadata
- The asset filename (without extension)
- Custom naming during spawn operations

## Examples

### Basic Character Finding

```typescript
import {Vrm} from '@homunculus/sdk';

// Find a character by name
try {
    const alice = await Vrm.findByName('Alice');
    console.log('Found character:', await alice.name());

    // Interact with the found character
    await alice.setState('greeting');
    await alice.speakOnVoiceVox('Hello! Someone found me!');
} catch (error) {
    console.error('Character not found:', error);
}
```

## Common Use Cases

### Mod Coordination

Find characters created by other mods for cross-mod interactions.

### Character Discovery

Discover and interact with characters that may appear dynamically.

### System Integration

Access system-managed characters from custom mod code.

### Multi-Mod Communication

Enable communication between characters managed by different mods.

### Character Verification

Verify character availability before performing operations.

## Related Documentation

- **[Vrm.spawn()](spawn.md)** - Creating new characters
- **[Vrm.findAll()](./findAll.md)** - Getting all characters
- **[Vrm.waitLoadByName()](./waitLoadByName.md)** - Waiting for character loading
- **[VRM Character Management](index.md)** - Overall character system
- **[Vrm.name()](name.md)** - Getting character names