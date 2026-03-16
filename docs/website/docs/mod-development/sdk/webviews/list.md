---
title: "Webview.list"
sidebar_position: 3
---

# Webview.list

Gets all open webviews.

```typescript
static async list(): Promise<WebviewInfo[]>
```

## Returns

A `Promise` that resolves to an array of [`WebviewInfo`](./types) objects.

## Example

```typescript
const webviews = await Webview.list();
for (const info of webviews) {
  console.log(`Entity ${info.entity}, source: ${info.source.type}`);
}
```
