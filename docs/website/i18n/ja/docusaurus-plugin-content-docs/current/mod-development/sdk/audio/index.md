---
sidebar_position: 1
---

# audio

効果音（SE）と BGM の再生。音量、フェード、再生制御に対応しています。効果音はワンショット再生で、BGM はトランスポートコントロール付きの連続再生です。

## インポート

```typescript
import { audio } from "@hmcs/sdk";
```

## 関数

| 関数 | 説明 |
|----------|-------------|
| [se.play](./se-play) | ワンショットの効果音を再生 |
| [bgm.play](./bgm-play) | BGM の再生を開始 |
| [bgm.stop](./bgm-stop) | 再生中の BGM を停止 |
| [bgm.pause](./bgm-pause) | 再生中の BGM を一時停止 |
| [bgm.resume](./bgm-resume) | 一時停止中の BGM を再開 |
| [bgm.update](./bgm-update) | 再生中の音量や速度を更新 |
| [bgm.status](./bgm-status) | 現在の BGM 再生ステータスを取得 |
