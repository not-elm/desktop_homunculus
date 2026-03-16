---
title: "position"
sidebar_position: 11
---

# position

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.position()` returns the character's position in both screen and world coordinates.

```typescript
const character = await Vrm.findByName("MyAvatar");
const pos = await character.position();

// Screen coordinates (multi-monitor global viewport)
if (pos.globalViewport) {
  console.log(`Screen: (${pos.globalViewport[0]}, ${pos.globalViewport[1]})`);
}

// Bevy world coordinates
console.log(`World: (${pos.world[0]}, ${pos.world[1]}, ${pos.world[2]})`);
```

Returns a [`PositionResponse`](./types). The `globalViewport` field is `null` if the character is not currently visible on screen.
