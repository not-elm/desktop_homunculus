---
sidebar_position: 7
---

# bgm.update

Change volume or speed while playing. Use the `tween` field to animate the transition.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options` | `BgmUpdateOptions` | The parameters to update |

## Returns

`Promise<void>`

## Example

```typescript
// Fade volume to 0.3 over 1 second
await audio.bgm.update({
  volume: 0.3,
  tween: { durationSecs: 1.0, easing: "easeInOut" },
});

// Change speed immediately
await audio.bgm.update({ speed: 0.8 });
```
