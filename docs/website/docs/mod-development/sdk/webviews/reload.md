---
title: "reload"
sidebar_position: 13
---

# reload

Reloads the current webview content.

```typescript
async reload(): Promise<void>
```

## Example

```typescript
const wv = new Webview(entity);

// Reload the current content
await wv.reload();
```
