---
title: "Codex"
sidebar_position: 3
---

# Codex

Set up Codex to control your Desktop Homunculus characters.

## Prerequisites

- Desktop Homunculus installed and running
- Node.js >= 22

## Configuration

Pass the MCP server configuration via the `--mcp-config` flag:

```bash
codex --mcp-config '{"homunculus":{"command":"npx","args":["-y","@hmcs/mcp-server@0.1.0"]}}'
```

## Verify

Ask Codex:

> "What characters are currently loaded on my desktop?"

If the connection is working, Codex will read the `homunculus://characters` resource and describe your loaded characters.

## Next Steps

- [MCP Reference](/docs/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
