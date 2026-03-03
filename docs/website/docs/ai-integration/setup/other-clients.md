---
title: "Other MCP Clients"
sidebar_position: 4
---

# Other MCP Clients

Any MCP-compatible client can connect to Desktop Homunculus. The MCP server uses **stdio transport** — the client spawns the server as a subprocess and communicates over stdin/stdout.

## Prerequisites

- Desktop Homunculus installed and running
- Node.js >= 22

## Server Configuration

Configure your MCP client to spawn the server with:

- **Command:** `npx`
- **Arguments:** `["-y", "@hmcs/mcp-server@latest"]`
- **Transport:** stdio

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `HOMUNCULUS_HOST` | `localhost:3100` | Host and port of the Desktop Homunculus HTTP API |

Set `HOMUNCULUS_HOST` if Desktop Homunculus is running on a non-default port.

## Verify

Once connected, read the `homunculus://characters` resource. If it returns character data, the connection is working.

## Next Steps

- [MCP Reference](/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [MCP Protocol Specification](https://modelcontextprotocol.io) — Official MCP documentation
- [Troubleshooting](../troubleshooting) — Common issues and solutions
