# Configuration File (mod.json)

The `mod.json` file is the heart of every MOD, defining its metadata, behavior, and integration points with Desktop
Homunculus. This configuration file must be present in the root directory of your MOD.

The metadata fields such as name and version are not currently in use, but they are expected to be utilized once the mod
registry becomes publicly available.

## Basic Structure

Here's the complete structure of a `mod.json` file with all possible fields:

```json
{
  "name": "string",
  "version": "string",
  "description": "string",
  "author": "string",
  "license": "string",
  "startupScripts": [
    "array of script references"
  ],
  "systemMenus": [
    "array of system menu objects"
  ],
  "menus": [
    "array of context menu objects"
  ]
}
```

## Required Fields

### `name` (Required)

The unique identifier for your MOD. This must match your MOD directory name.

```json
{
  "name": "my-awesome-mod"
}
```

**Rules:**

- Should be unique across all MODs

### `version` (Required)

The version of your MOD using semantic versioning.

```json
{
  "version": "1.0.0"
}
```

**Format:** `MAJOR.MINOR.PATCH`

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in a backwards-compatible manner
- **PATCH**: Backwards-compatible bug fixes

## Optional Metadata Fields

### `description` (Optional)

A brief description of what your MOD does.

```json
{
  "description": "A comprehensive chat interface with AI integration and custom themes"
}
```

### `author` (Optional)

The name or username of the MOD creator.

```json
{
  "author": "YourUsername"
}
```

### `license` (Optional)

The license under which your MOD is released.

```json
{
  "license": "MIT"
}
```

## Functional Configuration Fields

### `startupScripts` (Optional)

An array of JavaScript/TypeScript files to execute when Desktop Homunculus starts.

**Currently, the execution order is not guaranteed.**

```json
{
  "startupScripts": [
    "my-mod/scripts/initialization.js",
    "my-mod/scripts/background-tasks.js"
  ]
}
```

**Important Notes:**

- Scripts are executed in the builtin Deno runtime
- Scripts have access to the full TypeScript SDK via `Deno.api`
    - Please refer to the [SDK reference](../sdk/index.md) for available functions
- Scripts run automatically on application startup
- Use for initialization, background tasks, and automated behaviors
- When a script is changed, it is hot-reloaded and executed again.

### `systemMenus` (Optional)

Defines menu items that appear in the system tray menu.
Please refer to [System Menus Configuration](./system-menus.md) for details.

```json
{
  "systemMenus": [
    {
      "text": "Open Settings",
      "shortcut": {
        "key": "KeyS",
        "modifiers": [
          "Control",
          "Shift"
        ]
      },
      "webview": {
        "source": "my-mod/settings.html",
        "resolution": [
          400,
          300
        ]
      }
    },
    {
      "text": "Run Maintenance",
      "script": "my-mod/scripts/maintenance.js"
    }
  ]
}
```

### `menus` (Optional)

Defines menu items that appear when right-clicking on VRM characters.
Please refer to [Menus Configuration](./menus.md) for details.

```json
{
  "menus": [
    {
      "thumbnail": "my-mod/icons/chat.png",
      "text": "Start Chat",
      "webview": {
        "source": "my-mod/chat.html",
        "resolution": [
          500,
          400
        ],
        "transparent": true,
        "position": {
          "bone": "head",
          "offset": [
            0,
            100
          ],
          "tracking": true
        },
        "showToolbar": false
      }
    }
  ]
}
```

## Complete Example

Here's a comprehensive example showing all features:

```json
{
  "name": "character-companion",
  "version": "2.1.0",
  "description": "A comprehensive character interaction system with chat, emotions, and activities",
  "author": "ModDeveloper",
  "license": "MIT",
  "startupScripts": [
    "character-companion/scripts/init.js",
    "character-companion/scripts/emotion-system.js",
    "character-companion/scripts/activity-scheduler.js"
  ],
  "systemMenus": [
    {
      "text": "Character Settings",
      "shortcut": {
        "key": "KeyP",
        "modifiers": [
          "Control"
        ]
      },
      "webview": {
        "source": "character-companion/ui/settings.html",
        "resolution": [
          600,
          400
        ]
      }
    },
    {
      "text": "Reset All Characters",
      "script": "character-companion/scripts/reset-characters.js"
    }
  ],
  "menus": [
    {
      "thumbnail": "character-companion/icons/chat.png",
      "text": "Chat",
      "webview": {
        "source": "character-companion/ui/chat.html",
        "resolution": [
          400,
          300
        ],
        "transparent": true,
        "position": {
          "bone": "head",
          "offset": [
            50,
            -50
          ],
          "tracking": true
        },
        "openSound": "character-companion/sounds/chat-open.mp3",
        "closeSound": "character-companion/sounds/chat-close.mp3"
      }
    },
    {
      "thumbnail": "character-companion/icons/activities.png",
      "text": "Activities",
      "webview": {
        "source": "character-companion/ui/activities.html",
        "resolution": [
          350,
          250
        ],
        "showToolbar": false
      }
    }
  ]
}
```

## Next Steps

Now that you understand `mod.json` configuration, learn about specific menu types:

- **[Menus Configuration](./menus.md)** - Character context menus
- **[System Menus Configuration](./system-menus.md)** - System tray integration
- **[Startup Scripts](./startup-scripts.md)** - Automated initialization