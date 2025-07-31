# Vrma.play()

Starts playback of a VRMA animation with optional configuration for repeat behavior and transition timing.

## Parameters

- `args?` (PlayArgs) - Optional configuration object for animation playback

### PlayArgs Interface

```typescript
interface PlayArgs {
    repeat?: Repeat;          // Repeat configuration (default: play once)
    transitionSecs?: number;  // Transition duration in seconds (default: 0)
    waitFinished?: boolean; // Whether to wait for playback to finish before resolving (default: false)
}
```

## Returns

`Promise<void>` - Resolves when the animation completes (or immediately for infinite loops)

## Examples

### Basic Animation Playback

```typescript
import {Vrm} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::dancer.vrm');
const waveAnimation = await character.vrma('gestures::wave.vrma');

// Play animation once with default settings
await waveAnimation.play();

console.log('Wave animation completed');
```

### Animation with Repeat Control

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::performer.vrm');

// Play animation multiple times
const applauseAnimation = await character.vrma('reactions::applause.vrma');
await applauseAnimation.play({
    repeat: Repeat.count(5)
});

// Play animation continuously
const idleAnimation = await character.vrma('ambient::idle-breathing.vrma');
await idleAnimation.play({
    repeat: Repeat.forever()
});

// Stop the looping animation after some time
setTimeout(async () => {
    await idleAnimation.stop();
}, 10000);
```

### Smooth Animation Transitions

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::expressive.vrm');

// Start with idle animation
const idleAnim = await character.vrma('states::idle.vrma');
await idleAnim.play({
    repeat: Repeat.forever()
});

// Transition smoothly to new animation
const excitedAnim = await character.vrma('emotions::excited.vrma');
await excitedAnim.play({
    repeat: Repeat.count(3),
    transitionSecs: 0.8  // 800ms smooth transition
});

// Smooth transition back to idle
await idleAnim.play({
    repeat: Repeat.forever(),
    transitionSecs: 1.2,  // 1.2 second smooth transition
    waitFinished: true  // Wait for playback to finish before continuing
});
```

## Related Documentation

- **[VRMA Animation System](./index.md)** - Main VRMA documentation
- **[Repeat Class](./Repeat.md)** - Animation repeat configuration
- **[Vrma.stop()](./stop.md)** - Stopping animation playback
- **[VRM Character Management](../vrm/index.md)** - Character control and states