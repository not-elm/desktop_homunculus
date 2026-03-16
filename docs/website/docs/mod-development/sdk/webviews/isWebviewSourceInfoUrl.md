---
title: "isWebviewSourceInfoUrl"
sidebar_position: 26
---

# isWebviewSourceInfoUrl

Type guard that checks whether a `WebviewSourceInfo` is a URL source.

```typescript
function isWebviewSourceInfoUrl(source: WebviewSourceInfo): source is WebviewSourceInfoUrl
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSourceInfo` | The source info to check |

## Returns

`true` if `source.type === "url"`, narrowing the type to [`WebviewSourceInfoUrl`](./types#webviewsourceinfourl).

## Example

```typescript
const info = await webview.info();

if (isWebviewSourceInfoUrl(info.source)) {
  console.log(info.source.url);
}
```
