---
title: "スピーチタイムライン"
sidebar_position: 5
---

# スピーチタイムライン

オーディオとフレーム同期された表情キーフレームを再生してリップシンクを行います。スピーチタイムラインシステムにより、任意の TTS エンジンが WAV オーディオとタイミング付きの表情ターゲットのシーケンスを送信することで、VRM の口のアニメーションを駆動できます。

## インポート

```typescript
import { Vrm, speech } from "@hmcs/sdk";
```

## タイムライン付きスピーチ

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

### パラメータ

| パラメータ | 型 | 説明 |
|---|---|---|
| `audio` | `ArrayBuffer \| Uint8Array` | WAV オーディオデータ |
| `keyframes` | `TimelineKeyframe[]` | タイミング付き表情ターゲットのシーケンス |
| `options` | `SpeakTimelineOptions` | オプション設定 |

### オプション

| オプション | 型 | デフォルト | 説明 |
|---|---|---|---|
| `waitForCompletion` | `boolean` | `true` | `true` の場合、スピーチ完了までブロックします |
| `transitionDuration` | `number` | `0.05` | キーフレーム間のスムースステップブレンディングの秒数。各キーフレームの持続時間の 40% にクランプされます。 |

## キーフレーム形式

各キーフレームは持続時間とオプションの表情ターゲットを指定します：

```typescript
interface TimelineKeyframe {
  /** このキーフレームの持続時間（秒） */
  duration: number;
  /** 設定する表情ウェイト。キーは表情名、値は 0.0-1.0。 */
  targets?: Record<string, number>;
}
```

`targets` のないキーフレームは、口の表情が適用されない無音のポーズを作成します：

```typescript
const keyframes: TimelineKeyframe[] = [
  { duration: 0.1, targets: { aa: 1.0 } },           // 「あ」の音
  { duration: 0.05 },                                  // 短いポーズ
  { duration: 0.12, targets: { oh: 1.0, happy: 0.5 }}, // 笑顔付き「お」
  { duration: 0.08, targets: { ee: 0.8 } },            // 「い」の音
];
```

:::note
表情ターゲットは口の形状に限定されません。キーフレームに `happy` や `surprised` などの感情表現を含めて、オーディオの特定の瞬間に表情の反応を同期させることができます。
:::

## トランジションブレンディング

`transitionDuration` オプションは隣接するキーフレーム間の表情のブレンドの滑らかさを制御します。デフォルトは 50ms で、自然な見た目のトランジションを生成します。

```typescript
// よりスムーズなトランジション（100ms ブレンド）
await character.speakWithTimeline(wavData, keyframes, {
  transitionDuration: 0.1,
});

// よりキビキビしたトランジション（20ms ブレンド）
await character.speakWithTimeline(wavData, keyframes, {
  transitionDuration: 0.02,
});
```

トランジション時間は各キーフレームの持続時間の 40% に自動的にクランプされるため、短いキーフレームがトランジションに侵食されることはありません。

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

const character = await Vrm.findByName("MyAvatar");
await character.speakWithTimeline(wavData, keyframes);
```

各タプルは `[expression_name | null, duration_in_seconds]` です。`null` の表情は無音のキーフレームを作成します。

## ノンブロッキングスピーチ

デフォルトでは、`speakWithTimeline` はオーディオの完了を待ちます。`waitForCompletion: false` を設定すると、すぐに実行を継続します：

```typescript
// ブロックせずにスピーチを開始
await character.speakWithTimeline(wavData, keyframes, {
  waitForCompletion: false,
});

// キャラクターがまだ話している間にこれが実行されます
console.log("Speech started, doing other work...");
```

## 例：TTS パイプライン

外部 TTS サービスを使用した完全なテキスト読み上げパイプライン：

```typescript
import { Vrm, speech } from "@hmcs/sdk";

async function speak(character: Vrm, text: string) {
  // 1. TTS サービスからオーディオと音素を生成
  const response = await fetch("https://my-tts/synthesize", {
    method: "POST",
    body: JSON.stringify({ text }),
    headers: { "Content-Type": "application/json" },
  });
  const { audio, phonemes } = await response.json();

  // 2. オーディオをデコード
  const wavData = Uint8Array.from(atob(audio), (c) => c.charCodeAt(0));

  // 3. 音素をキーフレームに変換
  const keyframes = speech.fromPhonemes(phonemes);

  // 4. リップシンク付きで再生
  await character.speakWithTimeline(wavData, keyframes);
}

const character = await Vrm.findByName("MyAvatar");
await speak(character, "Hello, nice to meet you!");
```

## 型定義

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

## 次のステップ

- **[表情](./expressions)** -- スピーチタイムラインが駆動する表情システムを理解します。
- **[イベント](./events)** -- スピーチ中のアニメーションと状態イベントをリッスンします。
- **[VRM 概要](./)** -- 完全な API リファレンス表。
