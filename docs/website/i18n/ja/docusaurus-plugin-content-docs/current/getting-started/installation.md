---
title: "インストール"
sidebar_position: 2
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# インストール

Desktop Homunculus のインストールと MOD 環境のセットアップ手順です。

## システム要件

|                  | macOS                      | Windows         |
| ---------------- | -------------------------- | --------------- |
| **OS**           | macOS 12 (Monterey) 以降   | Windows 10 以降 |
| **CPU**          | Apple Silicon または Intel | x86_64          |
| **Node.js**      | 22 以上                    | 22 以上         |
| **ディスク容量** | 500 MB 以上                | 500 MB 以上     |

## Step 1: Desktop Homunculus のインストール

[GitHub リリースページ](https://github.com/not-elm/desktop-homunculus/releases)から最新版をダウンロードしてください。

<Tabs>
<TabItem value="macos" label="macOS" default>

1. `.dmg` ファイルをダウンロード
2. DMG を開き、**Desktop Homunculus** を `/Applications` フォルダにドラッグ
3. アプリケーションフォルダから起動

:::warning[Gatekeeper の警告]
「"Desktop Homunculus"は開発元を確認できないため開けません」と表示された場合:

1. **システム設定** > **プライバシーとセキュリティ** を開く
2. 下にスクロールし、**このまま開く** をクリック
3. 確認ダイアログで **開く** をクリック
   :::

</TabItem>
<TabItem value="windows" label="Windows">

:::caution[NVIDIA GPU ユーザー — 初回起動前に必須の設定]
NVIDIA GPU を搭載している場合、Desktop Homunculus を起動する**前に**以下の設定を行う必要があります。設定しないとウィンドウの背景が透明にならず黒くなります:

1. **NVIDIA コントロールパネル** を開く
2. **3D 設定の管理** に移動
3. **「Vulkan/OpenGL present method」** を探す
4. **「Prefer native」** に設定
5. **適用** をクリック

この設定は**初回起動前に**行う必要があります。
:::

1. `.msi` ファイルをダウンロード
2. インストーラーを実行し、指示に従う
3. スタートメニューから **Desktop Homunculus** を起動

</TabItem>
</Tabs>

## Step 2: Node.js のインストール

Desktop Homunculus の MOD は `tsx` を使用して TypeScript スクリプトを直接実行するため、**Node.js 22 以上** が必要です。

1. [Node.js ダウンロードページ](https://nodejs.org/ja/download)にアクセス
2. **LTS** バージョン（22 以上）をダウンロードしてインストール
3. インストールを確認:

```shell
node -v
```

`v22.0.0` 以上が表示されれば OK です。

:::tip
すでに Node.js がインストールされている場合は `node -v` でバージョンを確認してください。v22 未満の場合は最新の LTS バージョンに更新してください。
:::

## Step 3: MOD 管理ツールのインストール

**pnpm**（パッケージマネージャー）と **@hmcs/cli**（MOD 管理 CLI）をグローバルインストールします:

```shell
npm install -g pnpm @hmcs/cli
```

インストールを確認:

```shell
pnpm -v
hmcs --version
```

:::warning[権限エラー]
`EACCES` 権限エラーが出た場合は、[npm のドキュメント（権限エラーの解決方法）](https://docs.npmjs.com/resolving-eacces-permissions-errors-when-installing-packages-globally)を参照してください。
:::

## Step 4: 公式 MOD のインストール

推奨される公式 MOD をインストールします:

```shell
hmcs mod install @hmcs/assets @hmcs/persona @hmcs/menu @hmcs/settings @hmcs/app-exit
```

| MOD                        | 説明                                                                     |
| -------------------------- | ------------------------------------------------------------------------ |
| `@hmcs/assets`             | デフォルトのアニメーションと効果音                                       |
| `@hmcs/persona`            | ペルソナ管理 — 自動生成、アニメーション、設定 UI                         |
| `@hmcs/menu`               | 右クリックコンテキストメニュー                                           |
| `@hmcs/settings`           | システムトレイからのアプリケーション設定（フレームレート、影の不透明度） |
| `@hmcs/app-exit`           | システムトレイの終了メニュー                                             |

## Step 5: 動作確認

1. **Desktop Homunculus** を起動
2. デスクトップにキャラクターが表示される
3. キャラクターを右クリックしてコンテキストメニューが開く

すべて正常に動作すれば、セットアップ完了です!

:::tip[次のステップ]
[クイックスタート](./quick-start.md)ガイドに進んで、キャラクターとの基本的なやり取り方法を学びましょう。
:::

## トラブルシューティング

### `hmcs: command not found`

ターミナルが `hmcs` コマンドを認識できていません。

- **ターミナルを再起動**してください — PATH が更新されていない可能性があります
- npm のグローバル bin ディレクトリが PATH に含まれているか確認:
  ```shell
  npm bin -g
  ```

### Node.js のバージョンが 22 未満

MOD は `tsx` を使用して TypeScript を直接実行するため、Node.js 22 以上が必要です。[nodejs.org](https://nodejs.org/download) から最新の LTS バージョンをダウンロードしてください。

### Gatekeeper でアプリがブロックされる

[Step 1](#step-1-desktop-homunculus-のインストール)の手順を参照してください。

### Windows でウィンドウの背景が黒い/不透明になる

Windows でウィンドウの背景が透明にならず黒く表示される場合、NVIDIA GPU の設定変更が必要です。Step 1 の [NVIDIA GPU の設定手順](#step-1-desktop-homunculus-のインストール)（Windows タブ）を参照してください。

### MOD インストール後にキャラクターが表示されない

Desktop Homunculus を再起動してください。MOD の変更はアプリの再起動後に反映されます。

### MOD のインストールに失敗する

ネットワーク接続を確認して再試行してください。
