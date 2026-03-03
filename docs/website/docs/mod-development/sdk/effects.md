---
title: "effects"
sidebar_position: 8
---

# effects

Trigger visual stamp effects on screen -- images displayed at a given position with configurable size, opacity, and duration.

## Import

```typescript
import { effects } from "@hmcs/sdk";
```

## Stamp Effect

`effects.stamp(asset, options?)` displays an image asset as a temporary overlay on screen.

```typescript
// Minimal -- display at default position and size
await effects.stamp("my-mod:thumbs-up");

// Full options
await effects.stamp("my-mod:heart", {
  x: 100,
  y: 200,
  width: 80,
  height: 80,
  alpha: 0.9,
  duration: 1.5,
});
```

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `asset` | `string` | Asset ID of the stamp image (e.g., `"my-mod:heart"`) |
| `options` | `StampOptions` | Optional appearance configuration |

## Types

### `StampOptions`

| Field | Type | Description |
|-------|------|-------------|
| `x` | `number` | X position on screen (pixels) |
| `y` | `number` | Y position on screen (pixels) |
| `width` | `number` | Width in pixels |
| `height` | `number` | Height in pixels |
| `alpha` | `number` | Opacity (0--1) |
| `duration` | `number` | Duration in seconds before the stamp disappears |

## Next Steps

- **[Audio](./audio)** -- Play sound effects and background music
- **[Signals](./signals)** -- Cross-process pub/sub messaging
