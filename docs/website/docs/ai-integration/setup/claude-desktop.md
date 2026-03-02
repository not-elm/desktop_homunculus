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
      "args": ["-y", "@hmcs/mcp-server@0.1.0"]
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

## Next Steps

- [MCP Reference](/docs/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
