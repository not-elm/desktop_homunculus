---
title: "Webview.current"
sidebar_position: 4
---

# Webview.current

When code runs inside a WebView (e.g., in a React app loaded by CEF), `Webview.current()` returns a handle to that webview. Returns `undefined` outside a WebView context.

```typescript
static current(): Webview | undefined
```

## Returns

The current `Webview` instance, or `undefined` if not in a webview context.

## Example

```typescript
const webview = Webview.current();
if (webview) {
  const info = await webview.info();
  console.log("Viewport size:", info.viewportSize);
}
```

`Webview.current()` reads the `window.WEBVIEW_ENTITY` value that CEF injects into every webview context.
