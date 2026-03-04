---
title: "Tray Menus"
sidebar_position: 9
---

# Tray Menus

MODs can add items to the **system tray menu** — the menu that appears when the user clicks the Desktop Homunculus icon in the OS menu bar (macOS) or system tray (Windows). Unlike [context menus](./menus.md) which are tied to a specific character, tray menus are application-wide and useful for global actions like opening settings panels.

When Desktop Homunculus starts, it reads the `homunculus.tray` field from every installed MOD's `package.json` and registers all tray menu items. When the user clicks the tray icon, all registered items appear in the menu.

## Declaring a Tray Menu Item

Add a `tray` object to the `homunculus` field in your `package.json`. Each entry defines one menu item:

```json
{
  "name": "my-mod",
  "type": "module",
  "homunculus": {
    "tray": {
      "id": "open-panel",
      "text": "Open Panel",
      "command": "open-panel"
    }
  },
  "bin": {
    "open-panel": "commands/open-panel.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```

### Tray Item Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Unique identifier for this tray item within the MOD |
| `text` | `string` | Label displayed in the tray menu |
| `command` | `string` | Bin command to execute when selected (must match a key in `bin`) |
| `items` | `TrayItem[]` | (Optional) Nested child items for creating a submenu |

:::warning
The `command` value must exactly match a key in the `bin` field of your `package.json`. If the command is not found, the menu item will appear but do nothing when clicked.
:::

## Submenus

You can create submenus by nesting items with the `items` field. A parent item with `items` acts as a submenu container — it does not need a `command` itself.

```json
{
  "homunculus": {
    "tray": {
      "id": "tools",
      "text": "Tools",
      "items": [
        {
          "id": "tool-a",
          "text": "Tool A",
          "command": "run-tool-a"
        },
        {
          "id": "tool-b",
          "text": "Tool B",
          "command": "run-tool-b"
        }
      ]
    }
  }
}
```

## Handling Tray Commands

Unlike context menu commands which receive a character entity via stdin, tray commands receive **no stdin input**. They run as simple fire-and-forget scripts. This is because tray actions are application-wide, not tied to a specific character.

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";

try {
  await Webview.open({
    source: webviewSource.local("my-mod:ui"),
    size: [0.6, 0.6],
    viewportSize: [500, 400],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e);
}
```

## Tray vs. Context Menus

| Feature | Tray Menu | Context Menu |
|---------|-----------|--------------|
| Trigger | Click tray icon | Right-click a character |
| Scope | Application-wide | Per-character |
| Stdin input | None | `{ "linkedVrm": <entity> }` |
| Declaration | `homunculus.tray` (single object) | `homunculus.menus` (array) |
| Submenus | Supported via `items` | Not supported |

## Complete Example

This example adds a "Settings" entry to the system tray that opens an application settings panel.

**`package.json`**:

```json
{
  "name": "@hmcs/settings",
  "version": "1.0.0",
  "type": "module",
  "homunculus": {
    "tray": {
      "id": "open-settings",
      "text": "Settings",
      "command": "settings-open-ui"
    },
    "assets": {
      "settings:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "Application settings panel UI"
      }
    }
  },
  "bin": {
    "settings-open-ui": "commands/open-ui.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "workspace:*"
  }
}
```

**`commands/open-ui.ts`**:

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { audio, Webview, webviewSource } from "@hmcs/sdk";

try {
  await Webview.open({
    source: webviewSource.local("settings:ui"),
    size: [0.6, 0.6],
    viewportSize: [500, 400],
    offset: [1.1, 0],
  });
  await audio.se.play("se:open");
} catch (e) {
  console.error(e);
}
```

## Related Pages

- **[Context Menus](./menus.md)** -- Right-click character menus
- **[Bin Commands](./bin-commands.md)** -- Writing and invoking on-demand scripts
- **[Webviews](./sdk/webviews)** -- Embedding HTML UIs in 3D space
- **[Package Configuration](./project-setup/package-json.md)** -- Full `package.json` reference
