---
title: "Animations"
sidebar_position: 4
---

# Animations

Play VRMA animations on VRM characters. VRMA is the animation format for VRM models, supporting skeletal animation with blend shapes. The built-in `@hmcs/assets` MOD provides default animations.

## Import

```typescript
import { Vrm, repeat } from "@hmcs/sdk";
```

## Playing Animations

`vrm.playVrma(options)` starts a VRMA animation on the character.

```typescript
const character = await Vrm.spawn("my-mod:character");

await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

### Options

| Option              | Type         | Default    | Description                                                             |
| ------------------- | ------------ | ---------- | ----------------------------------------------------------------------- |
| `asset`             | `string`     | (required) | Asset ID of the VRMA animation                                          |
| `repeat`            | `VrmaRepeat` | —          | Repeat mode: `repeat.forever()`, `repeat.never()`, or `repeat.count(n)` |
| `transitionSecs`    | `number`     | —          | Crossfade duration in seconds for blending from the current animation   |
| `waitForCompletion` | `boolean`    | `false`    | If `true`, the call blocks until the animation finishes                 |
| `resetSpringBones`  | `boolean`    | `false`    | If `true`, resets spring bone velocities to prevent bouncing artifacts  |

## Repeat Modes

The `repeat` namespace provides helpers for building repeat configurations:

```typescript
import { repeat } from "@hmcs/sdk";

// Loop the animation indefinitely
repeat.forever();

// Play exactly once, then stop
repeat.never();

// Play exactly 3 times, then stop
repeat.count(3);
```

:::warning
`repeat.count(n)` requires a positive integer. Passing 0, negative numbers, or non-integers throws a `RangeError`.
:::

## Crossfade Transitions

Use `transitionSecs` to smoothly blend from the current animation into the new one. Without it, animations switch instantly.

```typescript
// Smooth 0.5-second crossfade into idle
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// Instant switch to grabbed animation
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
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

## Querying Animation State

### List Active Animations

```typescript
const animations = await character.listVrma();
for (const anim of animations) {
  console.log(`${anim.name}: entity=${anim.entity}, playing=${anim.playing}`);
}
```

### Check Specific Animation

```typescript
const state = await character.vrmaState("vrma:idle-maid");
console.log(`Playing: ${state.playing}`);
console.log(`Speed: ${state.speed}x`);
console.log(`Elapsed: ${state.elapsedSecs}s`);
console.log(`Repeat: ${state.repeat}`);
```

### Change Playback Speed

```typescript
// Slow motion
await character.setVrmaSpeed("vrma:idle-maid", 0.5);

// Double speed
await character.setVrmaSpeed("vrma:idle-maid", 2.0);

// Normal speed
await character.setVrmaSpeed("vrma:idle-maid", 1.0);
```

### Stop an Animation

```typescript
await character.stopVrma("vrma:idle-maid");
```

## Spring Bones

VRM models use spring bones for physics simulation on hair, clothing, and accessories. You can query and customize spring bone properties.

### Query All Chains

```typescript
const { chains } = await character.springBones();
for (const chain of chains) {
  console.log(`Chain ${chain.entity}: ${chain.joints.length} joints`);
  console.log(`  Stiffness: ${chain.props.stiffness}`);
  console.log(`  Drag: ${chain.props.dragForce}`);
}
```

### Modify Physics

```typescript
const { chains } = await character.springBones();
const hairChain = chains[0];

// Make hair bouncier
await character.setSpringBone(hairChain.entity, {
  stiffness: 0.5,
  dragForce: 0.2,
});

// Change gravity direction
await character.setSpringBone(hairChain.entity, {
  gravityPower: 1.0,
  gravityDir: [0, -1, 0],
});
```

## Built-in Animations

The `@hmcs/assets` MOD provides these default VRMA animations:

| Asset ID            | Description                       |
| ------------------- | --------------------------------- |
| `vrma:idle-maid`    | Standing idle animation (looping) |
| `vrma:grabbed`      | Picked-up/grabbed pose (looping)  |
| `vrma:idle-sitting` | Sitting idle animation (looping)  |

## Types

```typescript
interface VrmaPlayRequest {
  asset: string;
  transitionSecs?: number;
  repeat?: VrmaRepeat;
  waitForCompletion?: boolean;
  resetSpringBones?: boolean;
}

interface VrmaRepeat {
  type: "forever" | "never" | "count";
  count?: number;
}

interface VrmaState {
  playing: boolean;
  repeat: string;
  speed: number;
  elapsedSecs: number;
}

interface VrmaInfo {
  entity: number;
  name: string;
  playing: boolean;
}

interface SpringBoneProps {
  stiffness: number;
  dragForce: number;
  gravityPower: number;
  gravityDir: [number, number, number];
  hitRadius: number;
}

interface SpringBoneChain {
  entity: number;
  joints: string[];
  props: SpringBoneProps;
}
```

## Next Steps

- **[Expressions](./expressions)** -- Layer facial expressions on top of animations.
- **[Events](./events)** -- React to `vrma-play` and `vrma-finish` events.
- **[VRM Overview](./)** -- Full API reference table.
