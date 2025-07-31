# VRM Character Management

The VRM Character Management system provides comprehensive control over 3D virtual characters in Desktop Homunculus.
VRM (Virtual Reality Model) is a 3D avatar file format that enables rich character interactions, animations, and
immersive experiences.

## Overview

VRM characters are the core interactive elements of Desktop Homunculus. They can be spawned, controlled, animated, and
serve as the foundation for AI-powered conversations and interactions. The VRM system supports:

- **Character Spawning**: Load VRM models from mod assets
- **Animation Control**: Play VRMA animations and manage character states
- **Real-Time Events**: Respond to mouse interactions, drag events, and state changes
- **AI Integration**: Text-to-speech capabilities with VoiceVox
- **Look-At System**: Configure characters to look at cursor or specific targets
- **Bone Manipulation**: Access individual bones for precise control
- **Multi-Character Support**: Manage multiple VRM instances simultaneously

## Core Concepts

### Character States

VRM characters have the following built-in states:

- **`idle`** - Default resting state (automatically assigned on spawn)
- **`sitting`** - Character is in a sitting position
- **`drag`** - Character is being dragged by the user (automatically set during drag operations)
- **Custom states** - You can define any string as a custom state for mod-specific behaviors

### Event System

Characters emit events for various interactions:

- **Pointer Events**: Click, press, release, hover
- **Drag Events**: Drag start, drag, drag end
- **State Events**: State change notifications

### Bones

VRM characters have a standardized bone structure:

- **Core Bones**: hips, spine, chest, neck, head
- **Arms**: leftShoulder, leftArm, leftForeArm, leftHand (and right variants)
- **Legs**: leftUpLeg, leftLeg, leftFoot (and right variants)

## Related Documentation

- **[VRMA Animation System](../vrma/index.md)** - Animation control and playback
- **[GPT Integration](../gpt/index.md)** - AI-powered conversations
- **[Entity Management](../entities/index.md)** - Transform and positioning control
- **[Preferences](../preferences/index.md)** - Character state persistence
- **[Effects System](../effects/index.md)** - Visual and audio effects