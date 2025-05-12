# lookAtCursor()

Configure a VRM character to look at the mouse cursor position in real-time.

## Syntax

```typescript
await vrm.lookAtCursor(): Promise<void>
```

## Description

The `lookAtCursor()` method configures the VRM character to continuously track the mouse cursor position with their gaze. This creates a dynamic interaction where the character appears to follow the user's mouse movements, enhancing the sense of presence and responsiveness.

Once enabled, the character will automatically adjust their head and eye position to look toward the current cursor location on screen. This effect remains active until disabled with `unlook()` or overridden by another look-at target.

## Parameters

None.

## Return Value

A `Promise<void>` that resolves when the cursor tracking has been successfully enabled.

## Example

```typescript
import { Vrm } from '@desktop-homunculus/sdk';

// Find an existing VRM character
const character = await Vrm.findByName('MyCharacter');

// Enable cursor tracking
await character.lookAtCursor();

// The character will now follow the mouse cursor with their gaze
// Users will see the character's eyes and head orient toward their cursor position

// Later, disable the look-at behavior
await character.unlook();
```

## Real-World Usage

```typescript
// Create an interactive guide character
const guide = await Vrm.spawn('guide-character');

// Position the character
await entities.setTransform(guide.entity, {
  translation: [0, 0, 2]
});

// Make them look at the cursor for engagement
await guide.lookAtCursor();

// Set up click interaction
const events = guide.events();
events.on('pointer-click', async () => {
  await gpt.chat('Hello! I noticed you looking at me. How can I help?', {
    vrm: guide.entity,
    speaker: 1
  });
});
```

## Related Methods

- [`lookAtTarget()`](look-at-target.md) - Look at a specific entity
- [`unlook()`](unlook.md) - Disable look-at functionality
- [`events()`](events.md) - Handle character interaction events

## Notes

- Cursor tracking provides smooth, natural-looking eye movement
- The effect works across multiple monitors and display configurations
- Character head movement is subtle and follows realistic constraints
- Performance impact is minimal as tracking is optimized for real-time use