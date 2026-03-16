---
title: "Expression"
sidebar_position: 2
---

# Expression

Expression tools control facial weights, animation playback, and look-at behavior.

#### `set_expression`

Set facial expression weights on the active character. Weights are in the range `0.0–1.0`.

Common expression names: `happy`, `sad`, `angry`, `surprised`, `relaxed`, `neutral`, `aa`, `ih`, `ou`, `ee`, `oh`, `blink`.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `expressions` | `Record<string, number>` | — | Expression name → weight map. Required unless `mode` is `"clear"`. |
| `mode` | `"set" \| "modify" \| "clear"` | `"modify"` | `"modify"` updates only the listed expressions; `"set"` replaces all; `"clear"` resets to animation-controlled state. |

**Example — soft smile:**

```json
{
  "expressions": { "happy": 0.8, "relaxed": 0.3 },
  "mode": "modify"
}
```

---

#### `play_animation`

Play a VRMA animation on the active character. Use the `homunculus://assets` resource to discover available VRMA asset IDs.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `asset` | `string` | **required** | VRMA animation asset ID (e.g. `vrma:idle-maid`) |
| `repeat` | `"never" \| "forever" \| string` | `"never"` | `"never"` plays once, `"forever"` loops, or pass a number string like `"3"` to repeat N times |
| `transition_secs` | `number` | `0.3` | Crossfade transition duration in seconds |
| `wait` | `boolean` | `false` | Wait for animation to complete before returning |
| `reset_spring_bones` | `boolean` | `false` | Reset SpringBone physics on transition to prevent bouncing |

**Example — loop an idle animation:**

```json
{
  "asset": "vrma:idle-maid",
  "repeat": "forever",
  "transition_secs": 0.5
}
```

---

#### `set_look_at`

Control where the active character's eyes look.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `mode` | `"cursor" \| "none"` | **required** | `"cursor"` follows the mouse pointer; `"none"` disables look-at |
