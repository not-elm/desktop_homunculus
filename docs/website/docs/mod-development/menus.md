---
title: "Context Menus"
sidebar_position: 8
---

# Context Menus

MODs can add items to the right-click context menu that appears when a user right-clicks a VRM character. Each menu item maps to a [bin command](./bin-commands.md) that runs when the user selects it. The command receives the entity ID of the right-clicked character via stdin, so it can act on that specific character.

When Desktop Homunculus starts, it reads the `homunculus.menus` array from every installed MOD's `package.json` and registers all menu items. When the user right-clicks a character, all registered items appear in the context menu.

## Declaring Menu Items

Add a `menus` array to the `homunculus` field in your `package.json`. Each entry defines one menu item:

```json
{
  "name": "my-mod",
  "type": "module",
  "homunculus": {
    "menus": [
      {
        "id": "open-panel",
        "text": "Open Panel",
        "command": "open-panel"
      }
    ]
  },
  "bin": {
    "open-panel": "commands/open-panel.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```

### Menu Item Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Unique identifier for this menu item within the MOD |
| `text` | `string` | Label displayed in the right-click menu |
| `command` | `string` | Bin command to execute when selected (must match a key in `bin`) |

:::warning
The `command` value must exactly match a key in the `bin` field of your `package.json`. If the command is not found, the menu item will appear but do nothing when clicked.
:::

## Handling Menu Commands

When a user selects a menu item, the corresponding bin command runs with a JSON object on stdin containing the entity ID of the right-clicked character:

```json
{ "linkedVrm": 42 }
```

The `linkedVrm` field is the numeric entity ID of the VRM character that was right-clicked. You can use this ID to look up the character and perform actions on it.

Here is a minimal command handler that parses this input and acts on the character:

```typescript
#!/usr/bin/env -S node --experimental-strip-types

/// <reference types="node" />

import { input } from "@hmcs/sdk/commands";

try {
  const character = await input.parseMenu();
  await character.setExpressions({ happy: 1.0 });
} catch (e) {
  console.error(e);
  process.exit(1);
}
```

`input.parseMenu()` reads the `{ "linkedVrm": ... }` JSON from stdin and returns a `Vrm` instance for the right-clicked character. See [Bin Commands](./bin-commands.md) for full details on shebangs, `input.parse`, error handling, and output conventions.

## Opening a Webview

A common pattern is to open a webview panel from a menu command. This lets you provide rich UI experiences (settings panels, dashboards, etc.) attached to a specific character.

To open a webview, declare an HTML asset in your `package.json` and use `Webview.open()` in the command handler:

**`package.json`** (relevant fields):

```json
{
  "homunculus": {
    "menus": [
      {
        "id": "open-settings",
        "text": "Settings",
        "command": "open-ui"
      }
    ],
    "assets": {
      "my-mod:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "Settings panel UI"
      }
    }
  },
  "bin": {
    "open-ui": "commands/open-ui.ts"
  }
}
```

**`commands/open-ui.ts`**:

```typescript
#!/usr/bin/env -S node --experimental-strip-types

/// <reference types="node" />

import { Webview, webviewSource } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";

try {
  const vrm = await input.parseMenu();
  await Webview.open({
    source: webviewSource.local("my-mod:ui"),
    size: [1, 0.9],
    viewportSize: [900, 700],
    offset: [1.1, 0],
    linkedVrm: vrm.entity,
  });
} catch (e) {
  console.error(e);
}
```

The `linkedVrm` option in `Webview.open()` associates the webview with the right-clicked character. Inside the webview, you can retrieve this association using `Webview.current()` and then `linkedVrm()` to get the VRM instance.

See the [Webviews SDK reference](./sdk/webviews) for all available options and methods.

## Complete Example

This example adds a "Wave" menu item that makes the right-clicked character play a happy expression and an animation.

**`package.json`**:

```json
{
  "name": "my-wave",
  "version": "1.0.0",
  "type": "module",
  "homunculus": {
    "menus": [
      {
        "id": "wave",
        "text": "Wave",
        "command": "wave"
      }
    ]
  },
  "bin": {
    "wave": "commands/wave.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest"
  }
}
```

**`commands/wave.ts`**:

```typescript
#!/usr/bin/env -S node --experimental-strip-types

/// <reference types="node" />

import { repeat } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";

try {
  const character = await input.parseMenu();

  // Show a happy expression
  await character.setExpressions({ happy: 1.0 });

  // Play the idle animation once
  await character.playVrma({
    asset: "vrma:idle-maid",
    repeat: repeat.count(1),
    transitionSecs: 0.3,
  });
} catch (e) {
  console.error(e);
  process.exit(1);
}
```

Install and test:

```bash
hmcs mod install /path/to/my-wave
```

Restart Desktop Homunculus, right-click a character, and select **Wave** from the menu.

## Related Pages

- **[Bin Commands](./bin-commands.md)** -- Writing and invoking on-demand scripts
- **[Webviews](./sdk/webviews)** -- Embedding HTML UIs in 3D space
- **[Mods API](./sdk/mods-api)** -- Programmatic menu listing and command execution
- **[Package Configuration](./project-setup/package-json.md)** -- Full `package.json` reference
