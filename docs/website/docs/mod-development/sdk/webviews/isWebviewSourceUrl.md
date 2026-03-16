---
title: "isWebviewSourceUrl"
sidebar_position: 23
---

# isWebviewSourceUrl

Type guard that checks whether a `WebviewSource` is a URL source.

```typescript
function isWebviewSourceUrl(source: WebviewSource): source is WebviewSourceUrl
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSource` | The source to check |

## Returns

`true` if `source.type === "url"`, narrowing the type to [`WebviewSourceUrl`](./types#webviewsourceurl).

## Example

```typescript
const source = webviewSource.url("https://example.com");

if (isWebviewSourceUrl(source)) {
  console.log(source.url); // "https://example.com"
}
```
