---
title: "setExpressions"
sidebar_position: 28
---

# setExpressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setExpressions(weights)` replaces **all** current expression overrides. Any expression not included in the record returns to VRMA animation control.

```typescript
const character = await Vrm.findByName("MyAvatar");

// Override happy and blink -- all other expressions revert to animation
await character.setExpressions({ happy: 1.0, blink: 0.5 });
```

:::tip
Use `setExpressions` when you want full control over which expressions are overridden. Use [`modifyExpressions`](./modifyExpressions) when you want to layer changes without disturbing other overrides.
:::

## Example: Emotional Reaction Sequence

```typescript
const character = await Vrm.findByName("MyAvatar");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// Surprised reaction
await character.setExpressions({ surprised: 1.0 });
await sleep(1000);

// Transition to happy
await character.setExpressions({ happy: 1.0 });
await sleep(2000);

// Return to animation control
await character.clearExpressions();
```
