---
title: "@hmcs/stt"
sidebar_position: 10
---

# @hmcs/stt

The Speech-to-Text MOD (`@hmcs/stt`) provides voice recognition using OpenAI's Whisper model. It includes a UI control panel for managing Whisper models and configuring recognition settings.

## Overview

STT (Speech-to-Text) converts spoken audio from your microphone into text. The recognition engine runs locally — no cloud API required. The MOD supports:

- **Single-shot recognition** — Speak a sentence and get the transcription
- **Push-to-Talk (PTT)** — Hold a key to record, release to transcribe
- **6 model sizes** — From tiny (fast, less accurate) to large-v3 (slow, most accurate)
- **Language auto-detection** — Or manual language override

## Prerequisites

Install the STT MOD:

```shell
hmcs mod install @hmcs/stt
```

**Requirements:**
- Microphone access (the OS may prompt for permission on first use)
- Disk space for Whisper models (75 MB for tiny, up to 3 GB for large-v3)

**GPU acceleration (optional):**
- macOS: Metal acceleration via `stt-metal` feature flag
- NVIDIA GPUs: CUDA acceleration via `stt-cuda` feature flag

## Control Panel

Access from the **system tray** → **"Speech to Text"**.

The control panel lets you:

- **Download** Whisper models with progress display
- **Delete** downloaded models to free disk space
- **View** downloaded models and their file sizes
- **Configure** default language and model size

### Available Models

| Model | Size | Speed | Accuracy |
|-------|------|-------|----------|
| `tiny` | ~75 MB | Fastest | Basic |
| `base` | ~150 MB | Fast | Good |
| `small` | ~500 MB | Moderate | Better |
| `medium` | ~1.5 GB | Slow | High |
| `large-v3-turbo` | ~1.6 GB | Moderate | Very High |
| `large-v3` | ~3 GB | Slowest | Highest |

:::tip
Start with the `base` model for a good balance of speed and accuracy. Upgrade to `small` or `medium` if recognition quality isn't sufficient.
:::

## Agent Integration

When used with the [`@hmcs/agent`](./agent) MOD:

1. Press the configured **PTT key** (set in Agent settings)
2. Speak your input
3. Release the key — audio is transcribed via Whisper
4. The transcription is fed directly into the agent's input

This enables a voice → text → AI reasoning → action flow.

## SDK Reference

For programmatic STT access, see the [STT SDK reference](/reference/sdk/speech).

## Notes

- Whisper models are stored in `~/.homunculus/stt_models/`.
- The STT engine (`homunculus_microphone`) is built into the Desktop Homunculus engine, not the MOD itself. The MOD provides the UI and user-facing controls.
- Language defaults to auto-detection. Override in the control panel or via the SDK.
