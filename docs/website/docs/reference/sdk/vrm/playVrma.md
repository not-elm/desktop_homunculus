---
title: "playVrma"
sidebar_position: 16
---

# playVrma

```typescript
import { Vrm, repeat } from "@hmcs/sdk";
```

`vrm.playVrma(options)` starts a VRMA animation on the character.

```typescript
const character = await Vrm.spawn("my-mod:character");

await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## Options

| Option              | Type         | Default    | Description                                                             |
| ------------------- | ------------ | ---------- | ----------------------------------------------------------------------- |
| `asset`             | `string`     | (required) | Asset ID of the VRMA animation                                          |
| `repeat`            | `VrmaRepeat` | —          | Repeat mode: `repeat.forever()`, `repeat.never()`, or `repeat.count(n)` |
| `transitionSecs`    | `number`     | —          | Crossfade duration in seconds for blending from the current animation   |
| `waitForCompletion` | `boolean`    | `false`    | If `true`, the call blocks until the animation finishes                 |
| `resetSpringBones`  | `boolean`    | `false`    | If `true`, resets spring bone velocities to prevent bouncing artifacts  |

## Crossfade Transitions

Use `transitionSecs` to smoothly blend from the current animation into the new one. Without it, animations switch instantly.

```typescript
// Smooth 0.5-second crossfade into idle
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## Waiting for Completion

Set `waitForCompletion: true` to block until a one-shot animation finishes. This is useful for sequencing animations.

```typescript
// Play a wave animation and wait for it to finish
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
  waitForCompletion: true,
});

// This line runs after the wave animation is done
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## Spring Bone Reset

When switching between animations with very different poses (e.g., standing to grabbed), spring bone physics (hair, clothing) can cause unwanted bouncing. Use `resetSpringBones: true` to reset velocities.

```typescript
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
  resetSpringBones: true,
});
```

## Built-in Animations

The `@hmcs/assets` MOD provides these default VRMA animations:

| Asset ID            | Description                       |
| ------------------- | --------------------------------- |
| `vrma:idle-maid`    | Standing idle animation (looping) |
| `vrma:grabbed`      | Picked-up/grabbed pose (looping)  |
| `vrma:idle-sitting` | Sitting idle animation (looping)  |
