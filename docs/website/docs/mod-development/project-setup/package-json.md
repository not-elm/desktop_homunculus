---
title: "Package Configuration"
sidebar_position: 2
---

# Package Configuration

Every MOD is an npm package with a `package.json` file. Beyond the standard npm fields, MODs use a `homunculus` field to declare assets, menus, and other engine-specific metadata.

## Overview

A MOD's `package.json` includes:

| Field | Purpose | Required |
|---|---|---|
| `name` | Package name (used to derive asset IDs) | Yes |
| `type` | Must be `"module"` for ES module support | Yes |
| `bin` | On-demand commands (invoked via HTTP API) | No |
| `homunculus` | Engine metadata: service, assets, menus, and tray | Yes |
| `dependencies` | Must include `@hmcs/sdk` when using SDK features | No |

## The `homunculus` Field

The `homunculus` field is what makes a package a MOD. It has four sub-fields:

### `assets`

Declares files bundled with the MOD. Each entry maps an **asset ID** to a file description.

```json
{
  "homunculus": {
    "assets": {
      "<asset-id>": {
        "path": "<relative-path-to-file>",
        "type": "<asset-type>",
        "description": "<human-readable description>"
      }
    }
  }
}
```

**Supported asset types:**

| Type | File Formats | Description |
|---|---|---|
| `vrm` | `.vrm` | 3D character model (VRM 1.0) |
| `vrma` | `.vrma` | VRM animation clip |
| `sound` | `.mp3`, `.wav`, `.ogg` | Sound effect or audio file |
| `image` | `.png`, `.jpg`, `.svg` | Image file |
| `html` | `.html` | WebView UI entry point |

**Example** -- `@hmcs/elmer`, a MOD that spawns a desktop character:

```json
{
  "homunculus": {
    "assets": {
      "elmer:vrm": {
        "path": "assets/Elmer.vrm",
        "type": "vrm",
        "description": "VRM model named Elmer"
      },
      "elmer:open": {
        "path": "assets/open.mp3",
        "type": "sound",
        "description": "Sound effect for opening action"
      }
    }
  }
}
```

:::info[Asset IDs]
Asset IDs must be globally unique. The recommended convention is `<mod-name>:<asset-name>`. See [Asset IDs](./asset-ids.md) for details.
:::

### `menus`

Declares entries for the right-click context menu. Each menu entry triggers a `bin` command when clicked.

```json
{
  "homunculus": {
    "menus": [
      {
        "id": "<unique-id>",
        "text": "<display-label>",
        "command": "<bin-command-name>"
      }
    ]
  }
}
```

**Example** -- the `@hmcs/character-settings` MOD adds a "Character Settings" entry to the context menu:

```json
{
  "homunculus": {
    "menus": [
      {
        "id": "open-character-settings",
        "text": "Character Settings",
        "command": "open-ui"
      }
    ]
  }
}
```

When the user right-clicks the character and selects "Character Settings", the engine invokes the `open-ui` bin command.

### `tray`

Declares an item for the system tray menu. Unlike `menus` (which appear on right-click), tray items appear in the OS menu bar / system tray icon menu and are application-wide.

```json
{
  "homunculus": {
    "tray": {
      "id": "<unique-id>",
      "text": "<display-label>",
      "command": "<bin-command-name>"
    }
  }
}
```

Tray items can also contain nested `items` for submenus. See [Tray Menus](../tray-menus.md) for the full guide.

**Example** -- the `@hmcs/settings` MOD adds a "Settings" entry to the tray:

```json
{
  "homunculus": {
    "tray": {
      "id": "open-settings",
      "text": "Settings",
      "command": "settings-open-ui"
    }
  }
}
```

### `service`

The `homunculus.service` field specifies a **service** — a long-running Node.js process that runs automatically when Desktop Homunculus launches. The engine executes it as a child process using `node --experimental-strip-types`, so you can write TypeScript directly without a build step.

```json
{
  "homunculus": {
    "service": "index.ts"
  }
}
```

Services are typically used to spawn VRM characters and set up behaviors. The service starts at launch and stays alive as long as the app is running.

:::warning
The service script runs every time the app starts. Make sure it handles errors gracefully -- an unhandled exception will cause the script process to exit.
:::

## Bin Commands

The `bin` field exposes on-demand scripts that can be invoked through the HTTP API. Unlike the service script, these scripts run only when explicitly called.

```json
{
  "bin": {
    "<command-name>": "<path-to-script>"
  }
}
```

Bin commands are invoked via `POST /mods/{mod_name}/bin/{command}` with a JSON body. The script receives input from stdin using `input.parse` from `@hmcs/sdk/commands`.

**Example** -- the `@hmcs/voicevox` MOD exposes TTS commands:

```json
{
  "bin": {
    "voicevox:speak": "bin/speak.ts",
    "voicevox:speakers": "bin/speakers.ts",
    "voicevox:initialize": "bin/initialize.ts"
  }
}
```

:::tip
Bin command names are conventionally prefixed with the MOD name (e.g., `voicevox:speak`) to avoid collisions with other MODs.
:::

## Dependencies

If you want to use the SDK, you should include `@hmcs/sdk` as a dependency.

```json
{
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```


