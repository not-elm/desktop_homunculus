---
title: "commands"
sidebar_position: 18
---

# commands

Stdin/stdout utilities for bin scripts. `@hmcs/sdk/commands` is a **separate entry point** that provides structured input parsing and output helpers for on-demand commands declared in your MOD's `package.json` under `"bin"`.

:::warning
Do **not** import `@hmcs/sdk/commands` from a MOD's main script or from browser-side code. It uses `process.stdin` and other Node.js APIs that are only available in bin script contexts.
:::

## Import

```typescript
import { input, output, StdinParseError } from "@hmcs/sdk/commands";
```

## Reading Input

Bin commands receive JSON on stdin from the engine when invoked via `POST /mods/{mod_name}/bin/{command}`.

### `input.parse(schema)`

Read JSON from stdin and validate it against a Zod schema. Returns the validated and typed object.

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({
    entity: z.number(),
    text: z.union([z.string(), z.array(z.string())]),
    speaker: z.number().default(0),
  }),
);
```

Performs three steps internally:
1. Reads all of stdin via `input.read()`
2. Parses the raw string as JSON
3. Validates the parsed object against the provided Zod schema

Throws [`StdinParseError`](#stdinparseerror) if any step fails.

### `input.parseMenu()`

Parse menu command stdin and return a `Vrm` instance for the linked character. Menu commands receive `{ "linkedVrm": <entityId> }` on stdin from the menu UI.

```typescript
import { input } from "@hmcs/sdk/commands";

const vrm = await input.parseMenu();
await vrm.setExpressions({ happy: 1.0 });
```

### `input.read()`

Read all of stdin as a raw UTF-8 string. Useful when you need the raw string without JSON parsing or validation.

```typescript
import { input } from "@hmcs/sdk/commands";

const raw = await input.read();
console.log("Received:", raw);
```

## Writing Output

### `output.succeed(data)`

Write a JSON result to stdout and exit the process with code 0. Use this as the final call in a successful bin command.

```typescript
import { output } from "@hmcs/sdk/commands";

output.succeed({ greeting: `Hello, ${data.name}!` });
// stdout: {"greeting":"Hello, World!"}\n
// process exits with code 0
```

### `output.fail(code, message, exitCode?)`

Write a structured error to stderr and exit the process. Default exit code is `1`.

```typescript
import { output } from "@hmcs/sdk/commands";

output.fail("NOT_FOUND", "Entity does not exist");
// stderr: {"code":"NOT_FOUND","message":"Entity does not exist"}\n
// process exits with code 1
```

### `output.write(data)`

Write a JSON result to stdout **without** exiting. Useful for streaming partial results.

```typescript
output.write({ partial: "data" });
// stdout: {"partial":"data"}\n
```

### `output.writeError(code, message)`

Write a structured error to stderr **without** exiting. Useful for non-fatal warnings.

```typescript
output.writeError("WARNING", "non-fatal issue");
// stderr: {"code":"WARNING","message":"non-fatal issue"}\n
```

## Error Handling

### `StdinParseError`

Error thrown by `input.parse()` when stdin is empty, contains invalid JSON, or fails Zod schema validation. The `code` field identifies the failure stage:

| Code | Meaning |
|------|---------|
| `EMPTY_STDIN` | No input received on stdin |
| `INVALID_JSON` | Stdin content is not valid JSON |
| `VALIDATION_ERROR` | JSON does not match the Zod schema (the `details` field contains the `ZodError` instance) |

```typescript
import { input, output, StdinParseError } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(schema);
  output.succeed(await processData(data));
} catch (err) {
  if (err instanceof StdinParseError) {
    output.fail(err.code, err.message);
  }
  throw err;
}
```

## Complete Example

A full bin command script that reads validated input and writes structured output:

```typescript
#!/usr/bin/env tsx
import { z } from "zod";
import { input, output } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({ name: z.string() }),
);

output.succeed({ greeting: `Hello, ${data.name}!` });
```

## Next Steps

- **[Bin Commands](../bin-commands)** -- How to declare and invoke bin commands in your MOD's `package.json`
- **[SDK Quick Start](./quick-start)** -- Installation and first script tutorial
