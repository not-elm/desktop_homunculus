---
sidebar_position: 100
---

# Type Definitions

## SeOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `volume` | `number` | `1.0` | Volume level (0.0--1.0) |
| `speed` | `number` | `1.0` | Playback speed multiplier |
| `panning` | `number` | `0.0` | Stereo panning (-1.0 left to 1.0 right) |

## BgmPlayOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `loop` | `boolean` | `true` | Loop playback |
| `volume` | `number` | `1.0` | Volume level (0.0--1.0) |
| `speed` | `number` | `1.0` | Playback speed multiplier |
| `fadeIn` | `FadeTween` | -- | Fade-in transition settings |

## BgmStopOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `fadeOut` | `FadeTween` | -- | Fade-out transition settings. Omit for immediate stop. |

## BgmUpdateOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `volume` | `number` | -- | New volume level |
| `speed` | `number` | -- | New playback speed |
| `tween` | `FadeTween` | -- | Transition settings for the change |

## FadeTween

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `durationSecs` | `number` | -- | Duration in seconds |
| `easing` | `string` | `"linear"` | `"linear"`, `"easeIn"`, `"easeOut"`, or `"easeInOut"` |

## BgmStatus

| Field | Type | Description |
|-------|------|-------------|
| `asset` | `string \| null` | Current asset ID, or `null` if stopped |
| `state` | `string` | `"playing"`, `"paused"`, or `"stopped"` |
| `loop` | `boolean` | Whether looping is enabled |
| `volume` | `number` | Current volume level |
| `speed` | `number` | Current playback speed |
