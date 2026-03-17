---
sidebar_position: 3
---

# setFps

Updates the rendering frame rate. Persists and applies immediately.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `fps` | `number` | The target frame rate in frames per second (minimum 1) |

## Returns

`Promise<void>`

## Example

```typescript
await settings.setFps(30);
```

```typescript
import { settings, shadowPanel } from "@hmcs/sdk";

// Low-power mode: reduce frame rate and dim overlay
await settings.setFps(15);
await shadowPanel.setAlpha(0.3);
```
