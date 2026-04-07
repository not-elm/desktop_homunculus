---
title: "Webview"
sidebar_position: 1
---

# Webview

3D 空間内に埋め込み HTML インターフェースを作成・制御します。WebView は自由に浮遊させることも、ペルソナにリンクしてキャラクターの移動に追従させることもできます。

React UI を Vite と `@hmcs/ui` で構築する方法については、[WebView UI セットアップ](/mod-development/webview-ui/setup-and-build)を参照してください。

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";
```

## スタティックメソッド

| メソッド | 説明 |
|--------|-------------|
| [`Webview.open(options)`](./open) | 新しい WebView を作成し、`Webview` インスタンスを返す |
| [`Webview.list()`](./list) | 開いているすべての WebView を取得する |
| [`Webview.current()`](./current) | WebView コンテキスト内で実行中の現在の WebView を取得する |

## インスタンスメソッド

| メソッド | 説明 |
|--------|-------------|
| [`close()`](./close) | WebView を閉じる |
| [`isClosed()`](./isClosed) | WebView が閉じられたかどうかを確認する |
| [`info()`](./info) | WebView の情報を取得する |
| [`patch(options)`](./patch) | 複数のプロパティを一度に更新する |
| [`setOffset(offset)`](./setOffset) | 位置オフセットを設定する |
| [`setSize(size)`](./setSize) | 3D ワールド空間の寸法を設定する |
| [`setViewportSize(size)`](./setViewportSize) | HTML ピクセル寸法を設定する |
| [`navigate(source)`](./navigate) | 新しいソースにナビゲートする |
| [`reload()`](./reload) | 現在のコンテンツをリロードする |
| [`navigateBack()`](./navigateBack) | 履歴を戻る |
| [`navigateForward()`](./navigateForward) | 履歴を進む |
| [`linkedPersona()`](./linkedVrm) | この WebView にリンクされたペルソナを取得する |
| [`setLinkedPersona(personaId)`](./setLinkedVrm) | WebView をペルソナにリンクする |
| [`unlinkPersona()`](./unlinkVrm) | ペルソナリンクを解除する |

## ヘルパー

| 関数 | 説明 |
|----------|-------------|
| [`webviewSource.local(id)`](./webviewSource-local) | ローカルアセットソースを作成する |
| [`webviewSource.url(url)`](./webviewSource-url) | URL ソースを作成する |
| [`webviewSource.html(content)`](./webviewSource-html) | インライン HTML ソースを作成する |

## 型ガード

| 関数 | 説明 |
|----------|-------------|
| [`isWebviewSourceLocal(source)`](./isWebviewSourceLocal) | `WebviewSource` がローカルソースかどうかを確認する |
| [`isWebviewSourceUrl(source)`](./isWebviewSourceUrl) | `WebviewSource` が URL ソースかどうかを確認する |
| [`isWebviewSourceHtml(source)`](./isWebviewSourceHtml) | `WebviewSource` がインライン HTML ソースかどうかを確認する |
| [`isWebviewSourceInfoLocal(source)`](./isWebviewSourceInfoLocal) | `WebviewSourceInfo` がローカルソースかどうかを確認する |
| [`isWebviewSourceInfoUrl(source)`](./isWebviewSourceInfoUrl) | `WebviewSourceInfo` が URL ソースかどうかを確認する |
| [`isWebviewSourceInfoHtml(source)`](./isWebviewSourceInfoHtml) | `WebviewSourceInfo` がインライン HTML ソースかどうかを確認する |

## 次のステップ

- **[シグナル](../signals/)** -- スクリプトと WebView 間のクロスプロセス Pub/Sub メッセージング
