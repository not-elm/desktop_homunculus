---
title: "isClosed"
sidebar_position: 6
---

# isClosed

この WebView が閉じられたかどうかを確認します。

```typescript
async isClosed(): Promise<boolean>
```

## 戻り値

WebView が閉じられている場合は `true`、まだ開いている場合は `false` に解決される `Promise`。

## 例

```typescript
// WebView がまだ開いているか確認
const closed = await webview.isClosed();

// WebView を閉じる
await webview.close();
```
