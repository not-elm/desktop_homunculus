---
title: "info"
sidebar_position: 7
---

# info

この WebView の情報を取得します。

```typescript
async info(): Promise<WebviewInfo>
```

## 戻り値

[`WebviewInfo`](./types#webviewinfo) オブジェクトに解決される `Promise`。

## 例

```typescript
const info = await webview.info();
// info.entity, info.source, info.size, info.viewportSize, info.offset, info.linkedPersona
```
