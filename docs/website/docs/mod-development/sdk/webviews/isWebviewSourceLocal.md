---
title: "isWebviewSourceLocal"
sidebar_position: 22
---

# isWebviewSourceLocal

Type guard that checks whether a `WebviewSource` is a local asset source.

```typescript
function isWebviewSourceLocal(source: WebviewSource): source is WebviewSourceLocal
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSource` | The source to check |

## Returns

`true` if `source.type === "local"`, narrowing the type to `WebviewSourceLocal`.

## Example

```typescript
const source = webviewSource.local("my-mod:ui");

if (isWebviewSourceLocal(source)) {
  console.log(source.id); // "my-mod:ui"
}
```
