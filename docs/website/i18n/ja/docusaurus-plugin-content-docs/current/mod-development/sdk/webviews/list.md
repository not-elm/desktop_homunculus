---
title: "Webview.list"
sidebar_position: 3
---

# Webview.list

開いているすべての WebView を取得します。

```typescript
static async list(): Promise<WebviewInfo[]>
```

## 戻り値

[`WebviewInfo`](./types) オブジェクトの配列に解決される `Promise`。

## 例

```typescript
const webviews = await Webview.list();
for (const info of webviews) {
  console.log(`Entity ${info.entity}, source: ${info.source.type}`);
}
```
