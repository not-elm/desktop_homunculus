# Vrma.stop()

Immediately stops the playback of a VRMA animation, interrupting any ongoing playback or looping.

## Syntax

```typescript
await vrma.stop()
```

## Parameters

None.

## Returns

`Promise<void>` - Resolves when the animation has been stopped

## Examples

### Basic Animation Stopping

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::dancer.vrm');

// Start a looping idle animation
const idleAnimation = await character.vrma('ambient::idle-breathing.vrma');
await idleAnimation.play({
    repeat: Repeat.forever()
});

// Stop the animation after 5 seconds
setTimeout(async () => {
    await idleAnimation.stop();
    console.log('Idle animation stopped');
}, 5000);
```

## Common Use Cases

### User-Initiated Stops

- Stopping background animations when user interacts with character
- Interrupting long animations for more responsive interactions
- Pausing animations during drag operations

### State Management

- Stopping previous state animations before transitioning to new states
- Clearing all animations during character resets
- Managing animation conflicts between different behaviors

### Performance Optimization

- Stopping resource-intensive animations when not visible
- Limiting concurrent animations to maintain performance
- Cleaning up animations when characters are no longer needed

### Error Recovery

- Stopping problematic animations that might be causing issues
- Resetting animation state after errors
- Emergency stops for debugging purposes

## Related Documentation

- **[VRMA Animation System](./index.md)** - Main VRMA documentation
- **[Vrma.play()](./play.md)** - Animation playback method
- **[Repeat Class](./Repeat.md)** - Animation repeat configuration
- **[VRM Character Management](../vrm/index.md)** - Character control and states