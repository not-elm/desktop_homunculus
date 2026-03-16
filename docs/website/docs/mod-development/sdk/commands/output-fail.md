---
sidebar_position: 6
---

# output.fail

Write a structured error to stderr and exit the process. Default exit code is `1`.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | `string` | A machine-readable error code (e.g., `"NOT_FOUND"`, `"TIMEOUT"`) |
| `message` | `string` | A human-readable error description |
| `exitCode` | `number` | Process exit code (default: `1`) |

## Returns

`never`

## Example

```typescript
import { output } from "@hmcs/sdk/commands";

output.fail("NOT_FOUND", "Entity does not exist");
// stderr: {"code":"NOT_FOUND","message":"Entity does not exist"}\n
// process exits with code 1
```
