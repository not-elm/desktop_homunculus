---
title: "ディレクトリ構成"
sidebar_position: 1
---

# ディレクトリ構成

このページでは、Desktop Homunculus MOD のファイル構成について説明します。MOD はわずか 3 つのファイルで構成できるほどシンプルです。唯一の必須要件は `homunculus` フィールドを持つ `package.json` です。それ以外は MOD の用途に依存します。

## 最小構成の MOD

VRM モデルを生成するキャラクター MOD は、パッケージ設定、サービススクリプト、モデルファイルの 3 つだけで済みます：

```text
my-character/
├── package.json       # homunculus フィールドを持つパッケージ設定
├── index.ts           # サービス（アプリ起動時に実行）
└── assets/
    └── MyModel.vrm    # VRM キャラクターモデル
```

**`package.json`** -- `"type": "module"` とサービススクリプトおよびアセットを宣言する `"homunculus"` フィールドを含む必要があります。各フィールドの詳細は[パッケージ設定](./package-json.md)を参照してください。

**`index.ts`** -- `"homunculus.service"` で指定されるサービススクリプト。エンジンは `tsx` 経由で自動実行するため、ビルドステップなしで TypeScript を直接記述できます。このスクリプトは通常、キャラクターの生成と動作の設定を行います。

**`assets/`** -- バイナリアセット（VRM モデル、アニメーション、サウンド、画像）を格納する規約です。ディレクトリ名は柔軟に変更可能です。実際のファイルパスは `package.json` で宣言しますが、`assets/` が公式 MOD 全体での標準です。

:::tip
動作する MOD にはこれだけで十分です。`hmcs mod install <path-to-mod>` でインストールすれば、キャラクターがデスクトップに表示されます。
:::

## UI を持つ MOD

設定パネルやその他の UI を提供する MOD は、React アプリとオンデマンド bin コマンドを追加します：

```text
my-settings-mod/
├── package.json           # アセット、メニュー、bin コマンドを宣言
├── commands/
│   └── open-ui.ts         # bin コマンドスクリプト（HTTP API 経由で呼び出し）
└── ui/
    ├── index.html         # Vite エントリポイント
    ├── vite.config.ts     # Vite ビルド設定
    ├── tsconfig.json      # UI アプリ用 TypeScript 設定
    └── src/
        ├── main.tsx       # React エントリポイント
        ├── App.tsx         # ルートコンポーネント
        ├── index.css      # スタイル（Tailwind）
        └── components/    # UI コンポーネント
```

### `commands/`

`package.json` の `"bin"` フィールドで公開されるオンデマンドスクリプトです。HTTP API（`POST /commands/execute`）を通じて明示的に呼び出された場合にのみ実行されます。慣例的なディレクトリ名は `commands/` または `bin/` です。

### `ui/`

WebView 内でレンダリングされる React + Vite アプリです。ソースは `ui/src/` に置き、`ui/dist/` にビルドされます。ビルドされた `ui/dist/index.html` は `package.json` で `html` アセットとして宣言され、エンジンは WebView を開く際にこれを読み込みます。

UI アプリは通常、共有コンポーネント用に `@hmcs/ui` をインポートし、スタイリングに Tailwind を使用します。これは MOD の中でビルドステップが必要な唯一の部分です（`ui/` ディレクトリで `pnpm build` を実行）。

`ui/dist/` はビルド成果物であるため、上のツリーには表示されていません。生成するには `ui/` ディレクトリで `pnpm build` を実行してください。

:::info
この例にはルートに `index.ts` がありません。オンデマンドコマンドと UI パネルのみを使用しています。MOD はサービス、UI アプリ、bin コマンドの両方のパターンを同一パッケージ内で組み合わせることもできます。
:::

## ファイルリファレンス

| ファイル / ディレクトリ   | 用途                                            | 必須                 |
| ------------------------- | ----------------------------------------------- | -------------------- |
| `package.json`            | `homunculus` フィールドを持つパッケージ設定     | はい                 |
| `index.ts`                | サービス（アプリ起動時に実行）                  | いいえ               |
| `assets/`                 | バイナリアセット（VRM、VRMA、サウンド、画像）   | いいえ               |
| `commands/` または `bin/` | MODコマンドスクリプト                           | いいえ               |
| `ui/`                     | WebView UI アプリ（React + Vite）               | いいえ               |
| `ui/dist/`                | ビルド済み UI 出力（`html` アセットとして宣言） | `ui/` がある場合のみ |
| `lib/`                    | スクリプトで使用する共有ユーティリティコード    | いいえ               |

## 規約

- **バイナリアセットは `assets/` に格納します。** ディレクトリ名は規約であり、必須ではありません。実際のファイルパスは `package.json` で宣言します。
- **bin コマンドスクリプトは `commands/` または `bin/` に配置します。** どちらも公式 MOD で一般的です。
- **UI アプリは `ui/` に配置し、ビルド出力は `ui/dist/` に出力します。**
- **共有ヘルパーコードは `lib/` に配置します。**
- **必須ファイルは `package.json` のみです。** その他は MOD の用途に依存します。

## 次のステップ

- [パッケージ設定](./package-json.md) -- すべての `package.json` フィールドの詳細
- [アセット ID](./asset-ids.md) -- アセット識別子の仕組み
