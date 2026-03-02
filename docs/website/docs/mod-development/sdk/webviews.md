---
title: "WebViews"
sidebar_position: 7
---

# WebViews

Create and control embedded HTML interfaces in 3D space. WebViews can float freely or be linked to a VRM character so they follow as the character moves.

For building a React UI with Vite and `@hmcs/ui`, see [WebView UI Setup](../webview-ui/setup-and-build).

## Import

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";
```

## WebView Sources

Every webview needs a source specifying what to display. Use the `webviewSource` helper functions to create sources.

### Local Asset

Points to an HTML file declared in your MOD's `package.json` assets:

```typescript
const source = webviewSource.local("my-mod:ui");
```

### URL

Loads content from a URL:

```typescript
const source = webviewSource.url("https://example.com");
```

### Inline HTML

Renders an HTML string directly:

```typescript
const source = webviewSource.html("<h1>Hello</h1>");
```

## Opening a WebView

`Webview.open(options)` creates a new webview and returns a `Webview` instance.

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

### `WebviewOpenOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `source` | `WebviewSource` | -- | What to display (required) |
| `size` | `Vec2` | -- | Dimensions in 3D world space (e.g., `[0.7, 0.7]`) |
| `viewportSize` | `Vec2` | -- | HTML pixel dimensions (e.g., `[800, 600]`) |
| `offset` | `Vec2` | -- | Position relative to linked VRM or world origin |
| `linkedVrm` | `number` | -- | Entity ID of the VRM to attach to |

## Finding WebViews

### List All

```typescript
const webviews = await Webview.list();
for (const info of webviews) {
  console.log(`Entity ${info.entity}, source: ${info.source.type}`);
}
```

### Current WebView

When code runs inside a WebView (e.g., in a React app loaded by CEF), `Webview.current()` returns a handle to that webview. Returns `undefined` outside a WebView context.

```typescript
const webview = Webview.current();
if (webview) {
  const info = await webview.info();
  console.log("Viewport size:", info.viewportSize);
}
```

`Webview.current()` reads the `window.WEBVIEW_ENTITY` value that CEF injects into every webview context.

## Navigation

```typescript
const wv = new Webview(entity);

// Navigate to a new source
await wv.navigate(webviewSource.local("my-mod:other-page"));

// Reload the current content
await wv.reload();

// History navigation
await wv.navigateBack();
await wv.navigateForward();
```

## Properties

### Get Info

```typescript
const info = await webview.info();
// info.entity, info.source, info.size, info.viewportSize, info.offset, info.linkedVrm
```

### Update Properties

`patch()` updates multiple properties at once. Individual setters are also available.

```typescript
// Batch update
await webview.patch({
  offset: [0, 1.0],
  size: [0.5, 0.5],
  viewportSize: [600, 400],
});

// Individual setters
await webview.setOffset([0, 1.0]);
await webview.setSize([0.5, 0.5]);
await webview.setViewportSize([600, 400]);
```

## VRM Linking

Link a webview to a VRM character so it follows the character's position, or unlink to make it free-floating.

```typescript
import { Vrm } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// Link
await webview.setLinkedVrm(vrm);

// Query the linked VRM
const linked = await webview.linkedVrm();
// linked is a Vrm instance, or undefined if not linked

// Unlink
await webview.unlinkVrm();
```

## Lifecycle

```typescript
// Check if the webview is still open
const closed = await webview.isClosed();

// Close the webview
await webview.close();
```

## Types

### `WebviewSource`

A union of three source types. Always create using the `webviewSource` helpers:

| Helper | Produces | Fields |
|--------|----------|--------|
| `webviewSource.local(id)` | `WebviewSourceLocal` | `{ type: "local", id }` |
| `webviewSource.url(url)` | `WebviewSourceUrl` | `{ type: "url", url }` |
| `webviewSource.html(content)` | `WebviewSourceHtml` | `{ type: "html", content }` |

### `WebviewInfo`

Returned by `Webview.list()` and `webview.info()`.

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `number` | The webview's entity ID |
| `source` | `WebviewSourceInfo` | The current source |
| `size` | `Vec2` | 3D world space dimensions |
| `viewportSize` | `Vec2` | HTML pixel dimensions |
| `offset` | `Vec2` | Position offset |
| `linkedVrm` | `number \| null` | Linked VRM entity ID, or `null` |

### `WebviewPatchRequest`

| Field | Type | Description |
|-------|------|-------------|
| `offset` | `Vec2` | New position offset |
| `size` | `Vec2` | New 3D dimensions |
| `viewportSize` | `Vec2` | New pixel dimensions |

:::tip Building a React WebView UI
For a step-by-step guide to building a React app with Vite and the `@hmcs/ui` component library, see [Setup & Build](../webview-ui/setup-and-build) and [Component Library](../webview-ui/component-library).
:::

## Next Steps

- **[Signals](./signals)** -- Cross-process pub/sub messaging between scripts and WebViews
