---
title: "navigate"
sidebar_position: 12
---

# navigate

Navigates the webview to a new source.

```typescript
async navigate(source: WebviewSource): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSource` | The new source (URL, inline HTML, or local asset ID) |

## Example

```typescript
const wv = new Webview(entity);

// Navigate to a new source
await wv.navigate(webviewSource.local("my-mod:other-page"));
```
