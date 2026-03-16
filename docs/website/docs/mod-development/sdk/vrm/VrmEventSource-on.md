---
title: "VrmEventSource.on"
sidebar_position: 33
---

# VrmEventSource.on

```typescript
import { Vrm } from "@hmcs/sdk";
```

`eventSource.on(event, callback)` registers an event listener on a [`VrmEventSource`](./types#vrmeventsource). Callbacks can be synchronous or async.

```typescript
const character = await Vrm.findByName("MyAvatar");
const eventSource = character.events();

eventSource.on("state-change", (e) => {
  console.log("New state:", e.state);
});

eventSource.on("pointer-click", async (e) => {
  console.log(`Clicked at (${e.globalViewport[0]}, ${e.globalViewport[1]})`);
  console.log(`Button: ${e.button}`);
});
```

## Example: State Machine

A common pattern is using events to drive animation and behavior based on character state:

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

const animOption = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

await character.playVrma({ asset: "vrma:idle-maid", ...animOption });

character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await character.playVrma({ asset: "vrma:idle-maid", ...animOption });
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    await character.unlook();
    await character.playVrma({
      asset: "vrma:grabbed",
      ...animOption,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    await character.playVrma({ asset: "vrma:idle-sitting", ...animOption });
    await sleep(500);
    await character.lookAtCursor();
  }
});
```

See [`events`](./events) for the full list of available event types and their payloads.
