---
title: "Official Mods"
sidebar_position: 1
---

# Official Mods

MODs extend Desktop Homunculus with characters, animations, sound effects, UI panels, and integrations. The following official MODs are maintained as part of the project.

## MOD List

| MOD | Description | Install |
|---|---|---|
| [@hmcs/assets](./assets) | Default VRM character model, VRMA animations, and sound effects | Recommended |
| [@hmcs/menu](./menu) | Right-click context menu with a WebView-based HUD overlay | Recommended |
| [@hmcs/persona](./persona) | Persona management UI and default behavior service (identity, personality, appearance) | Recommended |
| [@hmcs/settings](./settings) | Application settings panel (frame rate, shadow opacity) via system tray | Recommended |
| [@hmcs/app-exit](./app-exit) | Exit menu in the system tray | Recommended |
| [@hmcs/voicevox](./voicevox) | Text-to-speech integration using the VoiceVox engine | Optional |
| [@hmcs/stt](./stt) | Speech-to-text control panel with Whisper model management | Optional |

## Managing MODs

### List installed MODs

```shell
hmcs mod list
```

Example output:

```text
 NAME            VERSION  DESCRIPTION
 @hmcs/persona   1.0.0    Persona management
 @hmcs/menu      1.0.0    Context menu
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

For detailed CLI reference, see [`hmcs mod`](/reference/cli/mod).
