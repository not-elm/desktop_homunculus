---
title: "navigateForward"
sidebar_position: 15
---

# navigateForward

WebView の履歴を進みます。

```typescript
async navigateForward(): Promise<void>
```

## 例

```typescript
const wv = new Webview(entity);

// 履歴ナビゲーション
await wv.navigateForward();
```
