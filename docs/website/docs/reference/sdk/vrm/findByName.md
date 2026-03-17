---
title: "Vrm.findByName"
sidebar_position: 3
---

# Vrm.findByName

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findByName(name)` returns a `Vrm` instance for a character that is already loaded. It throws an error if no character with that name exists.

```typescript
try {
  const character = await Vrm.findByName("MyAvatar");
  console.log("Found entity:", character.entity);
} catch (e) {
  console.log("Character not found");
}
```

:::tip
Use `findByName` when you expect the character to already exist. Use [`Vrm.waitLoadByName`](./waitLoadByName) when your MOD starts before the character's MOD has finished spawning.
:::
