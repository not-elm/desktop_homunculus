---
title: "despawn"
sidebar_position: 10
---

# despawn

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.despawn()` はキャラクターをシーンから削除します。

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.despawn();
```
