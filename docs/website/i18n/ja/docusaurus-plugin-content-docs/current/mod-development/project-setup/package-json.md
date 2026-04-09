---
title: "パッケージ設定"
sidebar_position: 2
---

# パッケージ設定

すべての MOD は `package.json` ファイルを持つ npm パッケージです。標準的な npm フィールドに加えて、MOD はアセット、メニュー、その他のエンジン固有のメタデータを宣言するための `homunculus` フィールドを使用します。

## 概要

MOD の `package.json` には以下が含まれます：

| フィールド | 用途 | 必須 |
|---|---|---|
| `name` | パッケージ名（アセット ID の導出に使用） | はい |
| `type` | ES モジュールサポートのため `"module"` であること | はい |
| `bin` | MOD コマンド（HTTP API 経由で呼び出し） | いいえ |
| `homunculus` | エンジンメタデータ：サービス、アセット、メニュー、トレイ | はい |
| `dependencies` | SDK 機能使用時は `@hmcs/sdk` を含む必要あり | いいえ |

## `homunculus` フィールド

`homunculus` フィールドがパッケージを MOD にします。4 つのサブフィールドがあります：

### `assets`

MOD にバンドルされるファイルを宣言します。各エントリは**アセット ID** をファイルの説明に対応付けます。

```json
{
  "homunculus": {
    "assets": {
      "<asset-id>": {
        "path": "<ファイルへの相対パス>",
        "type": "<アセットタイプ>",
        "description": "<人間が読める説明>"
      }
    }
  }
}
```

**サポートされるアセットタイプ：**

| タイプ | ファイル形式 | 説明 |
|---|---|---|
| `vrm` | `.vrm` | 3D キャラクターモデル（VRM 1.0） |
| `vrma` | `.vrma` | VRM アニメーションクリップ |
| `sound` | `.mp3`, `.wav`, `.ogg` | 効果音またはオーディオファイル |
| `image` | `.png`, `.jpg`, `.svg` | 画像ファイル |
| `html` | `.html` | WebView UI エントリポイント |

**例** -- デスクトップキャラクターを生成する `@hmcs/elmer` MOD：

```json
{
  "homunculus": {
    "assets": {
      "elmer:vrm": {
        "path": "assets/Elmer.vrm",
        "type": "vrm",
        "description": "VRM model named Elmer"
      },
      "elmer:open": {
        "path": "assets/open.mp3",
        "type": "sound",
        "description": "Sound effect for opening action"
      }
    }
  }
}
```

:::info[アセット ID]
アセット ID はグローバルに一意でなければなりません。推奨される命名規約は `<mod-name>:<asset-name>` です。詳細は[アセット ID](./asset-ids.md)を参照してください。
:::

### `menus`

右クリックコンテキストメニューのエントリを宣言します。各メニューエントリはクリック時に `bin` コマンドをトリガーします。

```json
{
  "homunculus": {
    "menus": [
      {
        "id": "<一意な ID>",
        "text": "<表示ラベル>",
        "command": "<bin コマンド名>"
      }
    ]
  }
}
```

**例** -- `@hmcs/character-settings` MOD はコンテキストメニューに「Character Settings」エントリを追加します：

```json
{
  "homunculus": {
    "menus": [
      {
        "id": "open-character-settings",
        "text": "Character Settings",
        "command": "open-ui"
      }
    ]
  }
}
```

ユーザーがキャラクターを右クリックして「Character Settings」を選択すると、エンジンは `open-ui` MOD コマンドを呼び出します。

### `tray`

システムトレイメニューの項目を宣言します。`menus`（右クリック時に表示）とは異なり、トレイ項目は OS メニューバー / システムトレイアイコンメニューに表示され、アプリケーション全体に適用されます。

```json
{
  "homunculus": {
    "tray": {
      "id": "<一意な ID>",
      "text": "<表示ラベル>",
      "command": "<bin コマンド名>"
    }
  }
}
```

トレイ項目はサブメニュー用にネストされた `items` を含めることもできます。完全なガイドは[トレイメニュー](../tray-menus.md)を参照してください。

**例** -- `@hmcs/settings` MOD はトレイに「Settings」エントリを追加します：

```json
{
  "homunculus": {
    "tray": {
      "id": "open-settings",
      "text": "Settings",
      "command": "settings-open-ui"
    }
  }
}
```

### `service`

`homunculus.service` フィールドは**サービス**を指定します。これは Desktop Homunculus の起動時に自動実行される長時間稼働の Node.js プロセスです。エンジンは `tsx` を使って子プロセスとして実行するため、ビルドステップなしで TypeScript を直接記述できます。

```json
{
  "homunculus": {
    "service": "service.ts"
  }
}
```

サービスは通常、VRM キャラクターの生成と動作の設定に使用されます。サービスは起動時に開始され、アプリの実行中は稼働し続けます。

:::warning
サービススクリプトはアプリが起動するたびに実行されます。エラーを適切に処理してください。未処理の例外が発生するとスクリプトプロセスが終了します。
:::

## MOD コマンド {#mod-commands}

`bin` フィールドは HTTP API 経由で呼び出せる MOD コマンドを公開します。サービススクリプトとは異なり、これらのスクリプトは明示的に呼び出された場合にのみ実行されます。

```json
{
  "bin": {
    "<コマンド名>": "<スクリプトへのパス>"
  }
}
```

MOD コマンドは JSON ボディを持つ `POST /mods/{mod_name}/bin/{command}` 経由で呼び出されます。スクリプトは `@hmcs/sdk/commands` の `input.parse` を使って stdin から入力を受け取ります。

**例** -- `@hmcs/voicevox` MOD は TTS コマンドを公開します：

```json
{
  "bin": {
    "voicevox:speak": "commands/speak.ts",
    "voicevox:speakers": "commands/speakers.ts",
    "voicevox:initialize": "commands/initialize.ts"
  }
}
```

:::tip
MOD コマンド名は慣例的に MOD 名のプレフィックスを付けます（例：`voicevox:speak`）。これは他の MOD との衝突を避けるためです。
:::

## 依存関係

SDK を使用する場合は、`@hmcs/sdk` を依存関係に含める必要があります。

```json
{
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```
