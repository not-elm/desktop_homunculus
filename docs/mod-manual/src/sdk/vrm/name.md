# Vrm.name()

Returns the name of the VRM character. This is the identifier that can be used to find and reference the character in
various operations.

## Parameters

None.

## Returns

`Promise<string>` - The name of the VRM character

## Description

The `name()` method retrieves the display name of a VRM character. This name is typically derived from the VRM model's
metadata or from the asset filename. Character names are useful for identification, logging, user interfaces, and
persistence operations.

## Examples

### Basic Name Retrieval

```typescript
import {Vrm} from '@homunculus/sdk';

// Spawn a character
const character = await Vrm.spawn('characters::alice.vrm');

// Get the character's name
const name = await character.name();
console.log(`Character name: ${name}`); // "Alice" or filename-based name

// Use name in logging
console.log(`${name} is ready for interaction`);
```

## Common Use Cases

### Character Identification

Use names to identify and track specific characters in complex scenarios.

### User Interfaces

Display character names in menus, lists, and status displays.

### Persistence Systems

Use names as keys for saving and loading character-specific data.

### Logging and Debugging

Include character names in log messages for better debugging.

### Multi-Character Coordination

Reference characters by name when coordinating behaviors between multiple characters.

## Related Documentation

- **[Vrm.findByName()](findByName.md)** - Finding characters by name
- **[Vrm.waitLoadByName()](./waitLoadByName.md)** - Waiting for named characters to load
- **[Preferences](../preferences/index.md)** - Storing character data by name
- **[VRM Character Management](index.md)** - Overall character system