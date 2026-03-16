---
title: "persona"
sidebar_position: 38
---

# persona

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.persona()` returns the character's current persona data.

```typescript
const character = await Vrm.findByName("MyAvatar");
const persona = await character.persona();

console.log("Profile:", persona.profile);
console.log("Personality:", persona.personality);
console.log("Openness:", persona.ocean.openness);
console.log("Metadata:", persona.metadata);
```

Returns a [`Persona`](./types) object containing the profile description, personality string, OCEAN trait values, and extension metadata. Use [`setPersona`](./setPersona) to update it.
