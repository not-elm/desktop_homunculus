---
title: "Official Mods"
sidebar_position: 1
---

# Official Mods

MODs extend Desktop Homunculus with characters, animations, sound effects, UI panels, and integrations. The following official MODs are maintained as part of the project.

## MOD List

| MOD | Description | Install |
|---|---|---|
| [Assets](./assets) | Default VRM character model, VRMA animations, and sound effects | Recommended |
| [Elmer](./elmer) | Default character that spawns Elmer with idle, grab, and sitting animations | Recommended |
| [Context Menu](./menu) | Right-click context menu with a WebView-based HUD overlay | Recommended |
| [Character Settings](./character-settings) | Per-character settings panel (name, scale, persona, OCEAN traits) | Recommended |
| [Settings](./settings) | Application settings panel (frame rate, shadow opacity) via system tray | Recommended |
| [VoiceVox](./voicevox) | Text-to-speech integration using the VoiceVox engine | Optional |

## Managing MODs

### List installed MODs

```shell
hmcs mod list
```

Example output:

```text
 NAME           VERSION  DESCRIPTION
 @hmcs/elmer    1.0.0    Default character model
 @hmcs/menu     1.0.0    Context menu
```

### Install a MOD

```shell
hmcs mod install <package>...
```

For example, to install the VoiceVox MOD:

```shell
hmcs mod install @hmcs/voicevox
```

### Uninstall a MOD

```shell
hmcs mod uninstall <package>...
```

For detailed CLI reference, see [`hmcs mod`](/docs/reference/cli/mod).
