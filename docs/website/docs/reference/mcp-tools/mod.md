---
title: "MOD"
sidebar_position: 6
---

# MOD

MOD integration tools execute packaged MOD commands.

#### `execute_command`

Execute a MOD command. Use the `homunculus://mods` resource to discover available commands per MOD.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `command` | `string` | **required** | Command name to execute |
| `args` | `string[]` | — | Command arguments |
| `stdin` | `string` | — | Standard input (typically JSON) |
| `timeoutMs` | `number` | `30000` | Timeout in milliseconds (range: 1000–300000) |

Returns `stdout`, `stderr`, `exitCode`, and `timedOut`. The tool result `isError` is set when `exitCode !== 0`.

