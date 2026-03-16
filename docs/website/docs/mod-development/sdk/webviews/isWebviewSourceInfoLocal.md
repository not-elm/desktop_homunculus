---
title: "isWebviewSourceInfoLocal"
sidebar_position: 25
---

# isWebviewSourceInfoLocal

Type guard that checks whether a `WebviewSourceInfo` is a local asset source.

```typescript
function isWebviewSourceInfoLocal(source: WebviewSourceInfo): source is WebviewSourceInfoLocal
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `WebviewSourceInfo` | The source info to check |

## Returns

`true` if `source.type === "local"`, narrowing the type to `WebviewSourceInfoLocal`.

## Example

```typescript
const info = await webview.info();

if (isWebviewSourceInfoLocal(info.source)) {
  console.log(info.source.id);
}
```
