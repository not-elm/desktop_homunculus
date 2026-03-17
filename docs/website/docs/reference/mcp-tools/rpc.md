---
title: "RPC"
sidebar_position: 6
---

# RPC

Call registered RPC methods on MOD services.

#### `call_rpc`

Call a stateful MOD service RPC method. The engine proxies the request to the MOD's local HTTP server.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `mod_name` | `string` | **required** | Target MOD package name |
| `method` | `string` | **required** | Method name to invoke |
| `body` | `object` | — | JSON payload forwarded to the handler |

Returns the method's JSON response as a string.

:::note
The default timeout for `call_rpc` via MCP is **10 seconds** (unlike the HTTP `POST /rpc/call` endpoint which defaults to 30 seconds). Per-method timeouts configured during registration are respected by both.
:::

:::note
Unlike the HTTP `/rpc/call` endpoint, the MCP tool performs a strict method lookup. If a MOD is pre-registered but has not yet called `/rpc/register` to populate its methods, `call_rpc` will return an error.
:::
