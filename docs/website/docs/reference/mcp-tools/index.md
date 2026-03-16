---
title: "MCP Reference"
sidebar_position: 1
slug: /reference/mcp-tools
---

# MCP Reference

The Desktop Homunculus MCP server exposes 20 tools, 5 resources, and 3 prompts via Streamable HTTP.

Use this page as a map, then open the category page you need.

## MCP Server Overview

The MCP server is built into the Desktop Homunculus engine and accessible at `http://localhost:3100/mcp`. The port can be changed in `~/.homunculus/config.toml`.

## Category Map

| Category | Coverage | Link |
|---|---|---|
| Character | Spawn, select, remove characters; manage persona and snapshot | [Character](./mcp-tools/character) |
| Expression | 4 tools for expressions, reactions, animation, look-at | [Expression](./mcp-tools/expression) |
| Movement | 5 tools for move and tween transforms | [Movement](./mcp-tools/movement) |
| Audio | 3 tools for speech, SFX, and BGM control | [Audio](./mcp-tools/audio) |
| Webview | 3 tools for webview lifecycle and content updates | [Webview](./mcp-tools/webview) |
| MOD | 1 tool for MOD command execution | [MOD](./mcp-tools/mod) |
| RPC | 1 tool for calling MOD service RPC methods | [RPC](./mcp-tools/rpc) |
| Resources | 5 read-only resource endpoints | [Resources](./mcp-tools/resources) |
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
| `play_animation` | Expression |
| `set_look_at` | Expression |
| `move_character` | Movement |
| `tween_position` | Movement |
| `tween_rotation` | Movement |
| `tween_scale` | Movement |
| `play_sound` | Audio |
| `control_bgm` | Audio |
| `open_webview` | Webview |
| `close_webview` | Webview |
| `navigate_webview` | Webview |
| `execute_command` | MOD |
| `spin_character` | Movement |
| `call_rpc` | RPC |
