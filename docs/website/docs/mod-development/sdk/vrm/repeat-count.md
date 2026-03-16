---
title: "repeat.count"
sidebar_position: 26
---

# repeat.count

```typescript
import { repeat } from "@hmcs/sdk";
```

`repeat.count(n)` returns a `VrmaRepeat` value that plays the animation exactly `n` times, then stops.

```typescript
await character.playVrma({
  asset: "my-mod:nod",
  repeat: repeat.count(3),
});
```

:::warning
`repeat.count(n)` requires a positive integer. Passing 0, negative numbers, or non-integers throws a `RangeError`.
:::

See also [`repeat.forever`](./repeat-forever) and [`repeat.never`](./repeat-never).
