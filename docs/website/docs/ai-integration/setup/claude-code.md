---
title: "Claude Code"
sidebar_position: 2
---

# Claude Code

Set up Claude Code to control your Desktop Homunculus characters from the terminal.

## Prerequisites

- Desktop Homunculus installed and running
- Node.js >= 22

## Configuration

Add the Homunculus MCP server to your Claude Code configuration.

**Project-level** (`.mcp.json` in your project root):

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "npx",
      "args": ["-y", "@hmcs/mcp-server@0.1.0"]
    }
  }
}
```

**Global** (`~/.claude/settings.json`):

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "npx",
      "args": ["-y", "@hmcs/mcp-server@0.1.0"]
    }
  }
}
```

## Verify

Ask Claude Code:

> "What characters are currently loaded on my desktop?"

If the connection is working, Claude Code will read the `homunculus://characters` resource and describe your loaded characters.

## Skills

Claude Code **Skills** chain MCP tool calls into complex workflows. For example, the `tech-lecture` skill combines `open_webview`, `speak_message`, and `set_expression` to have a character deliver a full lecture with slides and narration.

Official skills are available in the [`skills/` directory](https://github.com/not-elm/desktop-homunculus/tree/main/skills) of the repository. See the README there for installation instructions and the full catalog.

## Next Steps

- [MCP Reference](/docs/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
