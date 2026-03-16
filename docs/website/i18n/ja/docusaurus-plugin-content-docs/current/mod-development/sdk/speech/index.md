---
sidebar_position: 1
---

# speech

音素データを VRM リップシンク用のタイムラインキーフレームに変換するユーティリティです。`speech` モジュールは `vrm.speakWithTimeline()` に渡すキーフレーム配列を生成します。

## インポート

```typescript
import { speech } from "@hmcs/sdk";
```

## 関数

| 関数 | 説明 |
|----------|-------------|
| [fromPhonemes](./fromPhonemes) | `[expression_name, duration]` タプルのリストを `TimelineKeyframe[]` に変換します |

参照: [型定義](./types)
