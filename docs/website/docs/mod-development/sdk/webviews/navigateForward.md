---
title: "navigateForward"
sidebar_position: 15
---

# navigateForward

Navigates the webview forward in history.

```typescript
async navigateForward(): Promise<void>
```

## Example

```typescript
const wv = new Webview(entity);

// History navigation
await wv.navigateForward();
```
