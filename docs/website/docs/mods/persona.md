---
title: "@hmcs/persona"
sidebar_position: 5
---

# @hmcs/persona

The Persona MOD (`@hmcs/persona`) manages character identities and provides default behavior for all personas. It replaces the previous `@hmcs/character-settings` MOD.

## Overview

This MOD provides two things:

1. **Persona Management** — UI for creating, editing, and deleting personas with persistent identity data
2. **Default Behavior Service** — A background process that auto-spawns personas and manages their animation state machine (idle, drag, sitting)

## Persona Management UI

Access from the **system tray** → **"Persona"**.

The management dashboard lets you:

- **List** all personas with thumbnails and names
- **Create** new personas with custom identity fields
- **Edit** existing persona settings
- **Delete** personas you no longer need

## Per-Persona Settings

Access by **right-clicking a character** → **"Persona"**.

The settings panel opens as a floating WebView next to the character. It has two tabs:

### Persona Tab

| Setting | Description |
|---------|-------------|
| Name | Display name |
| Age | Character age |
| Gender | Gender identity (male, female, other, unknown) |
| First-Person Pronoun | How the character refers to itself (e.g., "I", "watashi", "boku") |
| Profile | Background and profile description (free text) |
| Personality | Personality traits in natural language (free text) — used by AI Agent for prompt context |
| Thumbnail | Persona thumbnail image displayed in management UI and menus |

### Appearance Tab

| Setting | Description |
|---------|-------------|
| Bone Scale | Adjust VRM model bone scales to customize character proportions |

Changes take effect after clicking **Save**. All settings are persisted to `~/.homunculus/preferences.db`.

## Default Behavior Service

The service runs automatically when Desktop Homunculus starts. It handles two responsibilities:

### Auto-Spawn

Personas with `metadata['auto-spawn']` set to `true` are automatically spawned at startup. The official `@hmcs/elmer` MOD sets this flag on its persona.

### Animation State Machine

The service listens for `state-change` events on all spawned personas and plays the appropriate animation:

| State | Animation | Additional Behavior |
|-------|-----------|-------------------|
| `idle` | `vrma:idle-maid` (looping) | Cursor tracking enabled |
| `drag` | `vrma:grabbed` (looping) | Cursor tracking disabled, spring bones reset |
| `sitting` | `vrma:idle-sitting` (looping) | Cursor tracking enabled |

:::tip
If you're building a custom MOD that creates personas, you don't need to reimplement idle/drag/sitting logic — the `@hmcs/persona` service handles it automatically for all spawned personas.
:::

## SDK Reference

For programmatic persona management, see the [Persona SDK reference](/reference/sdk/persona).

## Notes

- This MOD adds a "Persona" entry to both the [context menu](./menu) and the system tray menu.
- The settings panel UI uses the shared `@hmcs/ui` component library.
- Personality text is used by the `@hmcs/agent` MOD to build AI agent system prompts.
