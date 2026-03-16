---
title: "setPersona"
sidebar_position: 39
---

# setPersona

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setPersona(persona)` replaces the character's entire persona.

```typescript
await character.setPersona({
  profile: "A cheerful virtual assistant who loves to help with coding.",
  personality: "Friendly, curious, and always enthusiastic about learning new things.",
  ocean: {
    openness: 0.9,
    conscientiousness: 0.7,
    extraversion: 0.8,
    agreeableness: 0.85,
    neuroticism: 0.2,
  },
  metadata: {
    favoriteColor: "blue",
    skills: ["typescript", "rust"],
  },
});
```

## Setting Persona at Spawn

You can also set the initial persona when spawning a character:

```typescript
const character = await Vrm.spawn("my-mod:character", {
  persona: {
    profile: "A quiet observer who watches the screen.",
    ocean: { extraversion: 0.2, neuroticism: 0.1 },
    metadata: {},
  },
});
```

## Listening for Changes

Subscribe to persona updates with the events system:

```typescript
const eventSource = character.events();

eventSource.on("persona-change", (e) => {
  console.log("Persona updated:");
  console.log("  Profile:", e.persona.profile);
  console.log("  Openness:", e.persona.ocean.openness);
});
```

See [`Persona`](./types) and [`Ocean`](./types) for the full type definitions.
