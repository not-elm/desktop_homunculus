---
title: "setOffset"
sidebar_position: 9
---

# setOffset

Sets the position offset of the webview.

```typescript
async setOffset(offset: Vec2): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `offset` | `Vec2` | The new offset as `[x, y]` |

## Example

```typescript
await webview.setOffset([0, 1.0]);
```

To update multiple properties at once, use [`patch()`](./patch).
