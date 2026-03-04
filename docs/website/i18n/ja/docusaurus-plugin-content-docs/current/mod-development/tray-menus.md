---
title: "トレイメニュー"
sidebar_position: 9
---

# トレイメニュー

MOD は**システムトレイ（system tray）メニュー**に項目を追加できます。これは、ユーザーが OS メニューバー（macOS）またはシステムトレイ（Windows）の Desktop Homunculus アイコンをクリックしたときに表示されるメニューです。特定のキャラクターに紐づく[コンテキストメニュー](./menus.md)とは異なり、トレイメニューはアプリケーション全体に適用され、設定パネルを開くなどのグローバルな操作に適しています。

Desktop Homunculus が起動すると、インストール済みのすべての MOD の `package.json` から `homunculus.tray` フィールドを読み取り、すべてのトレイメニュー項目を登録します。ユーザーがトレイアイコンをクリックすると、登録されたすべての項目がメニューに表示されます。

## トレイメニュー項目の宣言

`package.json` の `homunculus` フィールドに `tray` オブジェクトを追加します。各エントリが 1 つのメニュー項目を定義します：

```json
{
  "name": "my-mod",
  "type": "module",
  "homunculus": {
    "tray": {
      "id": "open-panel",
      "text": "Open Panel",
      "command": "open-panel"
    }
  },
  "bin": {
    "open-panel": "commands/open-panel.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```

### トレイ項目のフィールド

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `id` | `string` | MOD 内でのトレイ項目の一意な識別子 |
| `text` | `string` | トレイメニューに表示されるラベル |
| `command` | `string` | 選択時に実行する bin コマンド（`bin` のキーと一致する必要あり） |
| `items` | `TrayItem[]` | （オプション）サブメニューを作成するためのネストされた子項目 |

:::warning
`command` の値は `package.json` の `bin` フィールドのキーと正確に一致する必要があります。コマンドが見つからない場合、メニュー項目は表示されますがクリックしても何も起こりません。
:::

## サブメニュー

`items` フィールドを使って項目をネストすることでサブメニューを作成できます。`items` を持つ親項目はサブメニューのコンテナとして機能し、`command` は不要です。

```json
{
  "homunculus": {
    "tray": {
      "id": "tools",
      "text": "Tools",
      "items": [
        {
          "id": "tool-a",
          "text": "Tool A",
          "command": "run-tool-a"
        },
        {
          "id": "tool-b",
          "text": "Tool B",
          "command": "run-tool-b"
        }
      ]
    }
  }
}
```

## トレイコマンドの処理

コンテキストメニューのコマンドがキャラクターのエンティティを stdin 経由で受け取るのとは異なり、トレイコマンドは **stdin 入力を受け取りません**。単純なファイア・アンド・フォーゲット（fire-and-forget）スクリプトとして実行されます。これは、トレイ操作が特定のキャラクターに紐づかないアプリケーション全体のものだからです。

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";

try {
  await Webview.open({
    source: webviewSource.local("my-mod:ui"),
    size: [0.6, 0.6],
    viewportSize: [500, 400],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e);
}
```

## トレイメニュー vs. コンテキストメニュー

| 特徴 | トレイメニュー | コンテキストメニュー |
|---------|-----------|--------------|
| トリガー | トレイアイコンのクリック | キャラクターの右クリック |
| スコープ | アプリケーション全体 | キャラクター単位 |
| stdin 入力 | なし | `{ "linkedVrm": <entity> }` |
| 宣言 | `homunculus.tray`（単一オブジェクト） | `homunculus.menus`（配列） |
| サブメニュー | `items` でサポート | 非対応 |

## 完全な例

この例では、アプリケーション設定パネルを開く「Settings」エントリをシステムトレイに追加します。

**`package.json`**：

```json
{
  "name": "@hmcs/settings",
  "version": "1.0.0",
  "type": "module",
  "homunculus": {
    "tray": {
      "id": "open-settings",
      "text": "Settings",
      "command": "settings-open-ui"
    },
    "assets": {
      "settings:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "Application settings panel UI"
      }
    }
  },
  "bin": {
    "settings-open-ui": "commands/open-ui.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "workspace:*"
  }
}
```

**`commands/open-ui.ts`**：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";

try {
  await Webview.open({
    source: webviewSource.local("settings:ui"),
    size: [0.6, 0.6],
    viewportSize: [500, 400],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e);
}
```

## 関連ページ

- **[コンテキストメニュー](./menus.md)** -- キャラクターの右クリックメニュー
- **[Bin コマンド](./bin-commands.md)** -- オンデマンドスクリプトの作成と呼び出し
- **[Webviews](./sdk/webviews)** -- 3D 空間への HTML UI の埋め込み
- **[パッケージ設定](./project-setup/package-json.md)** -- 完全な `package.json` リファレンス
