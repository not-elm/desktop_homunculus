---
title: "repeat.forever"
sidebar_position: 24
---

# repeat.forever

```typescript
import { repeat } from "@hmcs/sdk";
```

`repeat.forever()` returns a `VrmaRepeat` value that loops the animation indefinitely.

```typescript
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

Use with [`playVrma`](./playVrma) to create looping animations. See also [`repeat.never`](./repeat-never) for one-shot animations and [`repeat.count`](./repeat-count) for a fixed number of loops.
