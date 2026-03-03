---
title: "Character"
sidebar_position: 1
---

# Character

Character tools manage VRM lifecycle, active selection, and persona metadata.

#### `get_character_snapshot`

Get the current state of all desktop characters. Returns entity ID, name, position, active expressions, playing animations, persona, and lookAt state for each character.

This tool takes no parameters. The first character found is automatically set as the active character for subsequent tool calls.

**Example response:**

```json
[
  {
    "entity": 42,
    "name": "Elmer",
    "state": "idle",
    "position": [800, 600],
    "activeExpressions": [{ "name": "happy", "weight": 1.0 }],
    "playingAnimations": ["idle-maid"],
    "persona": { "profile": "A cheerful assistant", "personality": null },
    "lookAt": { "type": "cursor" }
  }
]
```

`position` is `[x, y]` in global viewport coordinates, or `null` if unavailable. `lookAt` is an object (for example `{ "type": "cursor" }` or `{ "type": "target", "entity": 123 }`) or `null`.

---

#### `spawn_character`

Spawn a new VRM character on the desktop. The spawned character becomes the active character. Use the `homunculus://assets` resource to discover available VRM asset IDs.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `asset` | `string` | **required** | VRM model asset ID (e.g. `vrm:elmer`) |
| `name` | `string` | — | Display name for the character |
| `persona_profile` | `string` | — | Character personality/background description |
| `x` | `number` | — | Initial viewport X position (pixels) |
| `y` | `number` | — | Initial viewport Y position (pixels) |

**Example:**

```json
{
  "asset": "vrm:elmer",
  "name": "Elmer",
  "persona_profile": "A cheerful coding companion who loves Rust",
  "x": 900,
  "y": 700
}
```

---

#### `remove_character`

Remove a VRM character from the desktop.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `string` | — | Name of the character to remove. If omitted, removes the active character. |

---

#### `select_character`

Switch the active character by name. All subsequent tools that target "the active character" will target this character.

Use `get_character_snapshot` to list available names.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `string` | **required** | Character name to select |

---

#### `set_persona`

Set the active character's personality profile. This affects how the character is presented in AI conversations.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `profile` | `string` | **required** | Character background description |
| `personality` | `string` | — | Personality traits in natural language |

**Example:**

```json
{
  "profile": "A serious researcher who specializes in distributed systems",
  "personality": "Precise, methodical, occasionally dry humor"
}
```
