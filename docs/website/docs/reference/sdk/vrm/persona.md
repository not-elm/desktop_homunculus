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
console.log("Metadata:", persona.metadata);
```

Returns a [`Persona`](./types#persona) object containing the profile description, personality text, and extension metadata. Use [`setPersona`](./setPersona) to update it.
