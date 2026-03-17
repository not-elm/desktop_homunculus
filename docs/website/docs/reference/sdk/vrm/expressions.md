---
title: "expressions"
sidebar_position: 27
---

# expressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.expressions()` returns the current state of all expressions, including their weights and metadata.

```typescript
const { expressions } = await character.expressions();
for (const expr of expressions) {
  if (expr.weight > 0) {
    console.log(`${expr.name}: weight=${expr.weight}, binary=${expr.isBinary}`);
  }
}
```

Each [`ExpressionInfo`](./types#expressioninfo) includes:
- `name` -- Expression name (e.g., `"happy"`, `"aa"`)
- `weight` -- Current weight value (0.0--1.0)
- `isBinary` -- Whether the expression snaps to 0 or 1 (no in-between)
- `overrideBlink` -- How this expression interacts with blink (`"none"`, `"blend"`, or `"block"`)
- `overrideLookAt` -- How this expression interacts with look-at
- `overrideMouth` -- How this expression interacts with mouth expressions

## Available Expressions

Standard VRM expressions available on most models:

| Category | Expressions |
|---|---|
| **Emotion** | `happy`, `angry`, `sad`, `relaxed`, `surprised`, `neutral` |
| **Mouth** | `aa`, `ih`, `ou`, `ee`, `oh` |
| **Eyes** | `blink`, `blinkLeft`, `blinkRight` |
| **Gaze** | `lookUp`, `lookDown`, `lookLeft`, `lookRight` |

:::note
Available expressions depend on the VRM model. Not all models include every expression. Use `expressions()` to query which expressions a specific model supports.
:::
