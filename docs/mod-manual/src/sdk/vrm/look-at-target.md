# lookAtTarget()

Configure a VRM character to look at a specific entity target.

## Syntax

```typescript
await vrm.lookAtTarget(target: number): Promise<void>
```

## Description

The `lookAtTarget()` method configures the VRM character to continuously look at a specified entity in the 3D world. This creates focused attention behaviors where characters can maintain eye contact with other characters, objects, or interactive elements.

The character will automatically adjust their head and eye position to maintain visual focus on the target entity, even as either the character or target moves. This behavior remains active until disabled with `unlook()` or overridden by another look-at command.

## Parameters

- `target` (`number`) - The entity ID of the target to look at

## Return Value

A `Promise<void>` that resolves when the target tracking has been successfully enabled.

## Example

```typescript
import { Vrm } from '@desktop-homunculus/sdk';

// Find two VRM characters
const speaker = await Vrm.findByName('Speaker');
const listener = await Vrm.findByName('Listener');

// Make the listener look at the speaker
await listener.lookAtTarget(speaker.entity);

// Now the listener will maintain eye contact with the speaker
// Even if either character moves, the gaze will be maintained

// Start a conversation with natural eye contact
await gpt.chat('Hello there! Nice to meet you.', {
  vrm: speaker.entity,
  speaker: 1
});
```

## Advanced Usage

```typescript
// Create a character that tracks another character's hand
const performer = await Vrm.findByName('Performer');
const audience = await Vrm.findByName('Audience');

// Find the performer's right hand bone
const rightHand = await performer.findBoneEntity('rightHand');

// Make the audience look at the performer's hand
await audience.lookAtTarget(rightHand);

// Now play an animation where the performer waves
const waveAnimation = await performer.vrma('wave-animation');
await waveAnimation.play();

// The audience will track the hand movement naturally
```

## Interactive Scene Example

```typescript
// Create a teaching scenario with multiple characters
async function createClassroom() {
  const teacher = await Vrm.spawn('teacher-character');
  const student1 = await Vrm.spawn('student-character');
  const student2 = await Vrm.spawn('student-character');
  
  // Position characters
  await entities.setTransform(teacher.entity, { translation: [0, 0, 3] });
  await entities.setTransform(student1.entity, { translation: [-2, 0, 1] });
  await entities.setTransform(student2.entity, { translation: [2, 0, 1] });
  
  // Students look at teacher
  await student1.lookAtTarget(teacher.entity);
  await student2.lookAtTarget(teacher.entity);
  
  // Teacher can address individual students
  async function addressStudent(student: Vrm, message: string) {
    await teacher.lookAtTarget(student.entity);
    await gpt.chat(message, { vrm: teacher.entity, speaker: 2 });
  }
  
  await addressStudent(student1, 'What do you think about this concept?');
}
```

## Related Methods

- [`lookAtCursor()`](look-at-cursor.md) - Look at the mouse cursor
- [`unlook()`](unlook.md) - Disable look-at functionality
- [`findBoneEntity()`](find-bone.md) - Get specific bone entities for targeting

## Notes

- Target tracking works with any entity in the scene, including other VRM characters, bones, or objects
- The system handles target movement automatically for smooth, natural-looking gaze
- Characters maintain realistic head and eye movement constraints
- Multiple characters can look at the same target simultaneously
- Performance is optimized for scenes with multiple look-at relationships