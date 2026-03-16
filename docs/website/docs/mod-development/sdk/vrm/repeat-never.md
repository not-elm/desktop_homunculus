---
title: "repeat.never"
sidebar_position: 25
---

# repeat.never

```typescript
import { repeat } from "@hmcs/sdk";
```

`repeat.never()` returns a `VrmaRepeat` value that plays the animation exactly once, then stops.

```typescript
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
  waitForCompletion: true,
});
```

Combine with `waitForCompletion: true` in [`playVrma`](./playVrma) to block until the one-shot animation finishes. See also [`repeat.forever`](./repeat-forever) and [`repeat.count`](./repeat-count).
