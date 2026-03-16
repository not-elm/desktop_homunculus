---
title: "Other MCP Clients"
sidebar_position: 4
---

# Other MCP Clients

Any MCP-compatible client can connect to Desktop Homunculus. The MCP server uses **Streamable HTTP** — the client connects to the engine's HTTP endpoint directly.

## Prerequisites

- Desktop Homunculus installed and running

## Server Configuration

Configure your MCP client to connect to:

- **URL:** `http://localhost:3100/mcp`
- **Transport:** Streamable HTTP

No separate server process or installation is needed — the MCP server is built into the Desktop Homunculus engine.

## Custom Port

The default port is `3100`. You can change it in `~/.homunculus/config.toml`. Update the URL in your client configuration accordingly.

## Verify

Once connected, read the `homunculus://characters` resource. If it returns character data, the connection is working.

## Next Steps

- [MCP Reference](/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [MCP Protocol Specification](https://modelcontextprotocol.io) — Official MCP documentation
- [Troubleshooting](../troubleshooting) — Common issues and solutions
