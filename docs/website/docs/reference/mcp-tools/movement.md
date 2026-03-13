---
title: "Movement"
sidebar_position: 3
---

# Movement

Movement tools teleport or tween transforms for the active character.

All movement tools target the active character. Use `select_character` first when working with multiple characters.

#### `move_character`

Teleport the active character to a viewport position instantly. `(0, 0)` is the top-left corner of the primary monitor.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `x` | `number` | **required** | Viewport X coordinate (pixels) |
| `y` | `number` | **required** | Viewport Y coordinate (pixels) |

---

#### `tween_position`

Smoothly animate the active character's position to a viewport position. Coordinates are in viewport pixels (`0,0` = top-left of primary monitor), matching `move_character`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `x` | `number` | **required** | Viewport X coordinate (pixels) |
| `y` | `number` | **required** | Viewport Y coordinate (pixels) |
| `durationMs` | `number` | **required** | Animation duration in milliseconds |
| `easing` | `string` | `"linear"` | Easing function (see [Easing Functions](#easing-functions)) |
| `wait` | `boolean` | `false` | Wait for animation to finish before returning |

---

#### `tween_rotation`

Smoothly animate the active character's rotation to a target quaternion.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `targetX` | `number` | **required** | Target quaternion X |
| `targetY` | `number` | **required** | Target quaternion Y |
| `targetZ` | `number` | **required** | Target quaternion Z |
| `targetW` | `number` | **required** | Target quaternion W |
| `durationMs` | `number` | **required** | Animation duration in milliseconds |
| `easing` | `string` | `"linear"` | Easing function (see [Easing Functions](#easing-functions)) |
| `wait` | `boolean` | `false` | Wait for animation to finish before returning |

**Example — 180° Y-axis rotation over 1 second:**

```json
{
  "targetX": 0,
  "targetY": 1,
  "targetZ": 0,
  "targetW": 0,
  "durationMs": 1000,
  "easing": "cubicInOut"
}
```

---

#### `spin_character`

Spin the active character around a world-space axis by a given angle. Supports full rotations (360°+). The rotation is additive — it preserves the character's current orientation. Use this instead of `tween_rotation` when you need full spins.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `axis` | `string` | **required** | Rotation axis: `"x"`, `"y"`, or `"z"` (world-space) |
| `angleDegrees` | `number` | **required** | Rotation angle in degrees (supports 360, 720, etc.) |
| `durationMs` | `number` | **required** | Animation duration in milliseconds |
| `easing` | `string` | `"linear"` | Easing function (see [Easing Functions](#easing-functions)) |
| `wait` | `boolean` | `false` | Wait for animation to finish before returning |

**Example — full 360° Y-axis spin over 3 seconds:**

```json
{
  "axis": "y",
  "angleDegrees": 360,
  "durationMs": 3000
}
```

---

#### `tween_scale`

Smoothly animate the active character's scale. `1.0` is normal size on each axis.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `targetX` | `number ≥ 0` | **required** | Target X scale factor |
| `targetY` | `number ≥ 0` | **required** | Target Y scale factor |
| `targetZ` | `number ≥ 0` | **required** | Target Z scale factor |
| `durationMs` | `number` | **required** | Animation duration in milliseconds |
| `easing` | `string` | `"linear"` | Easing function (see [Easing Functions](#easing-functions)) |
| `wait` | `boolean` | `false` | Wait for animation to finish before returning |

---

#### Easing Functions

All tween tools (`tween_position`, `tween_rotation`, `tween_scale`, `spin_character`) accept the same easing values:

`linear`, `quadraticIn`, `quadraticOut`, `quadraticInOut`, `cubicIn`, `cubicOut`, `cubicInOut`, `quarticIn`, `quarticOut`, `quarticInOut`, `quinticIn`, `quinticOut`, `quinticInOut`, `sineIn`, `sineOut`, `sineInOut`, `circularIn`, `circularOut`, `circularInOut`, `exponentialIn`, `exponentialOut`, `exponentialInOut`, `elasticIn`, `elasticOut`, `elasticInOut`, `backIn`, `backOut`, `backInOut`, `bounceIn`, `bounceOut`, `bounceInOut`, `smoothStepIn`, `smoothStepOut`, `smoothStep`, `smootherStepIn`, `smootherStepOut`, `smootherStep`

