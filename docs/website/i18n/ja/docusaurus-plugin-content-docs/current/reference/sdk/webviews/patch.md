---
title: "patch"
sidebar_position: 8
---

# patch

複数の WebView プロパティを一度に更新します。個別のセッター（[`setOffset`](./setOffset)、[`setSize`](./setSize)、[`setViewportSize`](./setViewportSize)）も利用可能です。

```typescript
async patch(options: WebviewPatchRequest): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `options` | [`WebviewPatchRequest`](./types#webviewpatchrequest) | 更新するプロパティ |

### `WebviewPatchRequest`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `offset` | `Vec2` | 新しい位置オフセット |
| `size` | `Vec2` | 新しい 3D 寸法 |
| `viewportSize` | `Vec2` | 新しいピクセル寸法 |

## 例

```typescript
// 一括更新
await webview.patch({
  offset: [0, 1.0],
  size: [0.5, 0.5],
  viewportSize: [600, 400],
});
```
