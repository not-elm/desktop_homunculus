---
title: "isWebviewSourceHtml"
sidebar_position: 24
---

# isWebviewSourceHtml

Type guard that checks whether a `WebviewSource` is an inline HTML source.

```typescript
function isWebviewSourceHtml(source: WebviewSource): source is WebviewSourceHtml
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSource` | The source to check |

## Returns

`true` if `source.type === "html"`, narrowing the type to `WebviewSourceHtml`.

## Example

```typescript
const source = webviewSource.html("<h1>Hello</h1>");

if (isWebviewSourceHtml(source)) {
  console.log(source.content); // "<h1>Hello</h1>"
}
```
