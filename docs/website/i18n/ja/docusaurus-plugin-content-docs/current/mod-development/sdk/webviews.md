---
title: "WebViews"
sidebar_position: 7
---

# WebViews

3D 空間内に埋め込み HTML インターフェースを作成・制御します。WebView は自由に浮遊させることも、VRM キャラクターにリンクしてキャラクターの移動に追従させることもできます。

React UI を Vite と `@hmcs/ui` で構築する方法については、[WebView UI セットアップ](../webview-ui/setup-and-build)を参照してください。

## インポート

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";
```

## WebView ソース

すべての WebView には、表示内容を指定するソースが必要です。`webviewSource` ヘルパー関数を使用してソースを作成します。

### ローカルアセット

MOD の `package.json` のアセットで宣言された HTML ファイルを指定します：

```typescript
const source = webviewSource.local("my-mod:ui");
```

### URL

URL からコンテンツを読み込みます：

```typescript
const source = webviewSource.url("https://example.com");
```

### インライン HTML

HTML 文字列を直接レンダリングします：

```typescript
const source = webviewSource.html("<h1>Hello</h1>");
```

## WebView を開く

`Webview.open(options)` は新しい WebView を作成し、`Webview` インスタンスを返します。

```typescript
const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [0.7, 0.7],
  viewportSize: [800, 600],
  offset: [0, 0.5],
});
```

WebView を VRM キャラクターにリンクして、キャラクターの位置に追従させることができます：

```typescript
const vrm = await Vrm.findByName("MyAvatar");
const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [1, 0.9],
  viewportSize: [900, 700],
  offset: [1.1, 0],
  linkedVrm: vrm.entity,
});
```

### `WebviewOpenOptions`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `source` | `WebviewSource` | -- | 表示内容（必須） |
| `size` | `Vec2` | -- | 3D ワールド空間での寸法（例：`[0.7, 0.7]`） |
| `viewportSize` | `Vec2` | -- | HTML ピクセル寸法（例：`[800, 600]`） |
| `offset` | `Vec2` | -- | リンクされた VRM またはワールド原点からの相対位置 |
| `linkedVrm` | `number` | -- | アタッチする VRM のエンティティ ID |

## WebView の検索

### すべて一覧

```typescript
const webviews = await Webview.list();
for (const info of webviews) {
  console.log(`Entity ${info.entity}, source: ${info.source.type}`);
}
```

### 現在の WebView

WebView 内でコードが実行されている場合（例：CEF で読み込まれた React アプリ）、`Webview.current()` はその WebView のハンドルを返します。WebView コンテキスト外では `undefined` を返します。

```typescript
const webview = Webview.current();
if (webview) {
  const info = await webview.info();
  console.log("Viewport size:", info.viewportSize);
}
```

`Webview.current()` は CEF がすべての WebView コンテキストに注入する `window.WEBVIEW_ENTITY` の値を読み取ります。

## ナビゲーション

```typescript
const wv = new Webview(entity);

// 新しいソースにナビゲート
await wv.navigate(webviewSource.local("my-mod:other-page"));

// 現在のコンテンツをリロード
await wv.reload();

// 履歴ナビゲーション
await wv.navigateBack();
await wv.navigateForward();
```

## プロパティ

### 情報の取得

```typescript
const info = await webview.info();
// info.entity, info.source, info.size, info.viewportSize, info.offset, info.linkedVrm
```

### プロパティの更新

`patch()` は複数のプロパティを一度に更新します。個別のセッターも利用可能です。

```typescript
// 一括更新
await webview.patch({
  offset: [0, 1.0],
  size: [0.5, 0.5],
  viewportSize: [600, 400],
});

// 個別セッター
await webview.setOffset([0, 1.0]);
await webview.setSize([0.5, 0.5]);
await webview.setViewportSize([600, 400]);
```

## VRM リンク

WebView を VRM キャラクターにリンクしてキャラクターの位置に追従させたり、リンクを解除してフリーフローティングにしたりできます。

```typescript
import { Vrm } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// リンク
await webview.setLinkedVrm(vrm);

// リンクされた VRM を照会
const linked = await webview.linkedVrm();
// linked は Vrm インスタンス、またはリンクされていない場合は undefined

// リンク解除
await webview.unlinkVrm();
```

## ライフサイクル

```typescript
// WebView がまだ開いているか確認
const closed = await webview.isClosed();

// WebView を閉じる
await webview.close();
```

## 型定義

### `WebviewSource`

3 種類のソース型のユニオンです。`webviewSource` ヘルパーを使用して常に作成してください：

| ヘルパー | 生成される型 | フィールド |
|--------|----------|--------|
| `webviewSource.local(id)` | `WebviewSourceLocal` | `{ type: "local", id }` |
| `webviewSource.url(url)` | `WebviewSourceUrl` | `{ type: "url", url }` |
| `webviewSource.html(content)` | `WebviewSourceHtml` | `{ type: "html", content }` |

### `WebviewInfo`

`Webview.list()` と `webview.info()` から返されます。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `entity` | `number` | WebView のエンティティ ID |
| `source` | `WebviewSourceInfo` | 現在のソース |
| `size` | `Vec2` | 3D ワールド空間の寸法 |
| `viewportSize` | `Vec2` | HTML ピクセル寸法 |
| `offset` | `Vec2` | 位置オフセット |
| `linkedVrm` | `number \| null` | リンクされた VRM のエンティティ ID、または `null` |

### `WebviewPatchRequest`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `offset` | `Vec2` | 新しい位置オフセット |
| `size` | `Vec2` | 新しい 3D 寸法 |
| `viewportSize` | `Vec2` | 新しいピクセル寸法 |

:::tip[React WebView UI の構築]
React アプリを Vite と `@hmcs/ui` コンポーネントライブラリで構築するステップバイステップガイドについては、[セットアップとビルド](../webview-ui/setup-and-build)と[コンポーネントライブラリ](../webview-ui/component-library)を参照してください。
:::

## 次のステップ

- **[シグナル](./signals)** -- スクリプトと WebView 間のクロスプロセス Pub/Sub メッセージング
