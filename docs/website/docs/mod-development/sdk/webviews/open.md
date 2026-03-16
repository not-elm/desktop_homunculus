---
title: "Webview.open"
sidebar_position: 2
---

# Webview.open

`Webview.open(options)` creates a new webview and returns a `Webview` instance.

```typescript
static async open(options: WebviewOpenOptions): Promise<Webview>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options` | [`WebviewOpenOptions`](./types#webviewopenoptions) | Configuration for the webview |

### `WebviewOpenOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | [`WebviewSource`](./types#webviewsource) | -- | What to display (required) |
| `size` | `Vec2` | -- | Dimensions in 3D world space (e.g., `[0.7, 0.7]`) |
| `viewportSize` | `Vec2` | -- | HTML pixel dimensions (e.g., `[800, 600]`) |
| `offset` | `Vec2` | -- | Position relative to linked VRM or world origin |
| `linkedVrm` | `number` | -- | Entity ID of the VRM to attach to |

## Returns

A `Promise` that resolves to a new `Webview` instance.

## Examples

Open a webview displaying a mod's local HTML asset:

```typescript
const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [0.7, 0.7],
  viewportSize: [800, 600],
  offset: [0, 0.5],
});
```

Link a webview to a VRM character so it follows the character's position:

```typescript
const vrm = await Vrm.findByName("MyAvatar");
const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [1, 0.9],
  viewportSize: [900, 700],
  offset: [1.1, 0],
  linkedVrm: vrm.entity,
});
```
