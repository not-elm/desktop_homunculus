---
title: "VoiceVox"
sidebar_position: 6
---

# VoiceVox

VoiceVox MOD（`@hmcs/voicevox`）は、[VoiceVox](https://voicevox.hiroshiba.jp/) 音声合成エンジンを Desktop Homunculus と連携させます。キャラクターがリップシンク付きの音声で話せるようになります。

## 概要

この MOD はローカルで動作している VoiceVox エンジンに接続して音声を合成します。呼び出されると、テキストを VoiceVox API に送信し、合成された音声を受け取り、キャラクターモデル上で自動リップシンクとともに再生します。

## 前提条件

1. **VoiceVox をダウンロードしてインストール** — [voicevox.hiroshiba.jp](https://voicevox.hiroshiba.jp/) から入手
2. **VoiceVox エンジンを起動** — 音声機能を使用する前に起動しておく必要があります
3. **VoiceVox MOD をインストール：**

```shell
hmcs mod install @hmcs/voicevox
```

## 機能

VoiceVox MOD は、他の MOD や MCP ツールから呼び出せる 3 つの [bin コマンド](/docs/mod-development/bin-commands)を提供します：

### `voicevox:speak`

キャラクターにリップシンク付き音声でテキストを読み上げさせます。

**パラメータ：**

| パラメータ | 型 | 必須 | デフォルト | 説明 |
|---|---|---|---|---|
| `entity` | number | はい | - | キャラクターのエンティティ（entity）ID |
| `text` | string または string[] | はい | - | 読み上げるテキスト（単一文字列または文の配列） |
| `speaker` | number | いいえ | `0` | VoiceVox のスピーカー ID |
| `voicevox_host` | string | いいえ | `http://localhost:50021` | VoiceVox エンジンの URL |
| `speed_scale` | number | いいえ | - | 話速の倍率 |
| `pitch_scale` | number | いいえ | - | ピッチの倍率 |
| `intonation_scale` | number | いいえ | - | イントネーションの倍率 |
| `volume_scale` | number | いいえ | - | 音量の倍率 |
| `fetch_timeout_ms` | number | いいえ | `30000` | VoiceVox API リクエストのタイムアウト（ミリ秒） |

### `voicevox:speakers`

利用可能な VoiceVox スピーカー（ボイス）の一覧を取得します。

**パラメータ：**

| パラメータ | 型 | 必須 | デフォルト | 説明 |
|---|---|---|---|---|
| `voicevox_host` | string | いいえ | `http://localhost:50021` | VoiceVox エンジンの URL |

### `voicevox:initialize`

スピーカーモデルを事前にロードして、初回の音声合成を高速化します。

**パラメータ：**

| パラメータ | 型 | 必須 | デフォルト | 説明 |
|---|---|---|---|---|
| `speaker` | number | いいえ | `0` | 初期化する VoiceVox スピーカー ID |
| `voicevox_host` | string | いいえ | `http://localhost:50021` | VoiceVox エンジンの URL |

## トラブルシューティング

### VoiceVox エンジンに接続できない

**症状：** コマンドが `VOICEVOX_UNREACHABLE` エラーで失敗する。

**解決策：** VoiceVox アプリケーションが起動していることを確認してください。デフォルトでは `http://localhost:50021` でリッスンしています。ポートを変更した場合は、`voicevox_host` パラメータを指定してください。

### 音声が遅い

**症状：** 音声が始まるまでに顕著な遅延がある。

**原因：** 各発話ごとに VoiceVox が再生前に音声を合成する必要があります。テキストが長いほど時間がかかります。

**ヒント：** VoiceVox セッションごとに一度 `voicevox:initialize` を使用してスピーカーモデルを事前にロードすると、初回合成のレイテンシが低減します。また、長いテキストを短い文に分割すると、段階的な再生が高速化します。
