---
title: "setViewportSize"
sidebar_position: 11
---

# setViewportSize

Sets the HTML pixel dimensions of the webview viewport.

```typescript
async setViewportSize(size: Vec2): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `size` | `Vec2` | The new viewport size as `[width, height]` in pixels |

## Example

```typescript
await webview.setViewportSize([600, 400]);
```

To update multiple properties at once, use [`patch()`](./patch).
