---
title: "Look At"
sidebar_position: 8
---

# Look At

Control where a VRM character's eyes are looking. Characters can follow the mouse cursor, track a specific entity, or have look-at behavior disabled entirely.

## Import

```typescript
import { Vrm } from "@hmcs/sdk";
```

## Follow Cursor

`vrm.lookAtCursor()` makes the character's eyes follow the mouse cursor across the screen.

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.lookAtCursor();
```

## Look at Entity

`vrm.lookAtTarget(entity)` makes the character look at a specific entity by its ID. This is useful for making characters look at each other or at specific objects in the scene.

```typescript
const character = await Vrm.findByName("MyAvatar");
const other = await Vrm.findByName("OtherCharacter");

// Make MyAvatar look at OtherCharacter's head
const headEntity = await other.findBoneEntity("head");
await character.lookAtTarget(headEntity);
```

You can also use the entity ID of any Bevy ECS entity, not just VRM bones:

```typescript
// Look at another VRM's root entity
await character.lookAtTarget(other.entity);
```

## Disable Look-At

`vrm.unlook()` turns off look-at behavior entirely. The character's eyes return to their default animation-driven state.

```typescript
await character.unlook();
```

## Common Pattern: State-Driven Look-At

A typical pattern is to enable cursor tracking when the character is idle and disable it during drag or other interactions:

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// Start tracking the cursor
await character.lookAtCursor();

character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    // Resume cursor tracking after a short delay
    // (delay prevents jitter during animation transitions)
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    // Stop tracking while being dragged
    await character.unlook();
  } else if (e.state === "sitting") {
    // Continue tracking while sitting
    await sleep(500);
    await character.lookAtCursor();
  }
});
```

## Example: Characters Looking at Each Other

```typescript
const alice = await Vrm.findByName("Alice");
const bob = await Vrm.findByName("Bob");

// Get head bone entities
const aliceHead = await alice.findBoneEntity("head");
const bobHead = await bob.findBoneEntity("head");

// Make them look at each other
await alice.lookAtTarget(bobHead);
await bob.lookAtTarget(aliceHead);
```

## Look-At State

The current look-at state is included in the `VrmSnapshot` returned by `Vrm.findAllDetailed()`:

```typescript
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  if (s.lookAt === null) {
    console.log(`${s.name}: look-at disabled`);
  } else if (s.lookAt.type === "cursor") {
    console.log(`${s.name}: following cursor`);
  } else if (s.lookAt.type === "target") {
    console.log(`${s.name}: looking at entity ${s.lookAt.entity}`);
  }
}
```

## Types

```typescript
type LookAtState =
  | { type: "cursor" }
  | { type: "target"; entity: number };
```

## Next Steps

- **[Spawn & Find](./spawn-and-find)** -- Find characters and bone entities for look-at targets.
- **[Events](./events)** -- React to state changes to toggle look-at behavior.
- **[VRM Overview](./)** -- Full API reference table.
