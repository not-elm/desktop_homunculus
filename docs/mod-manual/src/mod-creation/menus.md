# Menus Configuration

Context menus appear when users right-click on VRM characters, providing quick access to MOD functionality.
This page explains how to configure these menus in your `mod.json` file.

## Overview

Context menus allow users to:

- Access MOD interfaces directly from VRM characters
- Trigger scripts or actions specific to a character
- Open webviews positioned relative to the character
- Provide intuitive interaction points for character-based features

## Basic Menu Configuration

Add menu items to the `menus` array in your `mod.json`:

```json
{
  "menus": [
    {
      "text": "Chat",
      "thumbnail": "my-mod/icons/chat.png",
      "webview": {
        "source": "my-mod/chat.html",
        "resolution": [
          400,
          300
        ],
        "position": {
          "vrm": "CALLER",
          "bone": "head",
          "offset": [
            50,
            0
          ],
          "tracking": true
        },
        "sounds": {
          "open": "my-mod/sounds/open.mp3",
          "close": "my-mod/sounds/close.mp3"
        }
      }
    }
  ]
}
```

## Menu Properties

### `text` (Required)

The text displayed in the context menu.

### `thumbnail` (Optional)

An icon local(relative `assets/mods`) or remote path displayed next to the menu text.

**Specifications:**

- **Format**: PNG, JPG.

### `script` (Optional)

Executes a script when the menu item is clicked.
The script specified here runs in the built-in Deno runtime.

### `webview` (Optional)

Opens a user interface in a webview window:

```json
{
  "webview": {
    "source": "my-mod/stats.html",
    "resolution": [
      300,
      400
    ],
    "transparent": true,
    "position": {
      "bone": "head",
      "offset": [
        50,
        0
      ],
      "tracking": true
    }
  }
}
```

#### source

Webview local file path(relative to `assets/mods/`) or Remote URL.

#### resolution

Set the size of the webview window.

#### transparent

If true, the webview background is transparent.

> [!WARNING]
> Currently, transparent areas block mouse events.

> [!WARNING]
> On Windows, transparency may not work correctly.

#### showToolbar

Hide or show the browser toolbar.
If not specified, the toolbar is shown by default.

#### shadow (only macOS)

Enable display the window drop shadow on macOS.

#### Positioning Options

##### 1. Fixed Position

Position the webview at specific screen coordinates:

```json
{
  "webview": {
    "position": [
      100,
      200
    ]
  }
}
```

##### 2. VRM-Relative Position

Position the webview relative to a VRM avatar.

If not specified `bone`, it is calculated relative to the VRM root bone.

| Property | Description                                                       |
|----------|-------------------------------------------------------------------|
| bone     | Place the webview relative to a specific bone of the VRM          |
| offset   | Specify the offset position from the bone (in screen coordinates) |
| tracking | If true, the webview follows the bone's movement                  |

```json
{
  "webview": {
    "position": {
      "bone": "head",
      "offset": [
        0,
        100
      ],
      "tracking": true
    }
  }
}
```

#### VRM Bone Options

You can attach webviews to various bones:

- **Body**: `"hips"`, `"spine"`, `"chest"`, `"neck"`, `"head"`
- **Arms**: `"leftShoulder"`, `"leftArm"`, `"leftForeArm"`, `"leftHand"`
- **Arms**: `"rightShoulder"`, `"rightArm"`, `"rightForeArm"`, `"rightHand"`
- **Legs**: `"leftUpLeg"`, `"leftLeg"`, `"leftFoot"`
- **Legs**: `"rightUpLeg"`, `"rightLeg"`, `"rightFoot"`

#### tracking

- **`tracking: true`**: Webview follows the character as they move
- **`tracking: false`**: Webview stays at the initial position

#### sounds

Add audio feedback when webviews open and close:

```json
{
  "webview": {
    "sounds": {
      "open": "my-mod/sounds/open.mp3",
      "close": "my-mod/sounds/close.mp3"
    }
  }
}
```

**Supported formats**: MP3, WAV, OGG

## Next Steps

- **[System Menus Configuration](./system-menus.md)** - Learn about system tray integration
- **[Webview UI Development](../webview-ui/index.md)** - Build sophisticated webview interfaces
- **[TypeScript SDK](../sdk/index.md)** - Access advanced programming capabilities