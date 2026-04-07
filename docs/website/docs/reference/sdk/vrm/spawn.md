---
title: "Vrm.spawn"
sidebar_position: 2
---

# Vrm.spawn

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.spawn(asset, options?)` creates a new VRM character from a MOD asset ID and returns a `Vrm` instance bound to the spawned entity.

```typescript
const character = await Vrm.spawn("my-mod:character");
```

## Options

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
    personality: "Curious and open-minded, speaks with enthusiasm",
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
