---
title: "MOD とは？"
sidebar_position: 1
---

# MOD とは？

MOD は、Desktop Homunculus に新しいキャラクター、動作、UI パネル、インテグレーションを追加する拡張パッケージです。カスタム 3D アバターの追加、音声合成サービスの接続、インタラクティブな設定パネルの構築など、MOD を通じて実現できます。

MOD は標準的な npm パッケージです。npm レジストリに公開し、他のユーザーと共有し、1 つのコマンドでインストールできます。特別なツールやカスタムフォーマットは不要です。npm パッケージの作り方を知っていれば、基本はすでに身についています。

## MOD の仕組み

すべての MOD は、`package.json` に特別な `homunculus` フィールドを持つ **pnpm パッケージ**です。このフィールドで MOD のアセット（3D モデル、アニメーション、サウンド、HTML ファイル）やオプションのメニューエントリを宣言します。

Desktop Homunculus が起動すると、エンジンは MOD ディレクトリ（`~/.homunculus/mods/`）で `pnpm ls` を実行し、各 MOD の `package.json` を読み取ることでインストール済みの MOD を検出します。MOD は `hmcs mod install` コマンドでインストールします。各 MOD は以下を宣言できます：

- **サービス**（`homunculus.service` フィールド）-- アプリ起動時に自動実行される長時間稼働の Node.js 子プロセス
- **オンデマンドコマンド**（`bin` フィールド）-- 必要に応じて HTTP API 経由で呼び出されるスクリプト
- **アセット**（`homunculus.assets` フィールド）-- MOD にバンドルされるファイル（VRM モデル、アニメーション、サウンド、UI）

MOD は `localhost:3100` で動作するローカル **HTTP API** を通じてエンジンと通信します。TypeScript SDK（`@hmcs/sdk`）がこの API を高レベルで型安全なインターフェースでラップしています。スクリプトは `node --experimental-strip-types` で実行されるため、ビルドステップなしで TypeScript を直接記述できます。

## MOD でできること

MOD は以下の機能を自由に組み合わせられます。1 つの MOD でこれらのうち 1 つだけを行うことも、すべてを同時に行うことも可能です。

- **キャラクターの生成** — VRM 3D モデルを読み込み、アニメーション、表情、動作を制御します。`@hmcs/elmer` MOD はデスクトップ上でアイドル状態になり、ドラッグに反応するデフォルトキャラクターを生成します。

- **サービスの実行** — アプリ起動時に長時間稼働する TypeScript プロセスを実行します（`homunculus.service` フィールドで宣言）。サービスは通常、キャラクターやイベントリスナーのセットアップに使用されます。`@hmcs/menu` MOD はサービスを使って右クリックメニューのオーバーレイを初期化します。

- **オンデマンドコマンドの公開** — 他の MOD や AI エージェントが HTTP API 経由で呼び出せるコマンドを提供します（`bin` フィールドで宣言）。`@hmcs/voicevox` MOD は音声合成用の `voicevox:speak` と `voicevox:speakers` コマンドを公開しています。

- **UI パネルの埋め込み** — WebView ベースのインターフェース（React + Vite）を HTML アセットとしてバンドルします。`@hmcs/character-settings` MOD は HTML アセット、パネルを開く `bin` コマンド、メニューエントリを組み合わせており、機能の連携例を示しています。

- **メニューエントリの追加** — 右クリックコンテキストメニューにコマンドのトリガーや WebView を開く項目を登録します（`homunculus.menus` で宣言）。

- **アセットのバンドル** — VRM モデル、アニメーション（VRMA）、サウンド、画像、HTML ファイルをパッケージ化し、他の MOD がアセット ID で参照できるようにします。

## 簡単な例

VRM キャラクターを読み込む MOD の `package.json` です：

```json
{
  "name": "@hmcs/elmer",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@hmcs/sdk": "workspace:*"
  },
  "homunculus": {
    "service": "index.ts",
    "assets": {
      "vrm:elmer": {
        "path": "assets/Elmer.vrm",
        "type": "vrm",
        "description": "VRM model named Elmer"
      }
    }
  }
}
```

`homunculus.service` フィールドは SDK を使ってキャラクターを生成し動作を設定する TypeScript サービスを指します。`homunculus.assets` フィールドはアセット ID `vrm:elmer` で VRM モデルを登録します。

:::note
これは説明用の例です。実際の公式 `@hmcs/elmer` MOD は独自の VRM アセットを宣言せず、`@hmcs/assets` MOD の `vrm:elmer` を利用しています。アセット ID は `type:name` 形式に従います（例：`vrm:elmer`、`vrma:idle-maid`、`se:open`）。
:::

## はじめに

- **[クイックスタート](./quick-start.md)** -- 5 分で最初の MOD をゼロから構築
- **[パッケージ設定](./project-setup/package-json.md)** -- `package.json` と `homunculus` フィールドの完全なリファレンス
- **[アセット ID](./project-setup/asset-ids.md)** -- MOD システム全体でのアセット識別子の仕組み
