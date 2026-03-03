---
title: "speech"
sidebar_position: 6
---

# speech

Utilities for converting phoneme data into timeline keyframes for VRM lip-sync. The `speech` module produces keyframe arrays that you pass to `vrm.speakWithTimeline()`.

## Import

```typescript
import { speech } from "@hmcs/sdk";
```

## Phoneme to Timeline

`speech.fromPhonemes(phonemes)` converts a list of `[expression_name, duration]` tuples into `TimelineKeyframe[]`. A `null` expression name creates a silent (mouth-closed) keyframe.

```typescript
import { speech, Vrm } from "@hmcs/sdk";

const keyframes = speech.fromPhonemes([
  ["aa", 0.1],
  [null, 0.05],
  ["oh", 0.12],
  ["ee", 0.08],
  [null, 0.1],
]);

const vrm = await Vrm.findByName("MyAvatar");
await vrm.speakWithTimeline(wavData, keyframes);
```

Each tuple maps to a `TimelineKeyframe`:

- `["aa", 0.1]` becomes `{ duration: 0.1, targets: { aa: 1.0 } }`
- `[null, 0.05]` becomes `{ duration: 0.05 }` (no mouth expression -- silence)

### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `phonemes` | `Array<[string \| null, number]>` | Array of `[expression_name, duration_seconds]` tuples |

**Returns:** `TimelineKeyframe[]`

## Types

### `TimelineKeyframe`

Defined in the VRM module. Each keyframe specifies a duration and optional expression targets.

| Field | Type | Description |
|-------|------|-------------|
| `duration` | `number` | Duration in seconds |
| `targets` | `Record<string, number>` | Expression name to weight (0--1) mapping. Omit for silence. |

## Next Steps

- **[VRM Module](./vrm/)** -- Full character control including `speakWithTimeline()`. For VoiceVox text-to-speech synthesis, use the `@hmcs/voicevox` MOD's bin commands.
- **[Audio](./audio)** -- Sound effects and background music playback
