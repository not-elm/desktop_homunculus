---
title: "isWebviewSourceInfoHtml"
sidebar_position: 27
---

# isWebviewSourceInfoHtml

Type guard that checks whether a `WebviewSourceInfo` is an inline HTML source.

```typescript
function isWebviewSourceInfoHtml(source: WebviewSourceInfoHtml): source is WebviewSourceInfoHtml
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSourceInfoHtml` | The source info to check |

## Returns

`true` if `source.type === "html"`, narrowing the type to [`WebviewSourceInfoHtml`](./types#webviewsourceinfohtml).

## Example

```typescript
const info = await webview.info();

if (isWebviewSourceInfoHtml(info.source as WebviewSourceInfoHtml)) {
  console.log(info.source.content); // may be undefined in list responses
}
```
