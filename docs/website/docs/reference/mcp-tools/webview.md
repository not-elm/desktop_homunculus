---
title: "Webview"
sidebar_position: 5
---

# Webview

Webview tools open, close, and update CEF panels attached near characters.

Webviews are CEF-based browser panels attached near a character. `open_webview` returns an entity ID used by `close_webview` and `navigate_webview`.

#### `open_webview`

Open a webview panel displaying HTML content or a URL near the active character.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `html` | `string` | — | Inline HTML content. Mutually exclusive with `url`. |
| `url` | `string` | — | URL or MOD asset path. Mutually exclusive with `html`. |
| `size_x` | `number` | `0.7` | Panel width in world units |
| `size_y` | `number` | `0.5` | Panel height in world units |
| `viewport_width` | `number` | `800` | Browser viewport width in pixels |
| `viewport_height` | `number` | `600` | Browser viewport height in pixels |
| `offset_x` | `number` | `0` | Horizontal offset from character center |
| `offset_y` | `number` | `0.5` | Vertical offset from character center (positive = above) |

Either `html` or `url` is required.

**Example — display a styled card above the character:**

```json
{
  "html": "<html><body style='background:#1e1e2e;color:#cdd6f4;font-family:sans-serif;padding:16px'><h2>Build succeeded</h2></body></html>",
  "size_x": 0.8,
  "size_y": 0.3
}
```

---

#### `close_webview`

Close one or all webview panels.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `entity` | `number` | — | Entity ID of the webview to close. If omitted, closes the most recently opened webview. |
| `all` | `boolean` | `false` | Close all open webviews |

---

#### `navigate_webview`

Update an existing webview's HTML content without closing and reopening it.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `entity` | `number` | — | Webview entity ID. If omitted, targets the most recently opened webview. |
| `html` | `string` | **required** | New inline HTML content to display |

