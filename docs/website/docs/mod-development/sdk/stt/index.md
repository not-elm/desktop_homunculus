---
title: "STT"
sidebar_position: 1
---

# STT Module

Real-time speech-to-text transcription using local Whisper models. All processing runs on-device — no cloud services or API keys required.

```typescript
import { stt } from "@hmcs/sdk";
```

## Quick Start

```typescript
import { stt } from "@hmcs/sdk";

// 1. Ensure the model is downloaded
await stt.models.download({ modelSize: "small" });

// 2. Start a session
await stt.session.start({ language: "ja" });

// 3. Stream transcription results
const stream = stt.stream({
  onResult: (result) => {
    console.log(`[${result.language}] ${result.text}`);
  },
  onSessionError: (err) => {
    console.error(`STT error: ${err.message}`);
  },
});

// 4. Stop when done
await stt.session.stop();
stream.close();
```

## How It Works

The STT pipeline runs entirely on-device using [Whisper](https://github.com/openai/whisper) models:

1. **Capture** — Microphone audio is captured in a dedicated thread
2. **VAD** — Voice Activity Detection filters out silence, sending only speech segments forward
3. **Inference** — Whisper processes speech segments and emits transcription results

Sessions are managed via `stt.session` and results are streamed in real time via `stt.stream()`.

## Model Sizes

| Size | Download | Speed | Accuracy | Notes |
|------|----------|-------|----------|-------|
| `"tiny"` | 32.2 MB | Fastest | Lower | Good for quick prototyping |
| `"base"` | 59.7 MB | Fast | Moderate | Balanced for simple tasks |
| `"small"` | 190 MB | Moderate | Good | **Default.** Recommended for most use cases |
| `"medium"` | 539 MB | Slower | High | Higher accuracy, more resource usage |
| `"large-v3-turbo"` | 574 MB | Slower | Higher | Near Large v3 accuracy with faster inference |
| `"large-v3"` | 1.08 GB | Slowest | Best | Best accuracy, highest resource usage |

## Prerequisites

:::info[Microphone Access]
STT requires microphone access. On macOS, the system will prompt for permission on first use. If the microphone is unavailable, `stt.session.start()` will throw an error with code `no_microphone` or `microphone_permission_denied`.
:::

## Next Steps

- **[Session & Streaming](./session-and-streaming)** -- Session lifecycle, real-time transcription events, and error handling
- **[Models](./models)** -- Download and manage Whisper models
- **[speech](../speech)** -- Convert phoneme data into lip-sync keyframes (complementary output feature)
- **[audio](../audio)** -- Play sound effects and background music
