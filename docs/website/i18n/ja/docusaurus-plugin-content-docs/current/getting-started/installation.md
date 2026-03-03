---
title: "インストール"
sidebar_position: 2
---

# インストール

Desktop Homunculus のインストールと MOD 環境のセットアップ手順です。

## システム要件

| | macOS |
|---|---|
| **OS** | macOS 12 (Monterey) 以降 |
| **CPU** | Apple Silicon または Intel |
| **Node.js** | 22 以上 |
| **ディスク容量** | 500 MB 以上 |

:::info[Windows サポート]
Windows サポートは予定されています。現在は macOS のみ対応しています。
:::

## Step 1: Desktop Homunculus のインストール

[GitHub リリースページ](https://github.com/not-elm/desktop_homunculus/releases)から最新版をダウンロードしてください。

1. `.dmg` ファイルをダウンロード
2. DMG を開き、**Desktop Homunculus** を `/Applications` フォルダにドラッグ
3. アプリケーションフォルダから起動

:::warning[Gatekeeper の警告]
「"Desktop Homunculus"は開発元を確認できないため開けません」と表示された場合:

1. **システム設定** > **プライバシーとセキュリティ** を開く
2. 下にスクロールし、**このまま開く** をクリック
3. 確認ダイアログで **開く** をクリック
:::

## Step 2: Node.js のインストール

Desktop Homunculus の MOD は `--experimental-strip-types` を使用して TypeScript スクリプトを直接実行するため、**Node.js 22 以上** が必要です。

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
hmcs mod install @hmcs/assets @hmcs/elmer @hmcs/menu @hmcs/settings
```

| MOD | 説明 |
|---|---|
| `@hmcs/assets` | デフォルトのアニメーションと効果音 |
| `@hmcs/elmer` | デフォルトのキャラクターモデル |
| `@hmcs/menu` | 右クリックコンテキストメニュー |
| `@hmcs/settings` | 設定 UI パネル |

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

MOD は `--experimental-strip-types` を使用して TypeScript を直接実行するため、Node.js 22 以上が必要です。[nodejs.org](https://nodejs.org/download) から最新の LTS バージョンをダウンロードしてください。

### Gatekeeper でアプリがブロックされる

[Step 1](#step-1-desktop-homunculus-のインストール)の手順を参照してください。

### MOD インストール後にキャラクターが表示されない

Desktop Homunculus を再起動してください。MOD の変更はアプリの再起動後に反映されます。

### MOD のインストールに失敗する

ネットワーク接続を確認して再試行してください。
