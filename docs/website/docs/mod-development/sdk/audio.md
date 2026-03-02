---
title: "Audio (SE & BGM)"
sidebar_position: 5
---

# Audio (SE & BGM)

Play sound effects (SE) and background music (BGM) with volume, fading, and playback control. Sound effects are one-shot; BGM is continuous with transport controls.

## Import

```typescript
import { audio } from "@hmcs/sdk";
```

## Sound Effects

`audio.se.play(asset, options?)` plays a one-shot sound effect. The call returns immediately after the sound begins playing.

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

### `SeOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `volume` | `number` | `1.0` | Volume level (0.0--1.0) |
| `speed` | `number` | `1.0` | Playback speed multiplier |
| `panning` | `number` | `0.0` | Stereo panning (-1.0 left to 1.0 right) |

## Background Music

Only one BGM track plays at a time. Starting a new track replaces the current one.

### Play

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

### Stop

```typescript
// Immediate stop
await audio.bgm.stop();

// Fade out over 2 seconds
await audio.bgm.stop({
  fadeOut: { durationSecs: 2.0, easing: "easeOut" },
});
```

### Pause and Resume

```typescript
await audio.bgm.pause();
await audio.bgm.resume();
```

### Update

Change volume or speed while playing. Use the `tween` field to animate the transition.

```typescript
// Fade volume to 0.3 over 1 second
await audio.bgm.update({
  volume: 0.3,
  tween: { durationSecs: 1.0, easing: "easeInOut" },
});

// Change speed immediately
await audio.bgm.update({ speed: 0.8 });
```

### Status

```typescript
const status = await audio.bgm.status();
if (status.state === "playing") {
  console.log(`Now playing: ${status.asset} at volume ${status.volume}`);
}
```

## Types

### `BgmPlayOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `loop` | `boolean` | `true` | Loop playback |
| `volume` | `number` | `1.0` | Volume level (0.0--1.0) |
| `speed` | `number` | `1.0` | Playback speed multiplier |
| `fadeIn` | `FadeTween` | -- | Fade-in transition settings |

### `BgmStopOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `fadeOut` | `FadeTween` | -- | Fade-out transition settings. Omit for immediate stop. |

### `BgmUpdateOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `volume` | `number` | -- | New volume level |
| `speed` | `number` | -- | New playback speed |
| `tween` | `FadeTween` | -- | Transition settings for the change |

### `FadeTween`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `durationSecs` | `number` | -- | Duration in seconds |
| `easing` | `string` | `"linear"` | `"linear"`, `"easeIn"`, `"easeOut"`, or `"easeInOut"` |

### `BgmStatus`

| Field | Type | Description |
|-------|------|-------------|
| `asset` | `string \| null` | Current asset ID, or `null` if stopped |
| `state` | `string` | `"playing"`, `"paused"`, or `"stopped"` |
| `loop` | `boolean` | Whether looping is enabled |
| `volume` | `number` | Current volume level |
| `speed` | `number` | Current playback speed |

## Next Steps

- **[Effects](./effects)** -- Trigger visual stamp effects on screen
- **[Signals](./signals)** -- Cross-process pub/sub messaging for real-time sync
