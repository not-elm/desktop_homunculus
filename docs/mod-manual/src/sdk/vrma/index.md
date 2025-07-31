# VRM Animation System

The VRMA (VRM Animation) system provides sophisticated animation control for VRM characters in Desktop Homunculus. VRMA
files contain pre-recorded animations that can be applied to VRM models, enabling rich character expressions, movements,
and behaviors.

## Overview

VRMA animations are motion capture or hand-crafted animation sequences stored in the VRMA format. The animation system
supports:

- **Animation Playback**: Play, pause, and stop VRMA animations
- **Repeat Control**: Configure looping and repetition patterns
- **Smooth Transitions**: Blend between animations with customizable timing
- **Asset Integration**: Load animations from mod assets
- **Multi-Character Support**: Apply different animations to multiple VRM instances

## Core Concepts

### Repeat Patterns

VRMA animations support various repeat behaviors:

- **Never**: Play once and stop
- **Count**: Repeat a specific number of times
- **Forever**: Loop continuously until manually stopped

### Smooth Transitions

Animations can transition smoothly into each other with configurable durations to avoid jarring changes between
different movement patterns.

## API Reference

### Vrma Class

The main class for controlling individual VRMA animation instances.

#### Constructor

Creates a VRMA instance wrapper around an animation entity ID.
Typically obtained through [`vrmInstance.vrma()`](../vrm/vrma.md) method rather than direct construction.

#### Methods

- **[play(args?)](./play.md)** - Start animation playback with optional configuration
- **[stop()](./stop.md)** - Stop animation playback immediately

### Repeat Class

Utility class for configuring animation repeat behavior.

- **[Repeat](./Repeat.md)** - Repeat configuration class with static factory methods

## Usage Examples

### Basic Animation Playback

```typescript
import {Vrm} from '@homunculus/sdk';

// Spawn a character
const character = await Vrm.spawn('animated-character::dancer.vrm');

// Get an animation instance
const waveAnimation = await character.vrma('animations::wave-hello.vrma');

// Play the animation once
await waveAnimation.play();

console.log('Wave animation completed');
```

### Animation with Repeat Control

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::performer.vrm');

// Get dance animation
const danceAnimation = await character.vrma('animations::dance-loop.vrma');

// Play animation 5 times
await danceAnimation.play({
    repeat: Repeat.count(5)
});

// Play animation forever (until manually stopped)
const idleAnimation = await character.vrma('animations::idle-breathing.vrma');
await idleAnimation.play({
    repeat: Repeat.forever()
});

// Stop the looping animation after 10 seconds
setTimeout(async () => {
    await idleAnimation.stop();
}, 10000);
```

### Smooth Animation Transitions

```typescript
import {Vrm, Repeat} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::expressive.vrm');

// Start with idle animation
const idleAnim = await character.vrma('animations::idle.vrma');
await idleAnim.play({
    repeat: Repeat.forever()
});

// Transition to greeting with smooth blend
const greetingAnim = await character.vrma('animations::greeting.vrma');
await greetingAnim.play({
    transitionSecs: 0.5,  // 500ms smooth transition
    repeat: Repeat.count(1)
});

// Transition back to idle
await idleAnim.play({
    transitionSecs: 1.0,  // 1 second smooth transition
    repeat: Repeat.forever()
});
```

## Related Documentation

- **[VRM Character Management](../vrm/index.md)** - Character spawning and control
- **[Entity Management](../entities/index.md)** - Transform and positioning
- **[GPT Integration](../gpt/index.md)** - AI-powered character behaviors
- **[Effects System](../effects/index.md)** - Visual effects coordination