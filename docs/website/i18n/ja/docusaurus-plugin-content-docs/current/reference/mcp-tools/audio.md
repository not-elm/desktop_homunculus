---
title: "オーディオ"
sidebar_position: 4
---

# オーディオ

オーディオツールはスピーチ、効果音、BGM を扱います。

#### `speak_message`

VoiceVox テキスト読み上げを使用して、アクティブキャラクターにテキストを発話させます。

:::note
VoiceVox MOD がインストールされ、VoiceVox エンジンが実行中である必要があります。
:::

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `text` | `string \| string[]` | **必須** | 発話するテキスト。より確実な合成のために短い文の配列を渡してください。 |
| `speaker` | `number` | `0` | VoiceVox の話者 ID |
| `timeoutMs` | `number` | `30000` | タイムアウト（ミリ秒、範囲：1000--120000） |

**例：**

```json
{
  "text": ["Hello!", "How can I help you today?"],
  "speaker": 3,
  "timeoutMs": 15000
}
```

---

#### `play_sound`

効果音アセットを再生します。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `sound` | `string` | **必須** | サウンドアセット ID（例：`se:open`） |
| `volume` | `number` | `0.8` | 音量レベル（範囲：0.0--1.0） |

---

#### `control_bgm`

BGM の再生を制御します。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `action` | `"play" \| "stop" \| "pause" \| "resume" \| "status"` | **必須** | 実行するアクション |
| `asset` | `string` | -- | MOD アセット ID -- `action` が `"play"` の場合は必須 |
| `volume` | `number` | -- | 音量レベル（範囲：0.0--1.0） |

`"status"` アクションは現在の再生状態を JSON として返します。
