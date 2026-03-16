---
sidebar_position: 4
---

# bgm.stop

Stops the currently playing BGM.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options` | [`BgmStopOptions`](./types#bgmstopoptions) | Optional stop configuration (e.g., fade-out). Omit for immediate stop. |

## Returns

`Promise<void>`

## Example

```typescript
// Immediate stop
await audio.bgm.stop();

// Fade out over 2 seconds
await audio.bgm.stop({
  fadeOut: { durationSecs: 2.0, easing: "easeOut" },
});
```
