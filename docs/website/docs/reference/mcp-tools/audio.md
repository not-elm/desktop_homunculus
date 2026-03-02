---
title: "Audio"
sidebar_position: 4
---

# Audio

Audio tools handle speech, sound effects, and background music.

#### `speak_message`

Make the active character speak text aloud using VoiceVox text-to-speech.

:::note
Requires the VoiceVox MOD to be installed and the VoiceVox engine to be running.
:::

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `text` | `string \| string[]` | **required** | Text to speak. Pass an array of short sentences for more reliable synthesis. |
| `speaker` | `number` | `0` | VoiceVox speaker ID |
| `timeoutMs` | `number` | `30000` | Timeout in milliseconds (range: 1000–120000) |

**Example:**

```json
{
  "text": ["Hello!", "How can I help you today?"],
  "speaker": 3,
  "timeoutMs": 15000
}
```

---

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

