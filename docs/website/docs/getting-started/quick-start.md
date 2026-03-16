---
title: "Quick Start"
sidebar_position: 3
---

# Quick Start

:::tip[Before You Begin]
Make sure you have completed the [Installation](/getting-started/installation) guide. Desktop Homunculus and the official MODs should be installed and the app should be running.
:::

## Meet Elmer

When you launch Desktop Homunculus with the official MODs installed, **Elmer** appears on your desktop. Elmer is the default character -- a VRM 3D model rendered in a transparent window that floats above your other applications.

Elmer comes with built-in animations and behaviors powered by the `@hmcs/elmer` and `@hmcs/assets` MODs. The character will idle, react when grabbed, and follow your cursor.

## Interact with Your Character

### Drag and Move

Click and drag the character to reposition it anywhere on your desktop. While being dragged, Elmer switches to a "drag" state. Release to drop the character at the new position.

### Character States

The character responds to different states, each with its own behavior:

| State | Trigger |
|---|---|
| **`idle`** | Default state |
| **`drag`** | Click and drag the character |
| **`sitting`** | Drop the character on a window edge |

### Right-Click Menu

Right-click the character to open the context menu overlay. From here you can access settings and other actions provided by installed MODs.

### Settings

Open the Settings panel through the right-click context menu. The settings UI lets you configure the application and MOD-specific options.

### Exiting the App

1. Find the **Desktop Homunculus** icon in the **system tray** (notification area)
2. Click the tray icon to open the tray menu
3. Select **Exit**

:::tip
The `@hmcs/app-exit` MOD provides the Exit option in the system tray. Make sure it is installed — especially on Windows, where the app is hidden from the taskbar and Alt+Tab.
:::

## Explore Official MODs

Desktop Homunculus ships with a set of official MODs under the `@hmcs` scope:

| MOD | Description |
|---|---|
| `@hmcs/elmer` | Controls the default character (Elmer) — spawns the character, manages animations and cursor tracking |
| `@hmcs/assets` | Default VRMA animations (`idle-maid`, `grabbed`, `idle-sitting`) and sound effects |
| `@hmcs/character-settings` | Per-character settings panel accessible from the context menu |
| `@hmcs/settings` | Application settings panel accessible from the system tray |
| `@hmcs/app-exit` | Exit menu in the system tray (required on Windows) |
| `@hmcs/menu` | Right-click context menu overlay |

## Additional MODs

You can extend Desktop Homunculus with optional MODs. Install any of them at any time using the CLI:

| MOD | Description |
|---|---|
| `@hmcs/voicevox` | Text-to-speech integration using the [VoiceVox](https://voicevox.hiroshiba.jp/) engine |

:::info[VoiceVox Setup]
The `@hmcs/voicevox` MOD requires the VoiceVox engine to be installed and running separately. Visit the [VoiceVox website](https://voicevox.hiroshiba.jp/) for installation instructions.
:::

```shell
hmcs mod install @hmcs/voicevox
```

## What's Next?

- **[MOD Development](/mod-development)** — Build your own MODs with the TypeScript SDK
- **[API Reference](/reference/api/homunculus-api)** — Explore the full HTTP API documentation
