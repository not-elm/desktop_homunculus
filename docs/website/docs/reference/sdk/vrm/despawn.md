---
title: "despawn"
sidebar_position: 10
---

# despawn

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.despawn()` removes the character from the scene.

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.despawn();
```
