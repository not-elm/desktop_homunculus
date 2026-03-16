---
title: "webviewSource.local"
sidebar_position: 19
---

# webviewSource.local

Creates a local asset source pointing to an HTML file declared in your MOD's `package.json` assets.

```typescript
function local(id: string): WebviewSourceLocal
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | `string` | Asset ID (e.g., `"menu:ui"`, `"settings:ui"`) |

## Returns

A [`WebviewSourceLocal`](./types#webviewsourcelocal) object: `{ type: "local", id }`.

## Example

```typescript
const source = webviewSource.local("my-mod:ui");
// { type: "local", id: "my-mod:ui" }
```
