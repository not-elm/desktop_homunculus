# unlook()

Disable the look-at functionality for a VRM character, returning them to their default gaze behavior.

## Syntax

```typescript
await vrm.unlook(): Promise<void>
```

## Description

The `unlook()` method disables any active look-at behavior for the VRM character, whether they were tracking the cursor or looking at a specific target entity. After calling this method, the character will return to their natural, animation-driven gaze behavior.

This is useful for:
- Ending interactive gaze sequences
- Switching between different interaction modes
- Resetting character attention for new scenarios
- Allowing animations to control eye movement naturally

## Parameters

None.

## Return Value

A `Promise<void>` that resolves when the look-at functionality has been successfully disabled.

## Example

```typescript
import { Vrm } from '@desktop-homunculus/sdk';

// Find a VRM character
const character = await Vrm.findByName('MyCharacter');

// Enable cursor tracking
await character.lookAtCursor();

// Character follows cursor for a while...
await new Promise(resolve => setTimeout(resolve, 5000));

// Disable look-at behavior
await character.unlook();

// Character now uses natural gaze from animations
```

## Interactive Sequence Example

```typescript
async function interactionSequence() {
  const guide = await Vrm.findByName('Guide');
  
  // Phase 1: Get user's attention with cursor tracking
  await guide.lookAtCursor();
  await gpt.chat('Hello! I see you looking at me.', {
    vrm: guide.entity,
    speaker: 1
  });
  
  // Phase 2: Look at something specific to direct attention
  const importantObject = await entities.findByName('ImportantObject');
  await guide.lookAtTarget(importantObject);
  await gpt.chat('Let me show you this interesting object.', {
    vrm: guide.entity,
    speaker: 1
  });
  
  // Phase 3: Return to natural behavior
  await guide.unlook();
  await gpt.chat('Now I can move naturally again!', {
    vrm: guide.entity,
    speaker: 1
  });
  
  // Play an animation that includes natural eye movement
  const idleAnimation = await guide.vrma('idle-animation');
  await idleAnimation.play();
}
```

## State Management Example

```typescript
class CharacterController {
  private character: Vrm;
  private currentLookMode: 'none' | 'cursor' | 'target' = 'none';
  
  constructor(character: Vrm) {
    this.character = character;
  }
  
  async setCursorTracking() {
    await this.character.unlook(); // Clear any existing look-at
    await this.character.lookAtCursor();
    this.currentLookMode = 'cursor';
  }
  
  async setTargetTracking(target: number) {
    await this.character.unlook(); // Clear any existing look-at
    await this.character.lookAtTarget(target);
    this.currentLookMode = 'target';
  }
  
  async setNaturalGaze() {
    await this.character.unlook();
    this.currentLookMode = 'none';
  }
  
  getCurrentMode() {
    return this.currentLookMode;
  }
}
```

## Performance Considerations

```typescript
// Good practice: Always unlook before setting new targets
async function switchLookTarget(character: Vrm, newTarget: number) {
  await character.unlook();        // Clear previous target
  await character.lookAtTarget(newTarget);  // Set new target
}

// Avoid rapid switching without unlooking first
// This could cause performance issues:
// await character.lookAtTarget(target1);
// await character.lookAtTarget(target2);  // Bad: doesn't clear first

// Better approach:
await character.lookAtTarget(target1);
await character.unlook();
await character.lookAtTarget(target2);
```

## Related Methods

- [`lookAtCursor()`](look-at-cursor.md) - Enable cursor tracking
- [`lookAtTarget()`](look-at-target.md) - Look at a specific entity
- [`state()`](state.md) - Manage character states
- [`vrma()`](vrma.md) - Control character animations

## Notes

- Always call `unlook()` before setting a new look-at target for best performance
- The method is safe to call even if no look-at behavior is currently active
- Natural gaze behavior depends on the currently playing animations
- Character state changes (like playing animations) may override look-at behavior automatically