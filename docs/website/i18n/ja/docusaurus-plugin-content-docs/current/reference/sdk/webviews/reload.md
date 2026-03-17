---
title: "reload"
sidebar_position: 13
---

# reload

現在の WebView コンテンツをリロードします。

```typescript
async reload(): Promise<void>
```

## 例

```typescript
const wv = new Webview(entity);

// 現在のコンテンツをリロード
await wv.reload();
```
