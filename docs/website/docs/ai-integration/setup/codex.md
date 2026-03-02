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

Register the MCP server with `codex mcp add`:

```bash
codex mcp add homunculus -- npx -y @hmcs/mcp-server@latest
```

You can verify registration with:

```bash
codex mcp list
```

## Verify

Ask Codex:

> "What characters are currently loaded on my desktop?"

If the connection is working, Codex will read the `homunculus://characters` resource and describe your loaded characters.

## Custom Port

If Desktop Homunculus runs on a non-default port, set the `HOMUNCULUS_HOST` environment variable when registering the MCP server:

```bash
codex mcp add homunculus --env HOMUNCULUS_HOST=localhost:4000 -- npx -y @hmcs/mcp-server@latest
```

If `homunculus` is already registered, remove it first and then re-add it:

```bash
codex mcp remove homunculus
```

The default value is `localhost:3100`. You can change the port in `~/.homunculus/config.toml`.

## Next Steps

- [MCP Reference](/docs/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
