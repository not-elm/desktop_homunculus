---
title: "Webview.open"
sidebar_position: 2
---

# Webview.open

`Webview.open(options)` は新しい WebView を作成し、`Webview` インスタンスを返します。

```typescript
static async open(options: WebviewOpenOptions): Promise<Webview>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `options` | [`WebviewOpenOptions`](./types#webviewopenoptions) | WebView の設定 |

### `WebviewOpenOptions`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `source` | [`WebviewSource`](./types#webviewsource) | -- | 表示内容（必須） |
| `size` | `Vec2` | -- | 3D ワールド空間での寸法（例：`[0.7, 0.7]`） |
| `viewportSize` | `Vec2` | -- | HTML ピクセル寸法（例：`[800, 600]`） |
| `offset` | `Vec2` | -- | リンクされたペルソナまたはワールド原点からの相対位置 |
| `linkedPersona` | `string` | -- | アタッチするペルソナ ID |

## 戻り値

新しい `Webview` インスタンスに解決される `Promise`。

## 例

MOD のローカル HTML アセットを表示する WebView を開く：

```typescript
const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [0.7, 0.7],
  viewportSize: [800, 600],
  offset: [0, 0.5],
});
```

WebView をペルソナにリンクして、キャラクターの位置に追従させる：

```typescript
import { persona } from "@hmcs/sdk";

const p = await persona.load("alice");
const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [1, 0.9],
  viewportSize: [900, 700],
  offset: [1.1, 0],
  linkedPersona: p.id,
});
```
