---
sidebar_position: 2
---

# fromPhonemes

`speech.fromPhonemes(phonemes)` converts a list of `[expression_name, duration]` tuples into `TimelineKeyframe[]`. A `null` expression name creates a silent (mouth-closed) keyframe.

Each tuple maps to a `TimelineKeyframe`:

- `["aa", 0.1]` becomes `{ duration: 0.1, targets: { aa: 1.0 } }`
- `[null, 0.05]` becomes `{ duration: 0.05 }` (no mouth expression -- silence)

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `phonemes` | `Array<[string \| null, number]>` | Array of `[expression_name, duration_seconds]` tuples |

## Returns

`TimelineKeyframe[]`

## Example

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
