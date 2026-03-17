---
title: "webviewSource.html"
sidebar_position: 21
---

# webviewSource.html

Creates an inline HTML source that renders an HTML string directly.

```typescript
function html(content: string): WebviewSourceHtml
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `content` | `string` | HTML string |

## Returns

A [`WebviewSourceHtml`](./types#webviewsourcehtml) object: `{ type: "html", content }`.

## Example

```typescript
const source = webviewSource.html("<h1>Hello</h1>");
// { type: "html", content: "<h1>Hello</h1>" }
```
