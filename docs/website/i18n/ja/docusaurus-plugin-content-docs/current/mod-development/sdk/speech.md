---
title: "speech"
sidebar_position: 6
---

# speech

音素データを VRM リップシンク用のタイムラインキーフレームに変換するユーティリティです。`speech` モジュールは `vrm.speakWithTimeline()` に渡すキーフレーム配列を生成します。

## インポート

```typescript
import { speech } from "@hmcs/sdk";
```

## 音素からタイムラインへ

`speech.fromPhonemes(phonemes)` は `[expression_name, duration]` タプルのリストを `TimelineKeyframe[]` に変換します。`null` の表情名は無音（口を閉じた）キーフレームを作成します。

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

各タプルは `TimelineKeyframe` にマッピングされます：

- `["aa", 0.1]` は `{ duration: 0.1, targets: { aa: 1.0 } }` になります
- `[null, 0.05]` は `{ duration: 0.05 }` になります（口の表情なし -- 無音）

### パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `phonemes` | `Array<[string \| null, number]>` | `[expression_name, duration_seconds]` タプルの配列 |

**戻り値：** `TimelineKeyframe[]`

## 型定義

### `TimelineKeyframe`

VRM モジュールで定義されています。各キーフレームは持続時間とオプションの表情ターゲットを指定します。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `duration` | `number` | 持続時間（秒） |
| `targets` | `Record<string, number>` | 表情名からウェイト（0--1）へのマッピング。無音の場合は省略します。 |

## 次のステップ

- **[VRM モジュール](./vrm/)** -- `speakWithTimeline()` を含む完全なキャラクター制御。VoiceVox のテキスト読み上げ合成については、`@hmcs/voicevox` MOD の bin コマンドを使用してください。
- **[オーディオ](./audio)** -- 効果音と BGM の再生
