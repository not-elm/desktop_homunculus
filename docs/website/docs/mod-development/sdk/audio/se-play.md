---
sidebar_position: 2
---

# se.play

Plays a one-shot sound effect. The call returns immediately after the sound begins playing.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `asset` | `string` | The asset ID of the sound effect (e.g., `"my-mod:click"`) |
| `options` | [`SeOptions`](./types#seoptions) | Optional playback configuration |

## Returns

`Promise<void>`

## Example

```typescript
// Play a sound effect by asset ID
await audio.se.play("my-mod:click");

// With playback options
await audio.se.play("my-mod:alert", {
  volume: 0.5,
  speed: 1.2,
  panning: -0.5,  // -1.0 = left, 0.0 = center, 1.0 = right
});
```
