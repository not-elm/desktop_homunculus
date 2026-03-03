---
title: "@hmcs/voicevox"
sidebar_position: 6
---

# @hmcs/voicevox

The VoiceVox MOD (`@hmcs/voicevox`) integrates the [VoiceVox](https://voicevox.hiroshiba.jp/) text-to-speech engine with Desktop Homunculus. It enables characters to speak with lip-synced audio.

## Overview

This MOD connects to a locally running VoiceVox engine to synthesize speech. When invoked, it sends text to the VoiceVox API, receives synthesized audio, and plays it back with automatic lip-sync on the character model.

## Prerequisites

1. **Download and install VoiceVox** from [voicevox.hiroshiba.jp](https://voicevox.hiroshiba.jp/)
2. **Start the VoiceVox engine** — it must be running before using speech features
3. **Install the VoiceVox MOD:**

```shell
hmcs mod install @hmcs/voicevox
```

## Features

The VoiceVox MOD provides three [bin commands](/mod-development/bin-commands) that other MODs and MCP tools can invoke:

### `voicevox:speak`

Makes a character speak text with lip-synced audio.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|---|---|---|---|---|
| `entity` | number | Yes | — | Character entity ID |
| `text` | string or string[] | Yes | — | Text to speak (single string or array of sentences) |
| `speaker` | number | No | `0` | VoiceVox speaker ID |
| `voicevox_host` | string | No | `http://localhost:50021` | VoiceVox engine URL |
| `speed_scale` | number | No | — | Speech speed multiplier |
| `pitch_scale` | number | No | — | Pitch multiplier |
| `intonation_scale` | number | No | — | Intonation multiplier |
| `volume_scale` | number | No | — | Volume multiplier |
| `fetch_timeout_ms` | number | No | `30000` | Timeout in milliseconds for VoiceVox API requests |

### `voicevox:speakers`

Lists available VoiceVox speakers (voices).

**Parameters:**

| Parameter | Type | Required | Default | Description |
|---|---|---|---|---|
| `voicevox_host` | string | No | `http://localhost:50021` | VoiceVox engine URL |

### `voicevox:initialize`

Pre-loads a speaker model for faster first synthesis.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|---|---|---|---|---|
| `speaker` | number | No | `0` | VoiceVox speaker ID to initialize |
| `voicevox_host` | string | No | `http://localhost:50021` | VoiceVox engine URL |

## Troubleshooting

### VoiceVox engine not reachable

**Symptom:** Commands fail with `VOICEVOX_UNREACHABLE`.

**Solution:** Ensure the VoiceVox application is running. By default it listens on `http://localhost:50021`. If you changed the port, pass the `voicevox_host` parameter.

### Speech is slow

**Symptom:** Noticeable delay before speech starts.

**Cause:** Each utterance requires VoiceVox to synthesize audio before playback. Longer text takes longer.

**Tip:** Use `voicevox:initialize` once per VoiceVox session to pre-load the speaker model, which reduces first-synthesis latency. You can also split long text into shorter sentences for faster incremental playback.
