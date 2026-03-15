---
title: "@hmcs/settings"
sidebar_position: 6
---

# @hmcs/settings

The Settings MOD (`@hmcs/settings`) provides an application-wide settings panel for tuning rendering and display preferences. It registers a **system tray menu** entry so you can open it without right-clicking a character.

## Overview

Open the settings panel from the **system tray icon**:

1. Click the Desktop Homunculus tray icon in your OS menu bar / system tray
2. Select **"Settings"**
3. The settings panel opens as a floating WebView window

## Features

Changes take effect after clicking **Save**. All settings are persisted to `~/.homunculus/preferences.db`.

### Frame Rate

| Setting | Description | Range |
|---------|-------------|-------|
| Frame Rate | Controls the rendering frame rate. Lower values reduce CPU/GPU usage. | 1+ fps |

### Shadow Opacity

| Setting | Description | Range |
|---------|-------------|-------|
| Shadow Opacity | Controls the transparency of the shadow panel overlay behind the character. | 0 – 100% |

### Speech (STT)

The **Speech** tab controls real-time speech-to-text transcription powered by local [Whisper](https://github.com/openai/whisper) models. All processing runs on-device.

#### Language

| Setting | Description | Default |
|---------|-------------|---------|
| Language | The language for speech recognition. Choose a specific language (ISO 639-1 code) for better accuracy, or leave as **Auto** to let the model detect the language automatically. | Auto |

#### Model

Select a Whisper model to use for transcription. Larger models are more accurate but require more memory and CPU.

| Model | Download Size | Notes |
|-------|--------------|-------|
| Tiny | 32.2 MB | Fastest, lower accuracy |
| Base | 59.7 MB | Fast, moderate accuracy |
| Small | 190 MB | **Default.** Good accuracy for most languages |
| Medium | 539 MB | Higher accuracy, more resource usage |
| Large v3 Turbo | 574 MB | Near Large v3 accuracy with faster inference |
| Large v3 | 1.08 GB | Best accuracy, highest resource usage |

Models are downloaded on demand — click a model card to start the download. A progress indicator is shown during download.

#### Session Control

| Action | Description |
|--------|-------------|
| Start | Begin capturing microphone audio and transcribing speech |
| Stop | End the current transcription session |

A status indicator shows the current session state: **Idle**, **Loading**, **Listening**, or **Error**.

:::info[Microphone Access]
STT requires microphone access. On macOS, the system will prompt for permission on first use.
:::

## SDK

The `settings` namespace in `@hmcs/sdk` provides programmatic access to application settings:

```typescript
import { settings } from "@hmcs/sdk";

// Read the current FPS
const currentFps = await settings.fps();

// Set a new FPS value
await settings.setFps(30);
```

See the [Settings SDK reference](/reference/sdk/settings) for full API documentation.

## Notes

- The Settings MOD uses a [tray menu](../mod-development/tray-menus) instead of a context menu, because application settings are not tied to a specific character.
- The settings panel UI uses the shared `@hmcs/ui` component library.
