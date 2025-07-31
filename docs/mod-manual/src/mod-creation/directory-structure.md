# Directory Structure

Proper organization of your MOD files is essential for maintainability and functionality. This page explains how to
structure your MOD directory for optimal development and deployment.

## Basic Structure

Every MOD must be placed in the `assets/mods/` directory with the following basic structure:

```
assets/mods/your-mod-name/
├── mod.json                    # Required: MOD configuration
├── index.html                  # Optional: Main HTML interface
├── icon.png                    # Optional: MOD icon for menus
└── README.md                   # Optional: Documentation
```

## File and Folder Requirements

### Required Files

#### `mod.json` (Required)

The configuration file that defines your MOD's metadata, menus, and behavior. This file must be present in the root of
your MOD directory.

```json
{
  "name": "your-mod",
  "version": "1.0.0",
  "description": "Brief description of your MOD",
  "startupScripts": [
    "your-mod/scripts/index.js"
  ],
  "systemMenus": [
    {
      "text": "Open Dashboard",
      "shortcut": {
        "key": "KeyD",
        "modifiers": "ALT"
      },
      "script": "your-mod/scripts/dashboard.js",
      "webview": {
        "source": "your-mod/dashboard.html"
      }
    }
  ],
  "menus": [
    {
      "text": "Chat",
      "thumbnail": "my-mod/icons/chat.png",
      "webview": {
        "source": "my-mod/chat.html"
      }
    }
  ]
}
```

## Platform Considerations

### Cross-Platform Compatibility

- Use forward slashes (`/`) in all paths, even on Windows
- Avoid platform-specific file names or characters

### macOS Specific

- Avoid `.DS_Store` files by adding them to `.gitignore`
- Be aware of case-sensitive file systems on some macOS configurations

### Windows Specific

- Long path names may cause issues on some Windows versions
- Avoid reserved file names (`CON`, `PRN`, `AUX`, etc.)

## Next Steps

Once you have your directory structure set up, proceed to:

- **[Configuration File (mod.json)](./mod-json.md)** - Define your MOD's behavior and metadata
- **[Menus Configuration](./menus.md)** - Add entries to character context menus