---
title: "Settings"
sidebar_position: 5
---

# Settings

The Settings MOD (`@hmcs/settings`) provides a WebView-based settings panel for configuring characters and application preferences.

## Overview

Open the settings panel in one of these ways:

- **Right-click a character** and select **"Settings"** from the [context menu](./menu)
- The panel opens next to the character as a floating WebView window

## Features

The settings panel is organized into three tabs. Changes take effect after clicking **Save**. All settings are persisted to `~/.homunculus/preferences.db`.

### Basic

| Setting | Description | Range / Type |
|---------|-------------|--------------|
| Name    | Display name of the character (read-only) | — |
| Scale   | Character display size | 0.10 – 3.00 |

### Persona

| Setting | Description |
|---------|-------------|
| Profile | Character background and profile description (free text) |
| Personality | Personality traits written in natural language (free text) |

### OCEAN

Adjusts the character's Big Five personality dimensions. Each trait is set with a slider and visualized on a radar chart.

| Trait | Low | High |
|-------|-----|------|
| Openness | Conservative | Curious |
| Conscientiousness | Spontaneous | Organized |
| Extraversion | Introverted | Extroverted |
| Agreeableness | Independent | Cooperative |
| Neuroticism | Stable | Sensitive |

## Notes

- The Settings MOD adds a "Settings" entry to the [Context Menu](./menu).
- The settings panel UI uses the shared `@hmcs/ui` component library.
