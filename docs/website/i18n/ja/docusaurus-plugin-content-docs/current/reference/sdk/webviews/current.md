---
title: "Webview.current"
sidebar_position: 4
---

# Webview.current

WebView 内でコードが実行されている場合（例：CEF で読み込まれた React アプリ）、`Webview.current()` はその WebView のハンドルを返します。WebView コンテキスト外では `undefined` を返します。

```typescript
static current(): Webview | undefined
```

## 戻り値

現在の `Webview` インスタンス、または WebView コンテキスト外の場合は `undefined`。

## 例

```typescript
const webview = Webview.current();
if (webview) {
  const info = await webview.info();
  console.log("Viewport size:", info.viewportSize);
}
```

`Webview.current()` は CEF がすべての WebView コンテキストに注入する `window.WEBVIEW_ENTITY` の値を読み取ります。
