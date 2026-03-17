---
title: "name"
sidebar_position: 14
---

# name

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.name()` returns the VRM model name of this character.

```typescript
const character = await Vrm.findByName("MyAvatar");
const name = await character.name();
console.log(name); // "MyAvatar"
```

The name corresponds to the VRM model's internal name, which is also the identifier used by [`Vrm.findByName`](./findByName) and [`Vrm.waitLoadByName`](./waitLoadByName).
