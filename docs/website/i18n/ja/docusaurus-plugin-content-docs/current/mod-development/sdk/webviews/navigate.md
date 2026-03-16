---
title: "navigate"
sidebar_position: 12
---

# navigate

WebView を新しいソースにナビゲートします。

```typescript
async navigate(source: WebviewSource): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | [`WebviewSource`](./types#webviewsource) | 新しいソース（URL、インライン HTML、またはローカルアセット ID） |

## 例

```typescript
const wv = new Webview(entity);

// 新しいソースにナビゲート
await wv.navigate(webviewSource.local("my-mod:other-page"));
```
