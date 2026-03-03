---
title: "WebView UI 概要"
sidebar_position: 1
---

# WebView UI 概要

WebView は、Desktop Homunculus ウィンドウの上にレンダリングされる Chromium ベースの UI パネルです。MOD がリッチな HTML/CSS/JavaScript インターフェース（設定パネル、コンテキストメニュー、ダッシュボードなど）を表示し、`@hmcs/sdk` を通じてエンジンと通信できるようにします。

## WebView の仕組み

Desktop Homunculus は **CEF（Chromium Embedded Framework）** を組み込み、Bevy ゲームウィンドウ内に直接 HTML コンテンツをレンダリングします。各 WebView は 3D シーンに合成されるライブブラウザサーフェスです。

主要なアーキテクチャの詳細：

- **3D 配置** -- WebView は 3D ワールド空間に自由に浮遊でき、特定の VRM キャラクターにリンクしてキャラクターの移動に追従させることもできます。
- **通信経路** -- WebView の JavaScript は `@hmcs/sdk` をインポートし、`localhost:3100` のエンジン REST API に HTTP 呼び出しを行います。API はリクエストを Bevy ECS にルーティングし、次のフレームで反映されます。
- **DevTools** -- `F1` を押すと任意の WebView の CEF DevTools パネルが開き、`F2` で閉じます。

```
┌─────────────────────────────────────────────────────┐
│  WebView (CEF)                                      │
│  JavaScript  ─── @hmcs/sdk HTTP 呼び出し ───►       │
│                                          localhost:3100
│                                          (Axum REST API)
│                                               │     │
│                                          Bevy ECS    │
└─────────────────────────────────────────────────────┘
```

## WebView のソース

すべての WebView は、何を表示するかを指定する**ソース**が必要です。3 つのソースタイプがあります：

### ローカルアセット

最も一般的なアプローチです。MOD の `package.json` でアセットとして宣言された HTML ファイルを指します：

```typescript
import { webviewSource } from "@hmcs/sdk";

const source = webviewSource.local("my-mod:ui");
```

### インライン HTML

クイックプロトタイピングやシンプルな一回限りの表示に便利です：

```typescript
const source = webviewSource.html("<h1>Hello</h1>");
```

### URL

外部 URL からコンテンツを読み込みます：

```typescript
const source = webviewSource.url("https://example.com");
```

### 完全な例

ローカルアセットソースを使った `Webview.open()` の完全な呼び出し例です：

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";

const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [0.7, 0.7],
  viewportSize: [800, 600],
  offset: [0, 0.5],
});
```

:::tip[API リファレンス]
`Webview` API の完全な詳細（メソッド、オプション、型）については、[SDK WebViews](../sdk/webviews) を参照してください。
:::

## WebView を使うべき場面

- **複雑なインタラクティブ UI** -- 設定パネル、コンテキストメニュー、ダッシュボード、HTML/CSS/JavaScript が必要なあらゆるインターフェース。
- **シンプルな一回限りの表示** -- ステータスメッセージや通知を素早く表示するだけなら、インライン HTML ソースで十分な場合があります。
- **リッチなフォームとマルチページ UI** -- MOD にフォーム、設定エディター、マルチページナビゲーションが必要な場合は、Vite と `@hmcs/ui` コンポーネントライブラリを使ったフル React アプリを構築してください。

## 次のステップ

- **[セットアップ＆ビルド](./setup-and-build)** -- React、Vite、`@hmcs/ui` で最初の WebView UI を構築する
