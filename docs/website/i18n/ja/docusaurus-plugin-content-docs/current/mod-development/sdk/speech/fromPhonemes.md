---
sidebar_position: 2
---

# fromPhonemes

`speech.fromPhonemes(phonemes)` は `[expression_name, duration]` タプルのリストを [`TimelineKeyframe`](./types#timelinekeyframe)`[]` に変換します。`null` の表情名は無音（口を閉じた）キーフレームを作成します。

各タプルは [`TimelineKeyframe`](./types#timelinekeyframe) にマッピングされます：

- `["aa", 0.1]` は `{ duration: 0.1, targets: { aa: 1.0 } }` になります
- `[null, 0.05]` は `{ duration: 0.05 }` になります（口の表情なし -- 無音）

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `phonemes` | `Array<[string \| null, number]>` | `[expression_name, duration_seconds]` タプルの配列 |

## 戻り値

[`TimelineKeyframe`](./types#timelinekeyframe)`[]`

## 例

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
