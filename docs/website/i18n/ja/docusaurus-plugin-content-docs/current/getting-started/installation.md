---
title: "インストール"
sidebar_position: 2
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# インストール

Desktop Homunculus をダウンロードしてインストールします。インストーラーには必要なものがすべて含まれています — エンジン、Node.js ランタイム、`hmcs` CLI、すべての公式 MOD が自動的にセットアップされます。

## システム要件

|                | macOS                        | Windows             |
| -------------- | ---------------------------- | ------------------- |
| **OS**         | macOS 11（Big Sur）以降       | Windows 10 以降      |
| **CPU**        | Apple Silicon または Intel    | x86_64              |
| **ディスク容量** | 600 MB 以上                  | 600 MB 以上          |
| **ネットワーク** | 初回セットアップ時に必要       | 初回セットアップ時に必要 |

## Desktop Homunculus のインストール

[GitHub Releases ページ](https://github.com/not-elm/desktop-homunculus/releases)から最新版をダウンロードしてください。

<Tabs>
<TabItem value="macos" label="macOS" default>

1. `.pkg` ファイルをダウンロード
2. インストーラーを開いて指示に従う
3. アプリケーションからアプリを起動

インストーラーは以下を行います：
- Desktop Homunculus アプリを `/Applications` にインストール
- `hmcs` CLI を PATH に追加（`/usr/local/bin/hmcs`）
- すべての公式 MOD を自動検出してインストール

:::warning[Gatekeeper の警告]
macOS で「Desktop Homunculus は開発元が未確認のため開けません」と表示された場合：

1. **システム設定** > **プライバシーとセキュリティ** を開く
2. 下にスクロールして **このまま開く** をクリック
3. 確認ダイアログで **開く** をクリック
:::

</TabItem>
<TabItem value="windows" label="Windows">

:::caution[NVIDIA GPU ユーザー — 初回起動前に必須]
NVIDIA GPU を搭載している場合、Desktop Homunculus を起動する前に以下の設定を行う**必要があります**。設定しないとウィンドウの背景が透明ではなく黒になります：

1. **NVIDIA コントロールパネル** を開く
2. **3D 設定の管理** に移動
3. **「Vulkan/OpenGL present method」** を見つける
4. **「Prefer native」** に設定
5. **適用** をクリック

この設定は**初回起動前**に行う必要があります。
:::

1. `.msi` ファイルをダウンロード
2. インストーラーを実行して指示に従う
3. スタートメニューから **Desktop Homunculus** を起動

インストーラーは以下を行います：
- アプリとバンドルされた Node.js ランタイムをインストール
- `hmcs` CLI をシステム PATH に追加
- すべての公式 MOD を自動検出してインストール

</TabItem>
</Tabs>

## インストールの確認

1. **Desktop Homunculus** を起動
2. **システムトレイ**（通知領域）にアプリアイコンが表示されるはずです

CLI が利用可能か確認することもできます：

```shell
hmcs --version
```

すべて正常であれば、セットアップ完了です！

:::tip[次のステップ]
[基本コンセプト](./concepts)で Persona や MOD について学んでから、[クイックスタート](./quick-start)ガイドに進みましょう。
:::

## 開発者向け

Desktop Homunculus の開発や MOD のソースビルドを行う場合は、Cargo 経由で CLI をインストールしてください：

```shell
# engine/ ディレクトリから実行
make install-cli
```

これにより `hmcs` が `~/.cargo/bin/hmcs` にインストールされます。

## トラブルシューティング

### `hmcs: command not found`

- **macOS**: ターミナルを再起動してください。インストーラーは `/usr/local/bin/hmcs` にシンボリックリンクを作成します。
- **Windows**: ターミナルを再起動するか、サインアウトして再度サインインしてください。インストーラーは bin ディレクトリをシステム PATH に追加します。

### Gatekeeper にブロックされる（macOS）

上記のインストール手順を参照して、Gatekeeper を通してアプリを許可してください。

### Windows でウィンドウの背景が黒い/不透明

Windows でウィンドウの背景が透明ではなく黒く表示される場合、設定変更が必要な NVIDIA GPU を使用している可能性があります。上記の NVIDIA GPU セットアップ手順を参照してください。

### 起動してもキャラクターが表示されない

セットアップ中に公式 MOD のインストールに失敗した場合（ネットワーク問題など）、手動でインストールしてください：

```shell
hmcs mod install @hmcs/assets @hmcs/persona @hmcs/menu @hmcs/settings @hmcs/app-exit
```

その後、Desktop Homunculus を再起動してください。

### MOD のインストールに失敗する

ネットワーク接続を確認して、もう一度お試しください。MOD のインストールには npm レジストリへのアクセスが必要です。
