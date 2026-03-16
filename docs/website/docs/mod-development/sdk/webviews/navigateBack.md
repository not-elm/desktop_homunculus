---
title: "navigateBack"
sidebar_position: 14
---

# navigateBack

Navigates the webview back in history.

```typescript
async navigateBack(): Promise<void>
```

## Example

```typescript
const wv = new Webview(entity);

// History navigation
await wv.navigateBack();
```
