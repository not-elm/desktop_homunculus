# System Menus Configuration

You can add custom system menus to the system tray, allowing users to access MOD functionality.

## Overview

System menus allow users to:

- Access MOD functionality from the system tray icon
- Use keyboard shortcuts for quick actions
- Open global interfaces not tied to specific characters
- Trigger background scripts and maintenance tasks

## Basic System Menu Configuration

Add system menu items to the `systemMenus` array in your `mod.json`:

```json
{
  "systemMenus": [
    {
      "text": "Open Dashboard",
      "shortcut": {
        "key": "KeyD",
        "modifiers": "ALT"
      },
      "script": "my-mod/scripts/dashboard.js",
      "webview": {
        "source": "my-mod/dashboard.html",
        "position": [
          0,
          0
        ],
        "resolution": [
          800,
          600
        ],
        "showToolbar": true,
        "shadow": true,
        "sounds": {
          "open": "my-mod/sounds/dashboard-open.mp3",
          "close": "my-mod/sounds/dashboard-close.mp3"
        }
      }
      ]
    }
```

## System Menu Properties

### `text` (Required)

The display text that appears in the system tray menu.

### `shortcut` (Optional)

Keyboard shortcut for quick access to the menu item.

#### Available Keys

**Letter Keys:**

- `KeyA`, `KeyB`, `KeyC`, ..., `KeyZ`

**Number Keys:**

- `Digit0`, `Digit1`, `Digit2`, ..., `Digit9`

**Function Keys:**

- `F1`, `F2`, `F3`, ..., `F12`

**Special Keys:**

- `Space`, `Enter`, `Escape`, `Tab`, `Backspace`
- `ArrowUp`, `ArrowDown`, `ArrowLeft`, `ArrowRight`

#### Available Modifiers

**Cross-Platform:**

- `CMD_OR_CTRL`: Command key on macOS, Ctrl on Windows/Linux
- `SHIFT`: Shift key
- `ALT`: Alt key (Option on macOS)

**Platform-Specific:**

- `SUPER`: Windows key on Windows, Command on macOS
- `CTRL`: Control key (use `CMD_OR_CTRL` for cross-platform compatibility)

### script (Optional)

A script file to execute when the menu item is clicked.
This script runs in the Deno runtime.

### webview (Optional)

A webview interface to open when the menu item is clicked.
The basic settings are the same as those described in [menus](./menus.md).

## Next Steps

- **[Startup Scripts](./startup-scripts.md)** - Learn about automatic script execution
- **[Webview UI Development](../webview-ui/index.md)** - Build sophisticated interfaces
- **[TypeScript SDK](../sdk/index.md)** - Access advanced programming capabilities