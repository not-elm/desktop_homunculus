---
sidebar_position: 2
---

# stamp

Displays a visual stamp effect on the screen.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `asset` | `string` | Asset ID of the stamp image (e.g., `"my-mod:heart"`) |
| `options` | `StampOptions` | Optional appearance configuration |

## Returns

`Promise<void>`

## Example

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
