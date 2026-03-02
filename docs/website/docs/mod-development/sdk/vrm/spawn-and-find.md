---
title: "Spawn & Find"
sidebar_position: 2
---

# Spawn & Find

Create new VRM characters and locate existing ones. The `Vrm` class provides static methods for spawning, querying, and streaming VRM instances.

## Import

```typescript
import { Vrm } from "@hmcs/sdk";
```

## Spawning a Character

`Vrm.spawn(asset, options?)` creates a new VRM character from a MOD asset ID and returns a `Vrm` instance bound to the spawned entity.

```typescript
const character = await Vrm.spawn("my-mod:character");
```

### Spawn Options

Pass an options object to set the initial transform and persona:

```typescript
import { Vrm, type Persona } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character", {
  transform: {
    translation: [0, 0.5, 0],
    scale: [1.2, 1.2, 1.2],
    rotation: [0, 0, 0, 1],
  },
  persona: {
    profile: "A cheerful virtual assistant who loves to help.",
    ocean: { openness: 0.8, extraversion: 0.7 },
    metadata: {},
  },
});
```

The `transform` field accepts a partial `TransformArgs` -- you only need to specify the fields you want to override. Unspecified fields use engine defaults.

```typescript
// Only set position, keep default scale and rotation
const character = await Vrm.spawn("my-mod:character", {
  transform: { translation: [2, 0, 0] },
});
```

## Finding by Name

`Vrm.findByName(name)` returns a `Vrm` instance for a character that is already loaded. It throws an error if no character with that name exists.

```typescript
try {
  const character = await Vrm.findByName("MyAvatar");
  console.log("Found entity:", character.entity);
} catch (e) {
  console.log("Character not found");
}
```

### Waiting for Load

`Vrm.waitLoadByName(name)` blocks until a character with the given name finishes loading. Use this when your MOD starts before the character's MOD has finished spawning.

```typescript
// This will wait until "MyAvatar" is fully loaded
const character = await Vrm.waitLoadByName("MyAvatar");
```

:::tip
Use `waitLoadByName` in MOD startup scripts when you depend on a character spawned by another MOD. Use `findByName` when you expect the character to already exist.
:::

## Listing All Characters

```typescript
// Get Vrm instances for all loaded characters
const characters = await Vrm.findAll();
for (const vrm of characters) {
  const name = await vrm.name();
  console.log(`${name} (entity: ${vrm.entity})`);
}
```

### Entity IDs Only

If you only need entity IDs without wrapping them in `Vrm` instances:

```typescript
const entityIds = await Vrm.findAllEntities();
console.log(`Found ${entityIds.length} VRM entities`);
```

### Detailed Snapshots

`Vrm.findAllDetailed()` returns full runtime state for every loaded VRM -- including transform, expressions, animations, persona, and linked webviews.

```typescript
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  console.log(`${s.name}: state=${s.state}`);
  console.log(`  Position: (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
  console.log(`  Animations: ${s.animations.length} active`);
  console.log(`  Expressions: ${s.expressions.expressions.length} defined`);
}
```

## Streaming

`Vrm.stream(callback)` fires for every VRM that currently exists and for any VRM that is created in the future. It returns an `EventSource` that you can close when done.

```typescript
const es = Vrm.stream(async (vrm) => {
  const name = await vrm.name();
  console.log(`VRM appeared: ${name} (entity: ${vrm.entity})`);
});

// Later, stop streaming
es.close();
```

This is useful for MODs that need to react to any character appearing in the scene, regardless of which MOD spawned it.

## Despawning

Remove a character from the scene with `despawn()`:

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.despawn();
```

## Position

Query a character's position in both screen and world coordinates:

```typescript
const character = await Vrm.findByName("MyAvatar");
const pos = await character.position();

// Screen coordinates (multi-monitor global viewport)
if (pos.globalViewport) {
  console.log(`Screen: (${pos.globalViewport[0]}, ${pos.globalViewport[1]})`);
}

// Bevy world coordinates
console.log(`World: (${pos.world[0]}, ${pos.world[1]}, ${pos.world[2]})`);
```

## State

Characters have a state string (e.g., `"idle"`, `"drag"`, `"sitting"`) that other systems can read and write:

```typescript
const state = await character.state();
console.log("Current state:", state);

await character.setState("custom-state");
```

## Bones

Find the entity ID of a named bone for advanced operations like attaching objects or look-at targets:

```typescript
const headEntity = await character.findBoneEntity("head");
const leftHandEntity = await character.findBoneEntity("leftHand");
```

Available bone names: `hips`, `spine`, `chest`, `neck`, `head`, `leftShoulder`, `leftArm`, `leftForeArm`, `leftHand`, `rightShoulder`, `rightArm`, `rightForeArm`, `rightHand`, `leftUpLeg`, `leftLeg`, `leftFoot`, `rightUpLeg`, `rightLeg`, `rightFoot`.

## Types

```typescript
interface SpawnVrmOptions {
  transform?: TransformArgs;
  persona?: Persona;
}

interface PositionResponse {
  /** Global screen coordinates. Null if not visible. */
  globalViewport: [number, number] | null;
  /** Bevy world coordinates. */
  world: Vec3;
}

interface VrmSnapshot {
  entity: number;
  name: string;
  state: string;
  transform: Transform;
  globalViewport: [number, number] | null;
  expressions: ExpressionsResponse;
  animations: VrmaInfo[];
  lookAt: LookAtState | null;
  linkedWebviews: number[];
  persona: Persona;
}
```

## Next Steps

- **[Expressions](./expressions)** -- Control facial expressions and blend shapes.
- **[Animations](./animations)** -- Play VRMA animations with repeat and transition options.
- **[Events](./events)** -- Subscribe to pointer, drag, and state change events.
- **[VRM Overview](./)** -- Full API reference table for all VRM methods.
