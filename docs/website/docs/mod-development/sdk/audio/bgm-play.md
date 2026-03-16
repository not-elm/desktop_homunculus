---
sidebar_position: 3
---

# bgm.play

Plays background music, replacing any currently playing BGM. Only one BGM track plays at a time.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `asset` | `string` | The asset ID of the music track (e.g., `"my-mod:battle-theme"`) |
| `options` | [`BgmPlayOptions`](./types#bgmplayoptions) | Optional playback configuration |

## Returns

`Promise<void>`

## Example

```typescript
// Loop by default
await audio.bgm.play("my-mod:battle-theme");

// One-shot with fade-in
await audio.bgm.play("my-mod:intro", {
  loop: false,
  volume: 0.6,
  fadeIn: { durationSecs: 3.0, easing: "easeIn" },
});
```
