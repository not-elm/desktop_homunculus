---
title: "setSize"
sidebar_position: 10
---

# setSize

Sets the 3D world space dimensions of the webview.

```typescript
async setSize(size: Vec2): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `size` | `Vec2` | The new size as `[width, height]` in 3D world space |

## Example

```typescript
await webview.setSize([0.5, 0.5]);
```

To update multiple properties at once, use [`patch()`](./patch).
