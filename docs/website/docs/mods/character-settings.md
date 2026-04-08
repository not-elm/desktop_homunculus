---
title: "@hmcs/persona"
sidebar_position: 3
---

# @hmcs/persona

The Persona MOD (`@hmcs/persona`) manages persona lifecycle — spawning personas at startup, controlling animations and behavior, and providing settings and management UIs.

## Overview

When Desktop Homunculus starts, the Persona MOD service automatically spawns all personas marked with `auto-spawn: true` and sets up their idle animations, cursor tracking, and interaction behavior.

## Features

### Service (auto-start)

1. **Spawns personas at startup** — Iterates all personas and spawns those with `auto-spawn: true` in metadata
2. **Plays idle animation** (`vrma:idle-maid`) on loop for each spawned persona with a VRM
3. **Follows cursor** — Each persona's eyes track your mouse position
4. **Responds to interactions:**
   - **Dragging** — Switches to the grabbed pose (`vrma:grabbed`) and stops cursor tracking
   - **Sitting on a window edge** — Switches to the sitting animation (`vrma:idle-sitting`)
   - **Releasing** — Returns to idle and resumes cursor tracking

### Per-Persona Settings Panel

Open the settings panel by:

- **Right-clicking a character** and selecting **"Persona"** from the [context menu](./menu)
- The panel opens next to the character as a floating WebView window

The settings panel is organized into tabs. Changes take effect after clicking **Save**. All settings are persisted to `~/.homunculus/preferences.db`.

#### Basic

| Setting | Description | Range / Type |
|---------|-------------|--------------|
| Name    | Display name of the character | — |
| Scale   | Character display size | 0.10 – 3.00 |

#### Persona

| Setting | Description |
|---------|-------------|
| Profile | Character background and profile description (free text) |
| Personality | Personality traits written in natural language (free text) |

#### OCEAN

Adjusts the character's Big Five personality dimensions. Each trait is set with a slider and visualized on a radar chart.

| Trait | Low | High |
|-------|-----|------|
| Openness | Conservative | Curious |
| Conscientiousness | Spontaneous | Organized |
| Extraversion | Introverted | Extroverted |
| Agreeableness | Independent | Cooperative |
| Neuroticism | Stable | Sensitive |

### Persona Management Dashboard

Open the management dashboard from the system tray **"Persona"** entry. This UI allows creating, deleting, and configuring personas.

## Notes

- The Persona MOD adds a "Persona" entry to the [Context Menu](./menu) and a "Persona" entry to the system tray.
- The Persona MOD requires the [@hmcs/assets](./assets) MOD for VRMA animations.
- The settings panel UI uses the shared `@hmcs/ui` component library.
