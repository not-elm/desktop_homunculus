---
title: "navigateBack"
sidebar_position: 14
---

# navigateBack

WebView の履歴を戻ります。

```typescript
async navigateBack(): Promise<void>
```

## 例

```typescript
const wv = new Webview(entity);

// 履歴ナビゲーション
await wv.navigateBack();
```
