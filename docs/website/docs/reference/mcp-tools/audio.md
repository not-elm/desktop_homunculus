---
title: "Audio"
sidebar_position: 4
---

# Audio

Audio tools handle speech, sound effects, and background music.

#### `play_sound`

Play a sound effect asset.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `sound` | `string` | **required** | Sound asset ID (e.g. `se:open`) |
| `volume` | `number` | `0.8` | Volume level (range: 0.0–1.0) |

---

#### `control_bgm`

Control background music playback.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `action` | `"play" \| "stop" \| "pause" \| "resume" \| "status"` | **required** | Action to perform |
| `asset` | `string` | — | MOD asset ID — required when `action` is `"play"` |
| `volume` | `number` | — | Volume level (range: 0.0–1.0) |

The `"status"` action returns the current playback state as JSON.
