# Vrm.vrma()

Fetch or load a VRMA animation instance for the VRM character.
If the VRMA animation doesn't exist, it spawns a new one and returns it.

## Parameters

- `source` (string) - The VRMA animation path relative to `assets/mods` directory.

## Returns

`Promise<Vrma>` - A VRMA animation instance that can be controlled for playback

## Examples

### Basic Animation Loading

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters/dancer.vrm');

// Load and play a wave animation
const waveAnimation = await character.vrma('animations/wave-hello.vrma');
await waveAnimation.play({
    repeat: Repeat.count(3),
    transitionSecs: 0.5
});

console.log('Wave animation completed');
```

## Common Use Cases

### Character State Animations

Load different animations for different character behavioral states.

### Interactive Responses

Load reaction animations that play in response to user interactions.

### Cutscenes and Sequences

Load and coordinate multiple animations for storytelling sequences.

### Procedural Animation

Combine VRMA animations with bone manipulation for complex behaviors.

### User-Generated Content

Allow users to add their own animation assets to characters.

## Related Documentation

- **[VRMA Animation System](../vrma/index.md)** - Animation playback and control
- **[Vrma.play()](../vrma/play.md)** - Playing loaded animations
- **[Vrma.stop()](../vrma/stop.md)** - Stopping animations
- **[Repeat Class](../vrma/Repeat.md)** - Animation repeat configuration
- **[VRM Character Management](index.md)** - Overall character system