---
title: "Claude Code"
sidebar_position: 2
---

# Claude Code

Set up Claude Code to control your Desktop Homunculus characters from the terminal.

## Prerequisites

- Desktop Homunculus installed and running

## Configuration

Add the Homunculus MCP server to your Claude Code configuration.

**Recommended — CLI** (registers globally for all projects):

```bash
claude mcp add --transport http --scope user homunculus http://localhost:3100/mcp
```

**Project-level** (`.mcp.json` in your project root):

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "http",
      "url": "http://localhost:3100/mcp"
    }
  }
}
```

**Global** (`~/.claude.json` `mcpServers` section, or use the CLI command above):

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "http",
      "url": "http://localhost:3100/mcp"
    }
  }
}
```

> See the [Claude Code documentation](https://docs.anthropic.com/en/docs/claude-code) for the latest configuration options, as the settings format may change between versions.

## Verify

Ask Claude Code:

> "What characters are currently loaded on my desktop?"

If the connection is working, Claude Code will read the `homunculus://characters` resource and describe your loaded characters.

## Skills

Claude Code **Skills** chain MCP tool calls into complex workflows. For example, the `tech-lecture` skill combines `open_webview`, `speak_message`, and `set_expression` to have a character deliver a full lecture with slides and narration.

Official skills are available in the [`skills/` directory](https://github.com/not-elm/desktop_homunculus/tree/main/skills) of the repository. See the README there for installation instructions and the full catalog.

## Custom Port

If Desktop Homunculus runs on a non-default port (changed in `~/.homunculus/config.toml`), update the URL accordingly:

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "http",
      "url": "http://localhost:4000/mcp"
    }
  }
}
```

## Next Steps

- [MCP Reference](/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
