---
title: "close"
sidebar_position: 5
---

# close

Closes the webview.

```typescript
async close(): Promise<void>
```

## Example

```typescript
// Check if the webview is still open
const closed = await webview.isClosed();

// Close the webview
await webview.close();
```
