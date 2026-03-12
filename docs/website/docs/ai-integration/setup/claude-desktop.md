---
title: "Claude Desktop"
sidebar_position: 1
---

# Claude Desktop

Set up Claude Desktop to control your Desktop Homunculus characters.

## Prerequisites

- Desktop Homunculus installed and running

## Configuration

Add the following to your Claude Desktop configuration file:

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "streamable-http",
      "url": "http://localhost:3100/mcp"
    }
  }
}
```

## Restart

Restart Claude Desktop after saving the configuration file.

## Verify

Ask Claude:

> "What characters are currently loaded on my desktop?"

If the connection is working, Claude will read the `homunculus://characters` resource and describe your loaded characters.

## Custom Port

If Desktop Homunculus runs on a non-default port (changed in `~/.homunculus/config.toml`), update the URL accordingly:

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "streamable-http",
      "url": "http://localhost:4000/mcp"
    }
  }
}
```

## Next Steps

- [MCP Reference](/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
