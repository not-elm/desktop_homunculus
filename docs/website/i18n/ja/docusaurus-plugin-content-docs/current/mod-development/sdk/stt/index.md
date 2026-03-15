---
title: "STT"
sidebar_position: 1
---

# STT モジュール

ローカルの Whisper モデルを使用したリアルタイム音声認識（Speech-to-Text）です。すべての処理はデバイス上で実行され、クラウドサービスや API キーは不要です。

```typescript
import { stt } from "@hmcs/sdk";
```

## クイックスタート

```typescript
import { stt } from "@hmcs/sdk";

// 1. モデルがダウンロード済みであることを確認
await stt.models.download({ modelSize: "small" });

// 2. セッションを開始
await stt.session.start({ language: "ja" });

// 3. 文字起こし結果をストリーミング
const stream = stt.stream({
  onResult: (result) => {
    console.log(`[${result.language}] ${result.text}`);
  },
  onSessionError: (err) => {
    console.error(`STT エラー: ${err.message}`);
  },
});

// 4. 完了したら停止
await stt.session.stop();
stream.close();
```

## 仕組み

STT パイプラインは [Whisper](https://github.com/openai/whisper) モデルを使用してデバイス上で完結し、次の 3 段階で構成されます。

1. **キャプチャ** — マイクの音声を専用スレッドで取得
2. **VAD** — 音声区間検出（Voice Activity Detection）により無音をフィルタリングし、発話区間のみを後段に送信
3. **推論** — Whisper が発話区間を処理し、文字起こし結果を出力

セッションは `stt.session` で管理し、結果は `stt.stream()` を通じてリアルタイムに受信できます。

## モデルサイズ

| サイズ | ダウンロード | 速度 | 精度 | 備考 |
|--------|-------------|------|------|------|
| `"tiny"` | 32.2 MB | 最速 | 低め | プロトタイプの素早い検証に |
| `"base"` | 59.7 MB | 高速 | 中程度 | 単純なタスクに |
| `"small"` | 190 MB | 中程度 | 良好 | **デフォルト。** 多くのユースケースに最適 |
| `"medium"` | 539 MB | やや遅い | 高い | 高精度、リソース使用量が多い |
| `"large-v3-turbo"` | 574 MB | やや遅い | より高い | Large v3 に近い精度で高速な推論 |
| `"large-v3"` | 1.08 GB | 最も遅い | 最高 | 最高精度、リソース使用量が最も多い |

## 前提条件

:::info[マイクのアクセス許可]
STT にはマイクへのアクセスが必要です。macOS では初回使用時にシステムから許可を求められます。マイクが利用できない場合、`stt.session.start()` はエラーコード `no_microphone` または `microphone_permission_denied` のエラーをスローします。
:::

## 次のステップ

- **[セッションとストリーミング](./session-and-streaming)** -- セッションのライフサイクル、リアルタイム文字起こしイベント、エラーハンドリング
- **[モデル](./models)** -- Whisper モデルのダウンロードと管理
- **[speech](../speech)** -- 音素データをリップシンク用キーフレームに変換（補完的な出力機能）
- **[audio](../audio)** -- 効果音と BGM の再生
