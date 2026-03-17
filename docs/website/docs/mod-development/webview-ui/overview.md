---
title: "WebView UI Overview"
sidebar_position: 1
---

# WebView UI Overview

WebViews are Chromium-based UI panels rendered on top of the Desktop Homunculus window. They let MODs display rich HTML/CSS/JavaScript interfaces -- settings panels, context menus, dashboards -- that communicate with the engine through the `@hmcs/sdk`.

## How WebViews Work

Desktop Homunculus embeds **CEF (Chromium Embedded Framework)** to render HTML content directly inside the Bevy game window. Each webview is a live browser surface composited into the 3D scene.

Key architectural details:

- **3D positioning** -- WebViews can float freely in 3D world space or be linked to a specific VRM character so they follow the character as it moves.
- **Communication path** -- WebView JavaScript imports `@hmcs/sdk`, which makes HTTP calls to the Engine REST API at `localhost:3100`. The API routes requests into the Bevy ECS where they take effect on the next frame.
- **DevTools** -- Press `F1` to open the CEF DevTools panel for any webview, and `F2` to close it.

```
┌─────────────────────────────────────────────────────┐
│  WebView (CEF)                                      │
│  JavaScript  ─── @hmcs/sdk HTTP calls ───►          │
│                                          localhost:3100
│                                          (Axum REST API)
│                                               │     │
│                                          Bevy ECS    │
└─────────────────────────────────────────────────────┘
```

## WebView Sources

Every webview needs a **source** that tells it what to display. There are three source types:

### Local Asset

The most common approach. Points to an HTML file declared as an asset in your MOD's `package.json`:

```typescript
import { webviewSource } from "@hmcs/sdk";

const source = webviewSource.local("my-mod:ui");
```

### Inline HTML

Useful for quick prototyping or simple one-off displays:

```typescript
const source = webviewSource.html("<h1>Hello</h1>");
```

### URL

Loads content from an external URL:

```typescript
const source = webviewSource.url("https://example.com");
```

### Complete Example

Here is a full `Webview.open()` call using a local asset source:

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";

const webview = await Webview.open({
  source: webviewSource.local("my-mod:ui"),
  size: [0.7, 0.7],
  viewportSize: [800, 600],
  offset: [0, 0.5],
});
```

:::tip API Reference
For complete `Webview` API details (methods, options, and types), see
[SDK WebViews](/reference/sdk/webviews).
:::

## When to Use WebViews

- **Complex interactive UIs** -- settings panels, context menus, dashboards, or any interface that needs HTML/CSS/JavaScript.
- **Simple one-off displays** -- for a quick status message or notification, an inline HTML source may be all you need.
- **Rich forms and multi-page UIs** -- if your MOD needs forms, settings editors, or multi-page navigation, build a full React app with Vite and the `@hmcs/ui` component library.

## Next Steps

- **[Setup & Build](./setup-and-build)** -- Build your first WebView UI with React, Vite, and `@hmcs/ui`
