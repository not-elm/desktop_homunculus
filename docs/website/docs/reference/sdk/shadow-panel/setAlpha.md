---
sidebar_position: 3
---

# setAlpha

Sets the transparency level of the shadow panel.

`alpha` ranges from `0` (fully transparent / invisible) to `1` (fully opaque).

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `alpha` | `number` | The transparency value between 0 (invisible) and 1 (opaque) |

## Returns

`Promise<void>`

## Example

```typescript
import { shadowPanel } from "@hmcs/sdk";

// Dim the background
await shadowPanel.setAlpha(0.7);

// Remove the overlay
await shadowPanel.setAlpha(0);
```
