---
title: "clearExpressions"
sidebar_position: 30
---

# clearExpressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.clearExpressions()` removes all expression overrides, returning full control to VRMA animations.

```typescript
await character.clearExpressions();
```

This is useful when transitioning from a scripted facial sequence back to animation-driven expressions.
