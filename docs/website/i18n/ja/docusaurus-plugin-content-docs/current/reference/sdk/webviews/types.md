---
title: "型定義"
sidebar_position: 100
---

# 型定義

## `WebviewSource`

3 種類のソース型のユニオンです。`webviewSource` ヘルパーを使用して常に作成してください：

| ヘルパー | 生成される型 | フィールド |
|--------|----------|--------|
| `webviewSource.local(id)` | `WebviewSourceLocal` | `{ type: "local", id }` |
| `webviewSource.url(url)` | `WebviewSourceUrl` | `{ type: "url", url }` |
| `webviewSource.html(content)` | `WebviewSourceHtml` | `{ type: "html", content }` |

## `WebviewSourceLocal`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `type` | `"local"` | 判別子 |
| `id` | `string` | アセット ID（例：`"my-mod:ui"`） |

## `WebviewSourceUrl`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `type` | `"url"` | 判別子 |
| `url` | `string` | URL 文字列 |

## `WebviewSourceHtml`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `type` | `"html"` | 判別子 |
| `content` | `string` | HTML 文字列 |

## `WebviewSourceInfo`

API レスポンスで返される WebView ソース情報。`WebviewSourceInfoLocal`、`WebviewSourceInfoUrl`、`WebviewSourceInfoHtml` のユニオンです。リストレスポンスでは HTML コンテンツは省略されます。

## `WebviewSourceInfoLocal`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `type` | `"local"` | 判別子 |
| `id` | `string` | アセット ID |

## `WebviewSourceInfoUrl`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `type` | `"url"` | 判別子 |
| `url` | `string` | URL 文字列 |

## `WebviewSourceInfoHtml`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `type` | `"html"` | 判別子 |
| `content` | `string \| undefined` | HTML 文字列（リストレスポンスでは省略） |

## `WebviewInfo`

[`Webview.list()`](./list) と [`webview.info()`](./info) から返されます。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `entity` | `number` | WebView のエンティティ ID |
| `source` | `WebviewSourceInfo` | 現在のソース |
| `size` | `Vec2` | 3D ワールド空間の寸法 |
| `viewportSize` | `Vec2` | HTML ピクセル寸法 |
| `offset` | `Vec2` | 位置オフセット |
| `linkedPersona` | `string \| null` | リンクされたペルソナ ID、または `null` |

## `WebviewPatchRequest`

[`patch()`](./patch) で使用します。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `offset` | `Vec2` | 新しい位置オフセット |
| `size` | `Vec2` | 新しい 3D 寸法 |
| `viewportSize` | `Vec2` | 新しいピクセル寸法 |

## `WebviewOpenOptions`

[`Webview.open()`](./open) で使用します。

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `source` | `WebviewSource` | -- | 表示内容（必須） |
| `size` | `Vec2` | -- | 3D ワールド空間での寸法 |
| `viewportSize` | `Vec2` | -- | HTML ピクセル寸法 |
| `offset` | `Vec2` | -- | リンクされたペルソナまたはワールド原点からの相対位置 |
| `linkedPersona` | `string` | -- | アタッチするペルソナ ID |

## `WebviewNavigateRequest`

[`navigate()`](./navigate) が内部で使用します。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `source` | `WebviewSource` | ナビゲート先の新しいソース |

## `SetLinkedPersonaRequest`

[`setLinkedPersona()`](./setLinkedVrm) が内部で使用します。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `personaId` | `string` | リンクするペルソナ ID |
