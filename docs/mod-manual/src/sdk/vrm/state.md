# Vrm.state()

Returns the current state of the VRM character. Character states control behavior patterns and can be used to coordinate
animations, interactions, and other character-specific logic.

## Parameters

None.

## Returns

`Promise<string>` - The current state of the VRM character

## Description

The `state()` method retrieves the current behavioral state of a VRM character. States are string identifiers that
represent the character's current condition or activity. The system has built-in states for common behaviors, and you
can also define custom states for mod-specific functionality.

## Built-in States

### Default States

- **`idle`** - Default resting state (assigned automatically on spawn)
- **`sitting`** - Character is in a sitting position
- **`drag`** - Character is being dragged by the user (automatically set/unset)

### Custom States

Any string can be used as a custom state for mod-specific behaviors:

- `greeting`, `talking`, `thinking`, `celebrating`, `sleeping`, etc.

## Examples

### Basic State Checking

```typescript
import {Vrm} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::assistant.vrm');

// Check current state
const currentState = await character.state();
console.log(`Character is currently: ${currentState}`); // "idle"

// State-based logic
if (currentState === 'idle') {
    console.log('Character is available for interaction');
} else if (currentState === 'drag') {
    console.log('Character is being moved');
} else {
    console.log(`Character is in custom state: ${currentState}`);
}

// Set a custom state
await character.setState('greeting');
```

## Common Use Cases

### Conditional Logic

Use current state to determine appropriate actions and responses.

### Animation Coordination

Check state before triggering animations to avoid conflicts.

### UI Updates

Update interface elements based on character state changes.

### Behavior Trees

Implement complex AI behaviors that depend on character states.

### State Persistence

Save and restore character states across application sessions.

## Related Documentation

- **[Vrm.setState()](setState.md)** - Changing character states
- **[Vrm.events()](events.md)** - Monitoring state changes via events
- **[VRM Character Management](index.md)** - Overall character system
- **[Preferences](../preferences/index.md)** - Persisting state information