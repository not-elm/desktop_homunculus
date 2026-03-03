---
title: "Claude Desktop"
sidebar_position: 1
---

# Claude Desktop

Set up Claude Desktop to control your Desktop Homunculus characters.

## Prerequisites

- Desktop Homunculus installed and running
- Node.js >= 22

## Configuration

Add the following to your Claude Desktop configuration file:

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "npx",
      "args": ["-y", "@hmcs/mcp-server@latest"]
    }
  }
}
```

## Restart

Restart Claude Desktop after saving the configuration file. The MCP server will start automatically when Claude Desktop launches.

## Verify

Ask Claude:

> "What characters are currently loaded on my desktop?"

If the connection is working, Claude will read the `homunculus://characters` resource and describe your loaded characters.

## Custom Port

If Desktop Homunculus runs on a non-default port, set the `HOMUNCULUS_HOST` environment variable:

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "npx",
      "args": ["-y", "@hmcs/mcp-server@latest"],
      "env": {
        "HOMUNCULUS_HOST": "localhost:4000"
      }
    }
  }
}
```

The default value is `localhost:3100`. You can change the port in `~/.homunculus/config.toml`.

## Next Steps

- [MCP Reference](/docs/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
