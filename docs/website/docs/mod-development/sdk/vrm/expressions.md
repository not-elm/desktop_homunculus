---
title: "Expressions"
sidebar_position: 3
---

# Expressions

Control facial expressions on VRM characters. Expressions are named blend shapes -- `happy`, `sad`, `angry`, `blink`, and mouth shapes like `aa`, `ih`, `oh` -- with weight values from 0.0 to 1.0.

## Import

```typescript
import { Vrm } from "@hmcs/sdk";
```

## Setting Expressions

`setExpressions(weights)` replaces **all** current expression overrides. Any expression not included in the record returns to VRMA animation control.

```typescript
const character = await Vrm.findByName("MyAvatar");

// Override happy and blink -- all other expressions revert to animation
await character.setExpressions({ happy: 1.0, blink: 0.5 });
```

## Modifying Expressions

`modifyExpressions(weights)` performs a **partial update** -- only the specified expressions change. Existing overrides not mentioned in the call remain intact.

```typescript
// First, set happy
await character.modifyExpressions({ happy: 1.0 });

// Later, add a blink without removing the happy override
await character.modifyExpressions({ blink: 1.0 });
// Result: happy=1.0, blink=1.0
```

:::tip
Use `setExpressions` when you want full control over which expressions are overridden. Use `modifyExpressions` when you want to layer changes without disturbing other overrides.
:::

## Clearing Expressions

`clearExpressions()` removes all expression overrides, returning full control to VRMA animations.

```typescript
await character.clearExpressions();
```

This is useful when transitioning from a scripted facial sequence back to animation-driven expressions.

## Mouth Expressions

`modifyMouth(weights)` sets mouth-specific expressions for lip-sync. Unspecified mouth expressions are reset to 0.0, but non-mouth overrides (like `happy` or `blink`) are preserved.

```typescript
// Set mouth shape for "ah" sound
await character.modifyMouth({ aa: 0.8 });

// Change to "oh" sound -- aa resets to 0, other overrides stay
await character.modifyMouth({ oh: 1.0 });

// Close mouth -- all mouth expressions reset to 0
await character.modifyMouth({});
```

This separation lets you control lip-sync independently from emotional expressions.

## Querying Expressions

`expressions()` returns the current state of all expressions, including their weights and metadata.

```typescript
const { expressions } = await character.expressions();
for (const expr of expressions) {
  if (expr.weight > 0) {
    console.log(`${expr.name}: weight=${expr.weight}, binary=${expr.isBinary}`);
  }
}
```

Each `ExpressionInfo` includes:
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

## Types

```typescript
interface ExpressionInfo {
  name: string;
  weight: number;
  isBinary: boolean;
  overrideBlink: OverrideType;
  overrideLookAt: OverrideType;
  overrideMouth: OverrideType;
}

interface ExpressionsResponse {
  expressions: ExpressionInfo[];
}

type OverrideType = "none" | "blend" | "block";
```

## Next Steps

- **[Speech Timeline](./speech-timeline)** -- Use mouth expressions with audio for lip-sync.
- **[Animations](./animations)** -- Combine expressions with VRMA animations.
- **[VRM Overview](./)** -- Full API reference table.
