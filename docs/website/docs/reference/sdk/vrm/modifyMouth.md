---
title: "modifyMouth"
sidebar_position: 31
---

# modifyMouth

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.modifyMouth(weights)` sets mouth-specific expressions for lip-sync. Unspecified mouth expressions are reset to 0.0, but non-mouth overrides (like `happy` or `blink`) are preserved.

```typescript
// Set mouth shape for "ah" sound
await character.modifyMouth({ aa: 0.8 });

// Change to "oh" sound -- aa resets to 0, other overrides stay
await character.modifyMouth({ oh: 1.0 });

// Close mouth -- all mouth expressions reset to 0
await character.modifyMouth({});
```

This separation lets you control lip-sync independently from emotional expressions.
