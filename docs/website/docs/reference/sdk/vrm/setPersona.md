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
    personality: "Calm and reserved, rarely initiates conversation.",
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
  console.log("  Personality:", e.persona.personality);
});
```

See [`Persona`](./types#persona) for the full type definition.
