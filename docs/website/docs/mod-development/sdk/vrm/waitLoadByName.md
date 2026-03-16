---
title: "Vrm.waitLoadByName"
sidebar_position: 4
---

# Vrm.waitLoadByName

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.waitLoadByName(name)` blocks until a character with the given name finishes loading, then returns a `Vrm` instance. Use this when your MOD starts before the character's MOD has finished spawning.

```typescript
// This will wait until "MyAvatar" is fully loaded
const character = await Vrm.waitLoadByName("MyAvatar");
```

:::tip
Use `waitLoadByName` in MOD services when you depend on a character spawned by another MOD. Use [`Vrm.findByName`](./findByName) when you expect the character to already exist.
:::
