---
title: "Settings"
sidebar_position: 5
---

# Settings

The Settings MOD (`@hmcs/settings`) provides a WebView-based settings panel for configuring characters and application preferences.

## Usage

Open the settings panel in one of these ways:

- **Right-click a character** and select **"Settings"** from the [context menu](./menu)
- The panel opens next to the character as a floating WebView window

## What You Can Configure

The settings panel lets you adjust character and application preferences through a visual interface. Changes are saved automatically to the preferences database (`~/.homunculus/prefs.db`).

## Notes

- The Settings MOD adds a "Settings" entry to the [Context Menu](./menu).
- The settings panel UI uses the shared `@hmcs/ui` component library.
