---
title: "patch"
sidebar_position: 8
---

# patch

Updates multiple webview properties at once. Individual setters ([`setOffset`](./setOffset), [`setSize`](./setSize), [`setViewportSize`](./setViewportSize)) are also available.

```typescript
async patch(options: WebviewPatchRequest): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options` | `WebviewPatchRequest` | The properties to update |

### `WebviewPatchRequest`

| Field | Type | Description |
|-------|------|-------------|
| `offset` | `Vec2` | New position offset |
| `size` | `Vec2` | New 3D dimensions |
| `viewportSize` | `Vec2` | New pixel dimensions |

## Example

```typescript
// Batch update
await webview.patch({
  offset: [0, 1.0],
  size: [0.5, 0.5],
  viewportSize: [600, 400],
});
```
