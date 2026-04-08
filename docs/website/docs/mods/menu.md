---
title: "@hmcs/menu"
sidebar_position: 4
---

# @hmcs/menu

The Context Menu MOD (`@hmcs/menu`) provides a right-click menu that appears when you right-click on a character. It displays a WebView-based HUD overlay with available actions.

## Overview

**Right-click on a character** to open the context menu. The menu shows the character's name and a list of available actions. Actions are contributed by other installed MODs — for example, the [Persona](./character-settings) MOD adds a "Persona" entry.

Press **Escape** or click outside the menu to close it.

## Features

MODs register menu entries in their `package.json` under the `"homunculus"` field. Each entry specifies a label and a MOD command to execute when selected. The Context Menu MOD collects all registered entries and displays them.

## Notes

- The menu content depends on which MODs are installed — installing new MODs may add new menu entries.
- The menu UI uses the shared `@hmcs/ui` component library for consistent styling.
