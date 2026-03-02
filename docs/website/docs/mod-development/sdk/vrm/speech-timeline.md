---
title: "Speech Timeline"
sidebar_position: 5
---

# Speech Timeline

Play audio with frame-synchronized expression keyframes for lip-sync. The speech timeline system lets any TTS engine drive VRM mouth animations by sending WAV audio alongside a sequence of timed expression targets.

## Import

```typescript
import { Vrm, speech } from "@hmcs/sdk";
```

## Speaking with a Timeline

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

### Parameters

| Parameter | Type | Description |
|---|---|---|
| `audio` | `ArrayBuffer \| Uint8Array` | WAV audio data |
| `keyframes` | `TimelineKeyframe[]` | Sequence of timed expression targets |
| `options` | `SpeakTimelineOptions` | Optional settings |

### Options

| Option | Type | Default | Description |
|---|---|---|---|
| `waitForCompletion` | `boolean` | `true` | If `true`, the call blocks until the speech finishes |
| `transitionDuration` | `number` | `0.05` | Seconds for smoothstep blending between keyframes. Clamped to 40% of each keyframe's duration. |

## Keyframe Format

Each keyframe specifies a duration and optional expression targets:

```typescript
interface TimelineKeyframe {
  /** Duration of this keyframe in seconds. */
  duration: number;
  /** Expression weights to set. Keys are expression names, values are 0.0-1.0. */
  targets?: Record<string, number>;
}
```

A keyframe without `targets` creates a silent pause where no mouth expressions are applied:

```typescript
const keyframes: TimelineKeyframe[] = [
  { duration: 0.1, targets: { aa: 1.0 } },           // "ah" sound
  { duration: 0.05 },                                  // brief pause
  { duration: 0.12, targets: { oh: 1.0, happy: 0.5 }}, // "oh" with smile
  { duration: 0.08, targets: { ee: 0.8 } },            // "ee" sound
];
```

:::note
Expression targets are not limited to mouth shapes. You can include emotional expressions like `happy` or `surprised` in keyframes to synchronize facial reactions with specific moments in the audio.
:::

## Transition Blending

The `transitionDuration` option controls how smoothly expressions blend between adjacent keyframes. The default is 50ms, which produces natural-looking transitions.

```typescript
// Smoother transitions (100ms blend)
await character.speakWithTimeline(wavData, keyframes, {
  transitionDuration: 0.1,
});

// Snappier transitions (20ms blend)
await character.speakWithTimeline(wavData, keyframes, {
  transitionDuration: 0.02,
});
```

The transition duration is automatically clamped to 40% of each keyframe's duration, so short keyframes never get overrun by their transitions.

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

const character = await Vrm.findByName("MyAvatar");
await character.speakWithTimeline(wavData, keyframes);
```

Each tuple is `[expression_name | null, duration_in_seconds]`. A `null` expression creates a silent keyframe.

## Non-Blocking Speech

By default, `speakWithTimeline` waits for the audio to finish. Set `waitForCompletion: false` to continue execution immediately:

```typescript
// Start speaking without blocking
await character.speakWithTimeline(wavData, keyframes, {
  waitForCompletion: false,
});

// This runs while the character is still speaking
console.log("Speech started, doing other work...");
```

## Example: TTS Pipeline

A complete text-to-speech pipeline using an external TTS service:

```typescript
import { Vrm, speech } from "@hmcs/sdk";

async function speak(character: Vrm, text: string) {
  // 1. Generate audio and phonemes from TTS service
  const response = await fetch("https://my-tts/synthesize", {
    method: "POST",
    body: JSON.stringify({ text }),
    headers: { "Content-Type": "application/json" },
  });
  const { audio, phonemes } = await response.json();

  // 2. Decode audio
  const wavData = Uint8Array.from(atob(audio), (c) => c.charCodeAt(0));

  // 3. Convert phonemes to keyframes
  const keyframes = speech.fromPhonemes(phonemes);

  // 4. Play with lip-sync
  await character.speakWithTimeline(wavData, keyframes);
}

const character = await Vrm.findByName("MyAvatar");
await speak(character, "Hello, nice to meet you!");
```

## Types

```typescript
interface TimelineKeyframe {
  duration: number;
  targets?: Record<string, number>;
}

interface SpeakTimelineOptions {
  waitForCompletion?: boolean;
  transitionDuration?: number;
}
```

## Next Steps

- **[Expressions](./expressions)** -- Understand the expression system that speech timelines drive.
- **[Events](./events)** -- Listen for animation and state events during speech.
- **[VRM Overview](./)** -- Full API reference table.
