---
title: "setViewportSize"
sidebar_position: 11
---

# setViewportSize

WebView ビューポートの HTML ピクセル寸法を設定します。

```typescript
async setViewportSize(size: Vec2): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `size` | `Vec2` | 新しいビューポートサイズ（ピクセル単位の `[幅, 高さ]`） |

## 例

```typescript
await webview.setViewportSize([600, 400]);
```

複数のプロパティを一度に更新するには [`patch()`](./patch) を使用してください。
