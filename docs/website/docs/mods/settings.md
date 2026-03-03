---
title: "@hmcs/settings"
sidebar_position: 6
---

# @hmcs/settings

The Settings MOD (`@hmcs/settings`) provides an application-wide settings panel for tuning rendering and display preferences. It registers a **system tray menu** entry so you can open it without right-clicking a character.

## Overview

Open the settings panel from the **system tray icon**:

1. Click the Desktop Homunculus tray icon in your OS menu bar / system tray
2. Select **"Settings"**
3. The settings panel opens as a floating WebView window

## Features

Changes take effect after clicking **Save**. All settings are persisted to `~/.homunculus/preferences.db`.

### Frame Rate

| Setting | Description | Range |
|---------|-------------|-------|
| Frame Rate | Controls the rendering frame rate. Lower values reduce CPU/GPU usage. | 1+ fps |

### Shadow Opacity

| Setting | Description | Range |
|---------|-------------|-------|
| Shadow Opacity | Controls the transparency of the shadow panel overlay behind the character. | 0 – 100% |

## SDK

The `settings` namespace in `@hmcs/sdk` provides programmatic access to application settings:

```typescript
import { settings } from "@hmcs/sdk";

// Read the current FPS
const currentFps = await settings.fps();

// Set a new FPS value
await settings.setFps(30);
```

See the [Settings SDK reference](../mod-development/sdk/settings) for full API documentation.

## Notes

- The Settings MOD uses a [tray menu](../mod-development/tray-menus) instead of a context menu, because application settings are not tied to a specific character.
- The settings panel UI uses the shared `@hmcs/ui` component library.
