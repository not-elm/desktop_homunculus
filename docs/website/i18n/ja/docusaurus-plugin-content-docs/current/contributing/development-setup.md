---
title: 開発環境のセットアップ
sidebar_position: 2
---

# 開発環境のセットアップ

このガイドでは、Desktop Homunculus にコントリビューションする前にインストールが必要なツールについて説明します。

## 前提条件

### 全コントリビュータ向け

| ツール | バージョン | リンク |
|--------|-----------|--------|
| Git | 最新 | [git-scm.com](https://git-scm.com/) |
| Node.js | 22+ | [nodejs.org](https://nodejs.org/)（npm はセットアップスクリプトで使用されるためバンドル済み） |
| pnpm | 10.x | [pnpm.io](https://pnpm.io/) |

### Engine (Rust) コントリビュータ向け

上記すべてに加えて：

| ツール | バージョン | リンク |
|--------|-----------|--------|
| Rust | 最新 stable | [rustup.rs](https://rustup.rs/) |
| Python | 3.x | [python.org](https://www.python.org/)（セットアップスクリプトで必要） |
| Make | 最新 | Xcode Command Line Tools（macOS）またはビルドツール（Windows/Linux）に付属 |

### プラットフォーム固有の注意事項

- **macOS**: Xcode Command Line Tools をインストール — `xcode-select --install`
- **Windows**: [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) を **C++ ワークロード**付きでインストール

## Clone & セットアップ

```shell
git clone https://github.com/not-elm/desktop-homunculus.git
cd desktop-homunculus

# すべての依存関係をインストール（Node パッケージ、Rust ツール、グローバル npm ツール、CEF フレームワーク）
make setup

# デバッグモードでアプリを起動（ホットリロードと DevTools 付き）
make debug
```

`make setup` は Node の依存関係、Rust ツール、グローバル npm ツール（例：`@redocly/cli`）のインストールと CEF フレームワークのダウンロードを行います。利用可能なすべてのコマンドについては、ルートの `Makefile` をご覧ください。

## 次のステップ

コントリビューションの方法、PR ガイドライン、求められるコントリビューション分野については[コントリビューティング](/contributing)をご覧ください。
