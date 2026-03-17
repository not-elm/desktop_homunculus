---
title: "setOffset"
sidebar_position: 9
---

# setOffset

WebView の位置オフセットを設定します。

```typescript
async setOffset(offset: Vec2): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `offset` | `Vec2` | 新しいオフセット（`[x, y]`） |

## 例

```typescript
await webview.setOffset([0, 1.0]);
```

複数のプロパティを一度に更新するには [`patch()`](./patch) を使用してください。
