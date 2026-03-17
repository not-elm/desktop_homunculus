---
title: "Type Definitions"
sidebar_position: 100
---

# Type Definitions

## `WebviewSource`

A union of three source types. Always create using the `webviewSource` helpers:

| Helper | Produces | Fields |
|--------|----------|--------|
| `webviewSource.local(id)` | `WebviewSourceLocal` | `{ type: "local", id }` |
| `webviewSource.url(url)` | `WebviewSourceUrl` | `{ type: "url", url }` |
| `webviewSource.html(content)` | `WebviewSourceHtml` | `{ type: "html", content }` |

## `WebviewSourceLocal`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `"local"` | Discriminant |
| `id` | `string` | Asset ID (e.g., `"my-mod:ui"`) |

## `WebviewSourceUrl`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `"url"` | Discriminant |
| `url` | `string` | URL string |

## `WebviewSourceHtml`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `"html"` | Discriminant |
| `content` | `string` | HTML string |

## `WebviewSourceInfo`

Webview source information returned in API responses. A union of `WebviewSourceInfoLocal`, `WebviewSourceInfoUrl`, and `WebviewSourceInfoHtml`. In list responses, HTML content is omitted.

## `WebviewSourceInfoLocal`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `"local"` | Discriminant |
| `id` | `string` | Asset ID |

## `WebviewSourceInfoUrl`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `"url"` | Discriminant |
| `url` | `string` | URL string |

## `WebviewSourceInfoHtml`

| Field | Type | Description |
|-------|------|-------------|
| `type` | `"html"` | Discriminant |
| `content` | `string \| undefined` | HTML string (omitted in list responses) |

## `WebviewInfo`

Returned by [`Webview.list()`](./list) and [`webview.info()`](./info).

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `number` | The webview's entity ID |
| `source` | `WebviewSourceInfo` | The current source |
| `size` | `Vec2` | 3D world space dimensions |
| `viewportSize` | `Vec2` | HTML pixel dimensions |
| `offset` | `Vec2` | Position offset |
| `linkedVrm` | `number \| null` | Linked VRM entity ID, or `null` |

## `WebviewPatchRequest`

Used by [`patch()`](./patch).

| Field | Type | Description |
|-------|------|-------------|
| `offset` | `Vec2` | New position offset |
| `size` | `Vec2` | New 3D dimensions |
| `viewportSize` | `Vec2` | New pixel dimensions |

## `WebviewOpenOptions`

Used by [`Webview.open()`](./open).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `WebviewSource` | -- | What to display (required) |
| `size` | `Vec2` | -- | Dimensions in 3D world space |
| `viewportSize` | `Vec2` | -- | HTML pixel dimensions |
| `offset` | `Vec2` | -- | Position relative to linked VRM or world origin |
| `linkedVrm` | `number` | -- | Entity ID of the VRM to attach to |

## `WebviewNavigateRequest`

Used internally by [`navigate()`](./navigate).

| Field | Type | Description |
|-------|------|-------------|
| `source` | `WebviewSource` | The new source to navigate to |

## `SetLinkedVrmRequest`

Used internally by [`setLinkedVrm()`](./setLinkedVrm).

| Field | Type | Description |
|-------|------|-------------|
| `vrm` | `number` | Entity ID of the VRM to link |
