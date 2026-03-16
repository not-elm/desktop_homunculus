---
title: "speakWithTimeline"
sidebar_position: 40
---

# speakWithTimeline

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.speakWithTimeline(audio, keyframes, options?)` は WAV オーディオを再生しながら、正確なタイムスタンプで表情キーフレームを適用します。エンジンが口の動きとオーディオ再生を同期させます。

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

## パラメータ

| パラメータ  | 型                         | 説明                             |
| ---------- | -------------------------- | --------------------------------------- |
| `audio`    | `ArrayBuffer \| Uint8Array` | WAV オーディオデータ                          |
| `keyframes`| [`TimelineKeyframe`](./types#timelinekeyframe)`[]`       | タイミング付き表情ターゲットのシーケンス    |
| `options`  | [`SpeakTimelineOptions`](./types#speaktimelineoptions)     | オプション設定                       |

## オプション

| オプション               | 型        | デフォルト | 説明                                                                                          |
| -------------------- | --------- | ------- | ---------------------------------------------------------------------------------------------------- |
| `waitForCompletion`  | `boolean` | `true`  | `true` の場合、スピーチ完了までブロックします                                                 |
| `transitionDuration` | `number`  | `0.05`  | キーフレーム間のスムースステップブレンディングの秒数。各キーフレームの持続時間の 40% にクランプされます。      |

## キーフレーム形式

各キーフレームは持続時間とオプションの表情ターゲットを指定します：

```typescript
const keyframes: TimelineKeyframe[] = [
  { duration: 0.1, targets: { aa: 1.0 } },           // 「あ」の音
  { duration: 0.05 },                                  // 短いポーズ
  { duration: 0.12, targets: { oh: 1.0, happy: 0.5 }}, // 笑顔付き「お」
  { duration: 0.08, targets: { ee: 0.8 } },            // 「い」の音
];
```

`targets` のないキーフレームは、口の表情が適用されない無音のポーズを作成します。

:::note
表情ターゲットは口の形状に限定されません。キーフレームに `happy` や `surprised` などの感情表現を含めて、オーディオの特定の瞬間に表情の反応を同期させることができます。
:::

## ノンブロッキングスピーチ

デフォルトでは、`speakWithTimeline` はオーディオの完了を待ちます。`waitForCompletion: false` を設定すると、すぐに実行を継続します：

```typescript
await character.speakWithTimeline(wavData, keyframes, {
  waitForCompletion: false,
});
```

## 音素からキーフレームを構築

`speech` モジュールは、簡単な音素リストをタイムラインキーフレームに変換するヘルパーを提供します：

```typescript
import { speech, Vrm } from "@hmcs/sdk";

const keyframes = speech.fromPhonemes([
  ["aa", 0.1],     // 「あ」100ms
  [null, 0.05],    // 無音 50ms
  ["oh", 0.12],    // 「お」120ms
  ["ee", 0.08],    // 「い」80ms
]);

await character.speakWithTimeline(wavData, keyframes);
```

各タプルは `[expression_name | null, duration_in_seconds]` です。`null` の表情は無音のキーフレームを作成します。
