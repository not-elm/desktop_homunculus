---
title: "コンテキストメニュー"
sidebar_position: 8
---

# コンテキストメニュー

MOD は、ユーザーが VRM キャラクターを右クリックしたときに表示されるコンテキストメニューに項目を追加できます。各メニュー項目は、ユーザーが選択したときに実行される [MOD コマンド](./commands.md)に対応します。コマンドは右クリックされたキャラクターのエンティティ（entity）ID を stdin 経由で受け取るため、特定のキャラクターに対して操作を行えます。

Desktop Homunculus が起動すると、インストール済みのすべての MOD の `package.json` から `homunculus.menus` 配列を読み取り、すべてのメニュー項目を登録します。ユーザーがキャラクターを右クリックすると、登録されたすべての項目がコンテキストメニューに表示されます。

## メニュー項目の宣言

`package.json` の `homunculus` フィールドに `menus` 配列を追加します。各エントリが 1 つのメニュー項目を定義します：

```json
{
  "name": "my-mod",
  "type": "module",
  "homunculus": {
    "menus": [
      {
        "id": "open-panel",
        "text": "Open Panel",
        "command": "open-panel"
      }
    ]
  },
  "bin": {
    "open-panel": "commands/open-panel.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```

### メニュー項目のフィールド

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `id` | `string` | MOD 内でのメニュー項目の一意な識別子 |
| `text` | `string` | 右クリックメニューに表示されるラベル |
| `command` | `string` | 選択時に実行する MOD コマンド（`bin` のキーと一致する必要あり） |

:::warning
`command` の値は `package.json` の `bin` フィールドのキーと正確に一致する必要があります。コマンドが見つからない場合、メニュー項目は表示されますがクリックしても何も起こりません。
:::

## メニューコマンドの処理 {#handling-menu-commands}

ユーザーがメニュー項目を選択すると、対応する MOD コマンドが実行され、右クリックされたキャラクターのエンティティ ID を含む JSON オブジェクトが stdin に渡されます：

```json
{ "linkedVrm": 42 }
```

`linkedVrm` フィールドは右クリックされた VRM キャラクターの数値エンティティ ID です。この ID を使ってキャラクターを検索し、操作を行えます。

この入力を解析してキャラクターに操作を行う最小限のコマンドハンドラーです：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { input } from "@hmcs/sdk/commands";

try {
  const character = await input.parseMenu();
  await character.setExpressions({ happy: 1.0 });
} catch (e) {
  console.error(e);
  process.exit(1);
}
```

`input.parseMenu()` は stdin から `{ "linkedVrm": ... }` の JSON を読み取り、右クリックされたキャラクターの `Vrm` インスタンスを返します。シバン、`input.parse`、エラー処理、出力規約の詳細は [MOD コマンド](./commands.md)を参照してください。

## WebView を開く

一般的なパターンとして、メニューコマンドから WebView パネルを開くことがあります。これにより、特定のキャラクターに関連付けられたリッチな UI 体験（設定パネル、ダッシュボードなど）を提供できます。

WebView を開くには、`package.json` で HTML アセットを宣言し、コマンドハンドラーで `Webview.open()` を使用します：

**`package.json`**（関連フィールド）：

```json
{
  "homunculus": {
    "menus": [
      {
        "id": "open-settings",
        "text": "Settings",
        "command": "open-ui"
      }
    ],
    "assets": {
      "my-mod:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "Settings panel UI"
      }
    }
  },
  "bin": {
    "open-ui": "commands/open-ui.ts"
  }
}
```

**`commands/open-ui.ts`**：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { Webview, webviewSource } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";

try {
  const vrm = await input.parseMenu();
  await Webview.open({
    source: webviewSource.local("my-mod:ui"),
    size: [1, 0.9],
    viewportSize: [900, 700],
    offset: [1.1, 0],
    linkedVrm: vrm.entity,
  });
} catch (e) {
  console.error(e);
}
```

`Webview.open()` の `linkedVrm` オプションは WebView を右クリックされたキャラクターに関連付けます。WebView 内では `Webview.current()` を使用してこの関連付けを取得し、`linkedVrm()` で VRM インスタンスにアクセスできます。

利用可能なすべてのオプションとメソッドについては [Webviews SDK リファレンス](/reference/sdk/webviews)を参照してください。

## 完全な例

この例では、右クリックされたキャラクターに嬉しい表情とアニメーションを再生させる「Wave」メニュー項目を追加します。

**`package.json`**：

```json
{
  "name": "my-wave",
  "version": "1.0.0",
  "type": "module",
  "homunculus": {
    "menus": [
      {
        "id": "wave",
        "text": "Wave",
        "command": "wave"
      }
    ]
  },
  "bin": {
    "wave": "commands/wave.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```

**`commands/wave.ts`**：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { repeat } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";

try {
  const character = await input.parseMenu();

  // 嬉しい表情を表示
  await character.setExpressions({ happy: 1.0 });

  // アイドルアニメーションを 1 回再生
  await character.playVrma({
    asset: "vrma:idle-maid",
    repeat: repeat.count(1),
    transitionSecs: 0.3,
  });
} catch (e) {
  console.error(e);
  process.exit(1);
}
```

インストールとテスト：

```bash
hmcs mod install /path/to/my-wave
```

Desktop Homunculus を再起動し、キャラクターを右クリックして、メニューから **Wave** を選択してください。

## 関連ページ

- **[トレイメニュー](./tray-menus.md)** -- アプリケーション全体のトレイメニュー項目
- **[MOD コマンド](./commands.md)** -- MOD コマンドの作成と呼び出し
- **[Webviews](/reference/sdk/webviews)** -- 3D 空間への HTML UI の埋め込み
- **[Mods API](/reference/sdk/mods/)** -- プログラムによるメニュー一覧取得とコマンド実行
- **[パッケージ設定](./project-setup/package-json.md)** -- 完全な `package.json` リファレンス
