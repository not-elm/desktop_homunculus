---
title: "Webview"
sidebar_position: 1
---

# Webview

Create and control embedded HTML interfaces in 3D space. WebViews can float freely or be linked to a persona so they follow as the character moves.

For building a React UI with Vite and `@hmcs/ui`, see [WebView UI Setup](/mod-development/webview-ui/setup-and-build).

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";
```

## Static Methods

| Method | Description |
|--------|-------------|
| [`Webview.open(options)`](./open) | Create a new webview and return a `Webview` instance |
| [`Webview.list()`](./list) | Get all open webviews |
| [`Webview.current()`](./current) | Get the current webview when running inside a webview context |

## Instance Methods

| Method | Description |
|--------|-------------|
| [`close()`](./close) | Close the webview |
| [`isClosed()`](./isClosed) | Check if the webview has been closed |
| [`info()`](./info) | Get information about the webview |
| [`patch(options)`](./patch) | Update multiple properties at once |
| [`setOffset(offset)`](./setOffset) | Set the position offset |
| [`setSize(size)`](./setSize) | Set the 3D world space dimensions |
| [`setViewportSize(size)`](./setViewportSize) | Set the HTML pixel dimensions |
| [`navigate(source)`](./navigate) | Navigate to a new source |
| [`reload()`](./reload) | Reload the current content |
| [`navigateBack()`](./navigateBack) | Navigate back in history |
| [`navigateForward()`](./navigateForward) | Navigate forward in history |
| [`linkedPersona()`](./linkedVrm) | Get the persona linked to this webview |
| [`setLinkedPersona(personaId)`](./setLinkedVrm) | Link this webview to a persona |
| [`unlinkPersona()`](./unlinkVrm) | Remove the persona link |

## Helpers

| Function | Description |
|----------|-------------|
| [`webviewSource.local(id)`](./webviewSource-local) | Create a local asset source |
| [`webviewSource.url(url)`](./webviewSource-url) | Create a URL source |
| [`webviewSource.html(content)`](./webviewSource-html) | Create an inline HTML source |

## Type Guards

| Function | Description |
|----------|-------------|
| [`isWebviewSourceLocal(source)`](./isWebviewSourceLocal) | Check if a `WebviewSource` is a local source |
| [`isWebviewSourceUrl(source)`](./isWebviewSourceUrl) | Check if a `WebviewSource` is a URL source |
| [`isWebviewSourceHtml(source)`](./isWebviewSourceHtml) | Check if a `WebviewSource` is an inline HTML source |
| [`isWebviewSourceInfoLocal(source)`](./isWebviewSourceInfoLocal) | Check if a `WebviewSourceInfo` is a local source |
| [`isWebviewSourceInfoUrl(source)`](./isWebviewSourceInfoUrl) | Check if a `WebviewSourceInfo` is a URL source |
| [`isWebviewSourceInfoHtml(source)`](./isWebviewSourceInfoHtml) | Check if a `WebviewSourceInfo` is an inline HTML source |

## Next Steps

- **[Signals](../signals/)** -- Cross-process pub/sub messaging between scripts and WebViews
