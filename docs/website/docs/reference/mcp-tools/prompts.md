---
title: "Prompts"
sidebar_position: 8
---

# Prompts

Prompts are parameterized templates that guide an AI into specific MCP workflows.

### `developer-assistant`

Generate appropriate character reactions for development events.

| Parameter | Type | Description |
|-----------|------|-------------|
| `event` | `string` | Development event: `build-success`, `build-failure`, `test-pass`, `test-fail`, `git-push`, `git-commit`, `deploy` |

The prompt instructs the AI to call `play_reaction` with an appropriate preset based on the event outcome (e.g. `success` for `build-success`, `error` for `test-fail`).

---

### `character-interaction`

Have a natural interaction with the desktop character.

| Parameter | Type | Description |
|-----------|------|-------------|
| `message` | `string` | What to say or do with the character |
| `mood` | `string` | Desired mood: `happy`, `playful`, `serious`, `encouraging` |

The prompt instructs the AI to call `get_character_snapshot`, `play_reaction`, and optionally `speak_message`.

---

### `mod-command-helper`

Discover and execute MOD commands.

| Parameter | Type | Description |
|-----------|------|-------------|
| `mod_name` | `string` | MOD name to explore |

The prompt instructs the AI to read `homunculus://mods` and explain each command with `execute_command` example calls.
