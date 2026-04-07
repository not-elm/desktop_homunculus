---
title: "info"
sidebar_position: 7
---

# info

Gets information about this webview.

```typescript
async info(): Promise<WebviewInfo>
```

## Returns

A `Promise` that resolves to a [`WebviewInfo`](./types#webviewinfo) object.

## Example

```typescript
const info = await webview.info();
// info.entity, info.source, info.size, info.viewportSize, info.offset, info.linkedPersona
```
