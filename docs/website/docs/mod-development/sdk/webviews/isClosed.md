---
title: "isClosed"
sidebar_position: 6
---

# isClosed

Checks whether this webview has been closed.

```typescript
async isClosed(): Promise<boolean>
```

## Returns

A `Promise` that resolves to `true` if the webview is closed, `false` if it is still open.

## Example

```typescript
// Check if the webview is still open
const closed = await webview.isClosed();

// Close the webview
await webview.close();
```
