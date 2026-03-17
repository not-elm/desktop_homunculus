---
sidebar_position: 100
---

# Type Definitions

## RpcMethodDef\<I, O\>

A typed RPC method definition created by [`rpc.method()`](./method).

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | `string` | No | Human-readable description |
| `timeout` | `number` | No | Timeout in milliseconds |
| `input` | `ZodType<I>` | No | Zod schema for input validation |
| `handler` | `(params: I) => Promise<O>` | Yes | Request handler |

## RpcHandlerFn\<O\>

`(params: unknown) => Promise<O>`

A plain async function that can be used as an RPC method handler. The raw parsed JSON body is passed directly without validation.

## RpcMethodEntry

`RpcMethodDef | RpcHandlerFn`

Union type accepted by [`rpc.serve()`](./serve). You can mix `rpc.method()` definitions and plain functions in the same `methods` map.

## RpcServeOptions

| Field | Type | Description |
|-------|------|-------------|
| `methods` | `Record<string, RpcMethodEntry>` | Method name → definition map |

## RpcServer

The return value of [`rpc.serve()`](./serve).

| Field | Type | Description |
|-------|------|-------------|
| `port` | `number` | Port the server is listening on |
| `close` | `() => Promise<void>` | Gracefully shuts down the server |
