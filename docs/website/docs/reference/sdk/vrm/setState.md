---
title: "setState"
sidebar_position: 13
---

# setState

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setState(state)` sets the character's state string.

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.setState("custom-state");
```

The state is a free-form string. Built-in states used by the engine are `"idle"`, `"drag"`, and `"sitting"`. Setting the state triggers a `state-change` event on any open [`VrmEventSource`](./events).
