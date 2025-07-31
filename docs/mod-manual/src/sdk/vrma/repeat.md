# Repeat Class

The Repeat class provides factory methods for configuring VRMA animation repeat behavior. It offers a clean, type-safe
way to specify how animations should loop or repeat during playback.

## Overview

The Repeat class uses static factory methods to create repeat configuration objects that control animation playback
patterns. This approach ensures type safety and provides clear, readable code for animation control.

## Static Methods

### `Repeat.forever()`

Creates a repeat configuration that loops the animation indefinitely until manually stopped.

**Returns:** `Repeat` - Configuration for infinite looping

**Example:**

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::dancer.vrm');
const idleAnimation = await character.vrma('animations::idle-breathing.vrma');

// Play animation continuously
await idleAnimation.play({
    repeat: Repeat.forever()
});

// Animation will loop until manually stopped
setTimeout(async () => {
    await idleAnimation.stop();
}, 10000); // Stop after 10 seconds
```

### `Repeat.count(count)`

Creates a repeat configuration that plays the animation a specific number of times.

**Parameters:**

- `count` (number) - The number of times to repeat the animation

**Returns:** `Repeat` - Configuration for counted repetition

**Example:**

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::performer.vrm');
const waveAnimation = await character.vrma('gestures::wave-hello.vrma');

// Play animation exactly 3 times
await waveAnimation.play({
    repeat: Repeat.count(3)
});

console.log('Wave animation completed 3 times');

// Play animation once (equivalent to count(1))
const bowAnimation = await character.vrma('gestures::bow.vrma');
await bowAnimation.play({
    repeat: Repeat.count(1)
});
```

### `Repeat.never()`

Creates a repeat configuration that plays the animation once without repetition.

**Returns:** `Repeat` - Configuration for single playback

**Example:**

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::storyteller.vrm');
const gestureAnimation = await character.vrma('story::dramatic-gesture.vrma');

// Play animation once
await gestureAnimation.play({
    repeat: Repeat.never()
});

console.log('Gesture animation played once');
```

## Usage Patterns

### Default Behavior

When no repeat configuration is specified, animations typically play once:

```typescript
const animation = await character.vrma('actions::jump.vrma');

// These are equivalent:
await animation.play();
await animation.play({repeat: Repeat.never()});
await animation.play({repeat: Repeat.count(1)});
```

## Related Documentation

- **[VRMA Animation System](./index.md)** - Main VRMA documentation
- **[Vrma.play()](./play.md)** - Animation playback method
- **[VRM Character Management](../vrm/index.md)** - Character control and states