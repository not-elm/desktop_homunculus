---
title: "VRM"
sidebar_position: 1
---

# VRM Module

The `Vrm` class is the core module of `@hmcs/sdk`. It manages the full lifecycle of VRM 3D characters -- spawning, finding, animating, controlling expressions, handling pointer/drag events, lip-sync speech, and more.

```typescript
import { Vrm, repeat, VrmEventSource } from "@hmcs/sdk";
```

## Static Methods

| Method                              | Description                                                                              |
| ----------------------------------- | ---------------------------------------------------------------------------------------- |
| [`Vrm.spawn`](./spawn)              | Spawn a new VRM from a MOD asset ID. Returns a `Vrm` instance.                           |
| [`Vrm.findByName`](./findByName)    | Find a VRM by its model name. Throws if not found.                                       |
| [`Vrm.waitLoadByName`](./waitLoadByName) | Wait for a VRM to finish loading, then return it.                                  |
| [`Vrm.findAll`](./findAll)          | Get all loaded VRM instances as `Vrm[]`.                                                 |
| [`Vrm.findAllEntities`](./findAllEntities) | Get all loaded VRM entity IDs as `number[]`.                                     |
| [`Vrm.findAllDetailed`](./findAllDetailed) | Get detailed snapshots of all VRMs (state, transform, expressions, animations, persona). |
| [`Vrm.streamMetadata`](./streamMetadata) | Stream raw `VrmMetadata` for existing and future VRM instances.                    |
| [`Vrm.stream`](./stream)            | Stream existing and future VRM instances as `Vrm` objects.                               |

## Instance Methods

| Method                                          | Description                                                                              |
| ----------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [`despawn`](./despawn)                          | Remove this VRM from the scene.                                                          |
| [`position`](./position)                        | Get position in both screen (`globalViewport`) and world coordinates.                    |
| [`state`](./state)                              | Get the current state string (e.g., "idle", "drag", "sitting").                          |
| [`setState`](./setState)                        | Set the character's state.                                                               |
| [`name`](./name)                                | Get the VRM model name.                                                                  |
| [`findBoneEntity`](./findBoneEntity)            | Find the entity ID of a named bone (e.g., "head", "leftHand").                           |
| [`playVrma`](./playVrma)                        | Play a VRMA animation with repeat, transition, and completion options.                   |
| [`stopVrma`](./stopVrma)                        | Stop a specific VRMA animation by asset ID.                                              |
| [`listVrma`](./listVrma)                        | List all VRMA animations attached to this VRM.                                           |
| [`vrmaState`](./vrmaState)                      | Get the playback state of a specific animation (playing, speed, elapsed).                |
| [`setVrmaSpeed`](./setVrmaSpeed)                | Change the playback speed of an animation.                                               |
| [`springBones`](./springBones)                  | Get all spring bone chains (hair, clothing physics).                                     |
| [`springBone`](./springBone)                    | Get a specific spring bone chain by entity ID.                                           |
| [`setSpringBone`](./setSpringBone)              | Update spring bone physics properties (stiffness, drag, gravity).                        |
| [`expressions`](./expressions)                  | Get all expressions and their current weights.                                           |
| [`setExpressions`](./setExpressions)            | Set expression weights, replacing all previous overrides.                                |
| [`modifyExpressions`](./modifyExpressions)      | Partially update expression weights (other overrides remain).                            |
| [`clearExpressions`](./clearExpressions)        | Clear all expression overrides, returning control to VRMA animation.                     |
| [`modifyMouth`](./modifyMouth)                  | Set mouth expressions for lip-sync (non-mouth overrides are preserved).                  |
| [`lookAtCursor`](./lookAtCursor)                | Make the character's eyes follow the mouse cursor.                                       |
| [`lookAtTarget`](./lookAtTarget)                | Make the character look at a specific entity.                                            |
| [`unlook`](./unlook)                            | Disable the look-at behavior.                                                            |
| [`persona`](./persona)                          | Get the character's persona (profile, personality, OCEAN traits, metadata).              |
| [`setPersona`](./setPersona)                    | Set the character's persona data.                                                        |
| [`speakWithTimeline`](./speakWithTimeline)      | Play WAV audio with frame-synchronized expression keyframes for lip-sync.                |
| [`events`](./events)                            | Open a `VrmEventSource` for real-time event streaming.                                   |

## repeat namespace

| Function                          | Description                                        |
| --------------------------------- | -------------------------------------------------- |
| [`repeat.forever`](./repeat-forever) | Loop the animation indefinitely.                |
| [`repeat.never`](./repeat-never)  | Play the animation exactly once.                   |
| [`repeat.count`](./repeat-count)  | Play the animation a fixed number of times.        |

## VrmEventSource

| Method                                        | Description                                       |
| --------------------------------------------- | ------------------------------------------------- |
| [`VrmEventSource.on`](./VrmEventSource-on)    | Register an event listener.                       |
| [`VrmEventSource.close`](./VrmEventSource-close) | Close the SSE connection.                      |
