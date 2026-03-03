---
title: "VRM"
sidebar_position: 1
---

# VRM Module

The `Vrm` class is the core module of `@hmcs/sdk`. It manages the full lifecycle of VRM 3D characters -- spawning, finding, animating, controlling expressions, handling pointer/drag events, lip-sync speech, and more.

```typescript
import { Vrm } from "@hmcs/sdk";
```

## Spawning a Character

Use `Vrm.spawn()` to create a new VRM character from a MOD asset. By convention, asset IDs use the format `"mod-name:asset-name"`.

```typescript
import { Vrm } from "@hmcs/sdk";

// Basic spawn
const character = await Vrm.spawn("my-mod:character");

// Spawn with initial transform and persona
const character = await Vrm.spawn("my-mod:character", {
  transform: {
    translation: [0, 0, 0],
    scale: [1, 1, 1],
  },
  persona: {
    profile: "A cheerful virtual assistant",
    ocean: { openness: 0.8, extraversion: 0.7 },
    metadata: {},
  },
});
```

`Vrm.spawn()` returns a `Vrm` instance bound to the spawned entity. All subsequent operations use this instance.

## Finding Existing Characters

If a character has already been spawned (e.g., by another MOD or a previous session), you can find it by name or list all instances.

```typescript
// Find by VRM model name
const character = await Vrm.findByName("MyAvatar");

// Wait for a character to be loaded (blocks until ready)
const character = await Vrm.waitLoadByName("MyAvatar");

// Get all loaded VRM instances
const allCharacters = await Vrm.findAll();

// Get detailed snapshots of all VRMs (state, transform, expressions, animations)
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  console.log(`${s.name}: ${s.state} at (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
}
```

## Playing Animations

Play VRMA animations on a character with `playVrma()`. Animations are referenced by asset ID. The built-in `@hmcs/assets` MOD provides default animations: `vrma:idle-maid`, `vrma:grabbed`, and `vrma:idle-sitting`.

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");

// Play a looping idle animation with a 0.5s crossfade
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// Play a one-shot animation (plays once, then stops)
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
});

// Play an animation N times
await character.playVrma({
  asset: "my-mod:nod",
  repeat: repeat.count(3),
});

// Wait for a one-shot animation to finish before continuing
await character.playVrma({
  asset: "my-mod:dance",
  repeat: repeat.never(),
  waitForCompletion: true,
});

// Reset spring bones during transition to prevent hair/clothing bouncing
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
  resetSpringBones: true,
});
```

You can also query animation state, stop animations, and control playback speed:

```typescript
// Get all active animations
const animations = await character.listVrma();

// Check a specific animation's state
const state = await character.vrmaState("vrma:idle-maid");
console.log(`Playing: ${state.playing}, Elapsed: ${state.elapsedSecs}s`);

// Change playback speed
await character.setVrmaSpeed("vrma:idle-maid", 1.5);

// Stop an animation
await character.stopVrma("vrma:idle-maid");
```

## Character Events

Subscribe to real-time events from a character using `events()`. This opens a Server-Sent Events (SSE) connection that streams events as they happen.

```typescript
const character = await Vrm.spawn("my-mod:character");
const eventSource = character.events();

// State changes: idle, drag, sitting, etc.
eventSource.on("state-change", async (e) => {
  console.log("New state:", e.state);
});

// Pointer interactions
eventSource.on("pointer-click", (e) => {
  console.log(`Clicked at (${e.globalViewport[0]}, ${e.globalViewport[1]}), button: ${e.button}`);
});

// Drag events
eventSource.on("drag-start", (e) => console.log("Drag started"));
eventSource.on("drag", (e) => console.log(`Dragging, delta: ${e.delta}`));
eventSource.on("drag-end", (e) => console.log("Drag ended"));

// Hover events
eventSource.on("pointer-over", (e) => console.log("Mouse entered character"));
eventSource.on("pointer-out", (e) => console.log("Mouse left character"));

// Animation events
eventSource.on("vrma-play", (e) => console.log("Animation started:", e.state));
eventSource.on("vrma-finish", (e) => console.log("Animation finished:", e.state));

// Persona changes
eventSource.on("persona-change", (e) => {
  console.log("Persona updated:", e.persona.profile);
});

// Close the event stream when done
eventSource.close();
```

The `VrmEventSource` implements `Disposable`, so you can use it with `using` in TypeScript 5.2+:

```typescript
using eventSource = character.events();
eventSource.on("state-change", (e) => { /* ... */ });
```

### Available Events

| Event               | Payload                      | Description                                         |
| ------------------- | ---------------------------- | --------------------------------------------------- |
| `state-change`      | `{ state: string }`          | Character state changed (idle, drag, sitting, etc.) |
| `expression-change` | `{ state: string }`          | Expression changed                                  |
| `vrma-play`         | `{ state: string }`          | VRMA animation started playing                      |
| `vrma-finish`       | `{ state: string }`          | VRMA animation finished                             |
| `pointer-click`     | `{ globalViewport, button }` | Character was clicked                               |
| `pointer-press`     | `{ globalViewport, button }` | Mouse button pressed on character                   |
| `pointer-release`   | `{ globalViewport, button }` | Mouse button released on character                  |
| `pointer-over`      | `{ globalViewport }`         | Mouse entered character area                        |
| `pointer-out`       | `{ globalViewport }`         | Mouse left character area                           |
| `pointer-move`      | `{ globalViewport }`         | Mouse moved within character area                   |
| `pointer-cancel`    | `{ globalViewport }`         | Pointer interaction cancelled                       |
| `drag-start`        | `{ globalViewport }`         | Drag started                                        |
| `drag`              | `{ globalViewport, delta }`  | Dragging in progress (includes cursor delta)        |
| `drag-end`          | `{ globalViewport }`         | Drag ended                                          |
| `persona-change`    | `{ persona }`                | Persona data was updated                            |

## Key APIs

### Lifecycle

| Method                       | Description                                                                              |
| ---------------------------- | ---------------------------------------------------------------------------------------- |
| `Vrm.spawn(asset, options?)` | Spawn a new VRM from a MOD asset ID. Returns a `Vrm` instance.                           |
| `Vrm.findByName(name)`       | Find a VRM by its model name. Throws if not found.                                       |
| `Vrm.waitLoadByName(name)`   | Wait for a VRM to finish loading, then return it.                                        |
| `Vrm.findAll()`              | Get all loaded VRM instances as `Vrm[]`.                                                 |
| `Vrm.findAllEntities()`      | Get all loaded VRM entity IDs as `number[]`.                                             |
| `Vrm.findAllDetailed()`      | Get detailed snapshots of all VRMs (state, transform, expressions, animations, persona). |
| `Vrm.stream(callback)`       | Stream existing and future VRM instances as they are created.                            |
| `vrm.despawn()`              | Remove this VRM from the scene.                                                          |

### Animation

| Method                           | Description                                                               |
| -------------------------------- | ------------------------------------------------------------------------- |
| `vrm.playVrma(options)`          | Play a VRMA animation with repeat, transition, and completion options.    |
| `vrm.stopVrma(asset)`            | Stop a specific VRMA animation by asset ID.                               |
| `vrm.listVrma()`                 | List all VRMA animations attached to this VRM.                            |
| `vrm.vrmaState(asset)`           | Get the playback state of a specific animation (playing, speed, elapsed). |
| `vrm.setVrmaSpeed(asset, speed)` | Change the playback speed of an animation.                                |

### Expressions

| Method                           | Description                                                             |
| -------------------------------- | ----------------------------------------------------------------------- |
| `vrm.expressions()`              | Get all expressions and their current weights.                          |
| `vrm.setExpressions(weights)`    | Set expression weights, replacing all previous overrides.               |
| `vrm.modifyExpressions(weights)` | Partially update expression weights (other overrides remain).           |
| `vrm.clearExpressions()`         | Clear all expression overrides, returning control to VRMA animation.    |
| `vrm.modifyMouth(weights)`       | Set mouth expressions for lip-sync (non-mouth overrides are preserved). |

### Look-At

| Method                     | Description                                        |
| -------------------------- | -------------------------------------------------- |
| `vrm.lookAtCursor()`       | Make the character's eyes follow the mouse cursor. |
| `vrm.lookAtTarget(entity)` | Make the character look at a specific entity.      |
| `vrm.unlook()`             | Disable the look-at behavior.                      |

### Speech

| Method                                              | Description                                                               |
| --------------------------------------------------- | ------------------------------------------------------------------------- |
| `vrm.speakWithTimeline(audio, keyframes, options?)` | Play WAV audio with frame-synchronized expression keyframes for lip-sync. |

### State and Position

| Method                | Description                                                           |
| --------------------- | --------------------------------------------------------------------- |
| `vrm.state()`         | Get the current state string (e.g., "idle", "drag", "sitting").       |
| `vrm.setState(state)` | Set the character's state.                                            |
| `vrm.position()`      | Get position in both screen (`globalViewport`) and world coordinates. |
| `vrm.name()`          | Get the VRM model name.                                               |

### Persona

| Method                    | Description                                                                 |
| ------------------------- | --------------------------------------------------------------------------- |
| `vrm.persona()`           | Get the character's persona (profile, personality, OCEAN traits, metadata). |
| `vrm.setPersona(persona)` | Set the character's persona data.                                           |

### Physics

| Method                              | Description                                                       |
| ----------------------------------- | ----------------------------------------------------------------- |
| `vrm.springBones()`                 | Get all spring bone chains (hair, clothing physics).              |
| `vrm.springBone(chainId)`           | Get a specific spring bone chain by entity ID.                    |
| `vrm.setSpringBone(chainId, props)` | Update spring bone physics properties (stiffness, drag, gravity). |

### Events

| Method                     | Description                                                    |
| -------------------------- | -------------------------------------------------------------- |
| `vrm.events()`             | Open a `VrmEventSource` for real-time event streaming.         |
| `vrm.findBoneEntity(bone)` | Find the entity ID of a named bone (e.g., "head", "leftHand"). |

## Complete Example

The following is the full service from the `@hmcs/elmer` MOD. It demonstrates spawning a character, playing animations based on state, and cursor tracking.

```typescript
import { type TransformArgs, Vrm, preferences, repeat } from "@hmcs/sdk";

// Load the character's last known position from preferences
const transform = await preferences.load<TransformArgs>("transform::elmer:vrm");

// Spawn the Elmer character using its VRM asset
const elmer = await Vrm.spawn("elmer:vrm", {
  transform,
});

// Helper for async delays
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

// Shared animation options: loop forever with a 0.5s crossfade
const option = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

// Start with the idle animation
await elmer.playVrma({
  asset: "vrma:idle-maid",
  ...option,
});

// React to character state changes
elmer.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    // When idle: play idle animation, then track the cursor
    await elmer.playVrma({
      asset: "vrma:idle-maid",
      ...option,
    });
    await sleep(500);
    await elmer.lookAtCursor();
  } else if (e.state === "drag") {
    // When being dragged: stop cursor tracking, play grabbed animation
    await elmer.unlook();
    await elmer.playVrma({
      asset: "vrma:grabbed",
      ...option,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    // When sitting on a window edge: play sitting animation
    await elmer.playVrma({
      asset: "vrma:idle-sitting",
      ...option,
    });
    await sleep(500);
    await elmer.lookAtCursor();
  }
});
```

## Next Steps

- **[SDK Overview](../)** -- Full list of all SDK modules and their descriptions.
