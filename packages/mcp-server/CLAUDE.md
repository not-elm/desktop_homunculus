# @hmcs/mcp-server

MCP server for Desktop Homunculus. Exposes desktop mascot character control to AI agents via Model Context Protocol (stdio transport).

## Commands

```bash
pnpm install             # Install dependencies
pnpm build               # Compile TypeScript → dist/
pnpm dev                 # Watch mode
pnpm start               # Run the MCP server
pnpm test                # Run unit tests (vitest)
pnpm check-types         # Type-check without emitting
```

## Architecture

```
[AI Agent] <--stdio--> [MCP Server (Node.js)] <--HTTP--> [Desktop Homunculus :3100]
```

- **20 Tools**: get_character_snapshot, play_reaction, play_sound, execute_command, speak_message, select_character, move_character, control_bgm, tween_position, tween_rotation, tween_scale, spawn_character, remove_character, set_expression, play_animation, set_persona, open_webview, close_webview, navigate_webview, set_look_at
- **4 Resources**: homunculus://info, homunculus://characters, homunculus://mods, homunculus://assets
- **3 Prompts**: developer-assistant, character-interaction, mod-command-helper

## AI Client Configuration

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "node",
      "args": ["/path/to/desktop-homunculus/packages/mcp-server/dist/index.js"]
    }
  }
}
```

### Claude Code

Add to project's `.mcp.json` or `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "node",
      "args": ["/path/to/desktop-homunculus/packages/mcp-server/dist/index.js"]
    }
  }
}
```

### Codex

```bash
codex mcp add homunculus -- node /path/to/desktop-homunculus/packages/mcp-server/dist/index.js
```

Verify registration:

```bash
codex mcp list
```

### Custom host (non-default port)

Set the `HOMUNCULUS_HOST` environment variable.

For Claude Desktop / Claude Code:

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "node",
      "args": ["/path/to/desktop-homunculus/packages/mcp-server/dist/index.js"],
      "env": {
        "HOMUNCULUS_HOST": "localhost:4000"
      }
    }
  }
}
```

For Codex:

```bash
codex mcp add homunculus --env HOMUNCULUS_HOST=localhost:4000 -- node /path/to/desktop-homunculus/packages/mcp-server/dist/index.js
```

If `homunculus` is already registered in Codex, remove it first and re-add:

```bash
codex mcp remove homunculus
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOMUNCULUS_HOST` | `localhost:3100` | Desktop Homunculus HTTP server address |
