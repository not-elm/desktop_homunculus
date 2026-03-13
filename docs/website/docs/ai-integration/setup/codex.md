---
title: "Codex"
sidebar_position: 3
---

# Codex

Set up Codex to control your Desktop Homunculus characters.

## Prerequisites

- Desktop Homunculus installed and running

## Configuration

Register the MCP server with `codex mcp add`:

```bash
codex mcp add --transport http homunculus http://localhost:3100/mcp
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

If Desktop Homunculus runs on a non-default port (changed in `~/.homunculus/config.toml`), update the URL accordingly:

```bash
codex mcp remove homunculus
codex mcp add --transport http homunculus http://localhost:4000/mcp
```

## Next Steps

- [MCP Reference](/reference/mcp-tools) — Explore all available tools, resources, and prompts
- [Troubleshooting](../troubleshooting) — Common issues and solutions
