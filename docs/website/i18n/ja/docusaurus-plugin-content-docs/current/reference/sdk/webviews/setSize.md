---
title: "setSize"
sidebar_position: 10
---

# setSize

WebView の 3D ワールド空間での寸法を設定します。

```typescript
async setSize(size: Vec2): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `size` | `Vec2` | 新しいサイズ（3D ワールド空間での `[幅, 高さ]`） |

## 例

```typescript
await webview.setSize([0.5, 0.5]);
```

複数のプロパティを一度に更新するには [`patch()`](./patch) を使用してください。
