---
title: "state"
sidebar_position: 12
---

# state

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.state()` returns the character's current state string (e.g., `"idle"`, `"drag"`, `"sitting"`).

```typescript
const character = await Vrm.findByName("MyAvatar");
const state = await character.state();
console.log("Current state:", state);
```

Use [`setState`](./setState) to change the state, or subscribe to [`events()`](./events) to react when the state changes.
