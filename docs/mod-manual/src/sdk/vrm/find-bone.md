# Vrm.findBoneEntity()

Finds the entity ID of a specific bone within the VRM character's skeleton. This allows for precise manipulation of
individual bones for advanced character control and positioning.

## Parameters

- `bone` (Bones) - The name of the bone to find

## Returns

`Promise<number>` - The entity ID of the specified bone

## Available Bones

The VRM standard defines the following bone names:

### Core Bones

- **`hips`** - Root bone of the skeleton
- **`spine`** - Lower spine
- **`chest`** - Upper chest/torso
- **`neck`** - Neck connection
- **`head`** - Head bone

### Arm Bones

- **`leftShoulder`** / **`rightShoulder`** - Shoulder joints
- **`leftArm`** / **`rightArm`** - Upper arm bones
- **`leftForeArm`** / **`rightForeArm`** - Lower arm bones
- **`leftHand`** / **`rightHand`** - Hand bones

### Leg Bones

- **`leftUpLeg`** / **`rightUpLeg`** - Upper leg bones (thighs)
- **`leftLeg`** / **`rightLeg`** - Lower leg bones (shins)
- **`leftFoot`** / **`rightFoot`** - Foot bones

## Examples

### Basic Bone Access

```typescript
import {Vrm, entities} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::humanoid.vrm');

// Get head bone entity
const headEntity = await character.findBoneEntity('head');
const boneTransform = await entities.transform(headEntity);
console.log(`Head bone entity ID: ${headEntity}, transform: ${JSON.stringify(boneTransform)}`);

// Get hand bone entities
const leftHandEntity = await character.findBoneEntity('leftHand');
const rightHandEntity = await character.findBoneEntity('rightHand');

console.log(`Left hand: ${leftHandEntity}, Right hand: ${rightHandEntity}`);

```

## Common Use Cases

### Facial Animation

Manipulate head and neck bones for looking behaviors and expressions.

### Gesture Control

Control arm and hand bones for pointing, waving, and other gestures.

### Interactive Elements

Attach UI elements or effects to specific bones that move with the character.

### Physics Simulation

Apply constraints and forces to bones for realistic movement.

### Accessibility Features

Highlight or manipulate specific bones for educational or debugging purposes.

## Related Documentation

- **[Entity Management](../entities/index.md)** - Transform manipulation for bones
- **[Cameras](../cameras/index.md)** - Converting between coordinate systems
- **[VRM Character Management](index.md)** - Overall character system
- **[VRMA Animation System](../vrma/index.md)** - Animation integration with bone control