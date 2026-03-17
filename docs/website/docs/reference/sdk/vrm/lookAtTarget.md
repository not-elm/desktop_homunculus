---
title: "lookAtTarget"
sidebar_position: 36
---

# lookAtTarget

```typescript
import { Vrm } from "@hmcs/sdk";
```

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

## Example: Characters Looking at Each Other

```typescript
const alice = await Vrm.findByName("Alice");
const bob = await Vrm.findByName("Bob");

const aliceHead = await alice.findBoneEntity("head");
const bobHead = await bob.findBoneEntity("head");

await alice.lookAtTarget(bobHead);
await bob.lookAtTarget(aliceHead);
```

Use [`unlook`](./unlook) to disable look-at behavior, or [`lookAtCursor`](./lookAtCursor) to follow the mouse cursor instead.
