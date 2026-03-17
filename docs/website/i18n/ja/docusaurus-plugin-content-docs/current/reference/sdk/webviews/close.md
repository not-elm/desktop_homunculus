---
title: "close"
sidebar_position: 5
---

# close

WebView を閉じます。

```typescript
async close(): Promise<void>
```

## 例

```typescript
// WebView がまだ開いているか確認
const closed = await webview.isClosed();

// WebView を閉じる
await webview.close();
```
