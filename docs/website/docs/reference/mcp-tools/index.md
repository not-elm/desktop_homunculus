---
title: "MCP Reference"
sidebar_position: 1
slug: /reference/mcp-tools
---

# MCP Reference

The Desktop Homunculus MCP server exposes 20 tools, 4 resources, and 3 prompts over stdio transport.

Use this page as a map, then open the category page you need.

## MCP Server Overview

The MCP server communicates with Desktop Homunculus over HTTP at `localhost:3100` (configurable via the `HOMUNCULUS_HOST` environment variable).

## Category Map

| Category | Coverage | Link |
|---|---|---|
| Character | 5 tools for spawn/select/remove/persona/snapshot | [Character](./mcp-tools/character) |
| Expression | 4 tools for expressions, reactions, animation, look-at | [Expression](./mcp-tools/expression) |
| Movement | 4 tools for move and tween transforms | [Movement](./mcp-tools/movement) |
| Audio | 3 tools for speech, SFX, and BGM control | [Audio](./mcp-tools/audio) |
| Webview | 3 tools for webview lifecycle and content updates | [Webview](./mcp-tools/webview) |
| MOD | 1 tool for MOD command execution | [MOD](./mcp-tools/mod) |
| Resources | 4 read-only resource endpoints | [Resources](./mcp-tools/resources) |
| Prompts | 3 parameterized workflow prompts | [Prompts](./mcp-tools/prompts) |

## Tool-to-Category Quick Reference

| Tool | Category |
|---|---|
| `get_character_snapshot` | Character |
| `spawn_character` | Character |
| `remove_character` | Character |
| `select_character` | Character |
| `set_persona` | Character |
| `set_expression` | Expression |
| `play_reaction` | Expression |
| `play_animation` | Expression |
| `set_look_at` | Expression |
| `move_character` | Movement |
| `tween_position` | Movement |
| `tween_rotation` | Movement |
| `tween_scale` | Movement |
| `speak_message` | Audio |
| `play_sound` | Audio |
| `control_bgm` | Audio |
| `open_webview` | Webview |
| `close_webview` | Webview |
| `navigate_webview` | Webview |
| `execute_command` | MOD |
