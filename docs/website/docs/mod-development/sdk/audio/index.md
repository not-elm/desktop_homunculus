---
sidebar_position: 1
---

# audio

Play sound effects (SE) and background music (BGM) with volume, fading, and playback control. Sound effects are one-shot; BGM is continuous with transport controls.

## Import

```typescript
import { audio } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [se.play](./se-play) | Play a one-shot sound effect |
| [bgm.play](./bgm-play) | Start background music playback |
| [bgm.stop](./bgm-stop) | Stop the currently playing BGM |
| [bgm.pause](./bgm-pause) | Pause the currently playing BGM |
| [bgm.resume](./bgm-resume) | Resume paused BGM playback |
| [bgm.update](./bgm-update) | Update volume or speed while playing |
| [bgm.status](./bgm-status) | Get the current BGM playback status |
