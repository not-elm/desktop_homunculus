---
sidebar_position: 5
---

# output.succeed

Write a JSON result to stdout and exit the process with code 0. Use this as the final call in a successful bin command.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `data` | `unknown` | Optional value to serialize as JSON and write to stdout |

## Returns

`never`

## Example

```typescript
import { output } from "@hmcs/sdk/commands";

output.succeed({ greeting: `Hello, ${data.name}!` });
// stdout: {"greeting":"Hello, World!"}\n
// process exits with code 0
```
