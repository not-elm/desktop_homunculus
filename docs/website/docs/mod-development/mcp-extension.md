---
sidebar_position: 12
title: "MCP Extension API (v2)"
---

# Extending the Homunculus MCP Server

Starting with `@hmcs/sdk` v2, mods declare MCP Tools, Prompts, and Resources through the official
[`@modelcontextprotocol/sdk`](https://github.com/modelcontextprotocol/typescript-sdk) (v1.x). Mod-defined RPC
methods are no longer auto-exposed as MCP tools — a deliberate break to separate responsibilities.

## Quick start

```typescript
import { mcp } from '@hmcs/sdk/mcp';
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { z } from 'zod';

function build(): McpServer {
  const server = new McpServer({ name: '@my/mod', version: '1.0.0' });
  server.registerTool(
    'hello',
    {
      description: 'Say hello',
      inputSchema: { name: z.string() },
    },
    async ({ name }) => ({ content: [{ type: 'text', text: `Hello, ${name}` }] }),
  );
  return server;
}

await mcp.serve({ createServer: build, slug: 'mymod' });
```

## Migrating from v1

| Before (v1 `rpc.method`) | After (v2 `mcp.serve`) |
|---|---|
| `rpc.method({ description, inputSchema, meta: { category } })` | `McpServer.registerTool(name, { description, inputSchema }, handler)` |
| Auto-exposed as `rpc_<mod>_<method>` | Registered as `<slug>__<name>` via proxy |
| Metadata (title, annotations, icons) on `rpc.method` | Use official SDK equivalents |

Internal RPC methods (for mod-to-mod HTTP calls) keep using `rpc.method` / `rpc.serve`; they just no longer
appear on the MCP endpoint.

## Naming rules

- **Tools/Prompts**: `<slug>__<name>`. Slugs match `/^[a-z][a-z0-9_]*$/`.
- **Resources**: use `<slug>://…` URI scheme. Other schemes trigger a warning.
- `homunculus://` is reserved for built-in engine resources.

## Lifecycle

`mcp.serve()` registers the mod's MCP endpoint with the engine at startup. When the mod process
exits (SIGTERM or crash), the engine detects the exit and removes the registration. A re-registration with
the same slug replaces the previous one (upsert).

## Phase 1 limitations

- Sampling and Elicitation are not proxied (planned for Phase 2).
- Logging notifications from mod → engine are recorded in engine logs but not forwarded to the upstream
  MCP client.
- Pagination: `tools/list`, `prompts/list`, `resources/list` return all entries at once. A soft warning
  logs at 1000+ entries.
