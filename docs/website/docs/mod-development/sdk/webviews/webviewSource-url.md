---
title: "webviewSource.url"
sidebar_position: 20
---

# webviewSource.url

Creates a URL source that loads content from a remote URL.

```typescript
function url(url: string): WebviewSourceUrl
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `string` | URL string |

## Returns

A `WebviewSourceUrl` object: `{ type: "url", url }`.

## Example

```typescript
const source = webviewSource.url("https://example.com");
// { type: "url", url: "https://example.com" }
```
