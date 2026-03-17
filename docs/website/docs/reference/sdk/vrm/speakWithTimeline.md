---
title: "speakWithTimeline"
sidebar_position: 40
---

# speakWithTimeline

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.speakWithTimeline(audio, keyframes, options?)` plays WAV audio while applying expression keyframes at precise timestamps. The engine synchronizes mouth movements with the audio playback.

```typescript
const character = await Vrm.findByName("MyAvatar");
const wavData = await fetch("https://my-tts-service/generate?text=Hello")
  .then((r) => r.arrayBuffer());

await character.speakWithTimeline(wavData, [
  { duration: 0.1, targets: { aa: 1.0 } },
  { duration: 0.05 },
  { duration: 0.12, targets: { oh: 1.0 } },
  { duration: 0.08, targets: { ee: 0.8 } },
]);
```

## Parameters

| Parameter  | Type                       | Description                             |
| ---------- | -------------------------- | --------------------------------------- |
| `audio`    | `ArrayBuffer \| Uint8Array` | WAV audio data                          |
| `keyframes`| [`TimelineKeyframe`](./types#timelinekeyframe)`[]`       | Sequence of timed expression targets    |
| `options`  | [`SpeakTimelineOptions`](./types#speaktimelineoptions)     | Optional settings                       |

## Options

| Option               | Type      | Default | Description                                                                                          |
| -------------------- | --------- | ------- | ---------------------------------------------------------------------------------------------------- |
| `waitForCompletion`  | `boolean` | `true`  | If `true`, the call blocks until the speech finishes                                                 |
| `transitionDuration` | `number`  | `0.05`  | Seconds for smoothstep blending between keyframes. Clamped to 40% of each keyframe's duration.      |

## Keyframe Format

Each keyframe specifies a duration and optional expression targets:

```typescript
const keyframes: TimelineKeyframe[] = [
  { duration: 0.1, targets: { aa: 1.0 } },            // "ah" sound
  { duration: 0.05 },                                   // brief pause
  { duration: 0.12, targets: { oh: 1.0, happy: 0.5 }}, // "oh" with smile
  { duration: 0.08, targets: { ee: 0.8 } },             // "ee" sound
];
```

A keyframe without `targets` creates a silent pause where no mouth expressions are applied.

:::note
Expression targets are not limited to mouth shapes. You can include emotional expressions like `happy` or `surprised` in keyframes to synchronize facial reactions with specific moments in the audio.
:::

## Non-Blocking Speech

By default, `speakWithTimeline` waits for the audio to finish. Set `waitForCompletion: false` to continue execution immediately:

```typescript
await character.speakWithTimeline(wavData, keyframes, {
  waitForCompletion: false,
});
```

## Building Keyframes from Phonemes

The `speech` module provides a helper for converting simple phoneme lists into timeline keyframes:

```typescript
import { speech, Vrm } from "@hmcs/sdk";

const keyframes = speech.fromPhonemes([
  ["aa", 0.1],     // "ah" for 100ms
  [null, 0.05],    // silence for 50ms
  ["oh", 0.12],    // "oh" for 120ms
  ["ee", 0.08],    // "ee" for 80ms
]);

await character.speakWithTimeline(wavData, keyframes);
```

Each tuple is `[expression_name | null, duration_in_seconds]`. A `null` expression creates a silent keyframe.
