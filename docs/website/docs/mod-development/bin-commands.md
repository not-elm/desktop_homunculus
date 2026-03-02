---
title: "Bin Commands"
sidebar_position: 7
---

# Bin Commands

Bin commands are on-demand scripts that MODs expose through the `bin` field in `package.json`. Unlike `main` scripts that run automatically at startup, bin commands run only when explicitly invoked through the HTTP API.

See [Package Configuration](./project-setup/package-json.md#bin-commands) for how to declare the `bin` field in your `package.json`.

## Writing a Bin Command Script

### Shebang Line

Every TypeScript bin command must start with a shebang line that enables direct execution without a compile step:

```typescript
#!/usr/bin/env -S node --experimental-strip-types

/// <reference types="node" />
```

The shebang tells the system to run the file with Node.js using the `--experimental-strip-types` flag, which strips TypeScript syntax at runtime. The `/// <reference types="node" />` directive provides Node.js type definitions (like `process.stdin`).

:::warning
Node.js 22 or later is required for `--experimental-strip-types`. See [Installation](/docs/getting-started/installation) for setup instructions.
:::

### Parsing Input with `input.parse`

Bin commands receive input via stdin as JSON. The SDK provides `input.parse` from `@hmcs/sdk/commands` to read, parse, and validate this input using a Zod schema.

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({
    name: z.string(),
    count: z.number().default(1),
  })
);

console.log(data.name);  // validated string
console.log(data.count); // validated number, defaults to 1
```

`input.parse` performs three steps:

1. **Read** all of stdin as a UTF-8 string
2. **Parse** the string as JSON
3. **Validate** the parsed object against the Zod schema

If any step fails, it throws a `StdinParseError` (see [Error Handling](#error-handling) below).

:::note
`@hmcs/sdk/commands` is a **separate entry point** — it is not re-exported from the main `@hmcs/sdk` package. This is intentional because it uses Node.js APIs (`process.stdin`) that are not available in browser environments like WebViews.
:::

If you need the raw stdin string without JSON parsing or validation, use `input.read` instead:

```typescript
import { input } from "@hmcs/sdk/commands";

const raw = await input.read();
```

### Output Conventions

Bin commands communicate results through stdout and stderr:

| Stream | Purpose | Format |
|--------|---------|--------|
| **stdout** | Command output (results, data) | JSON recommended |
| **stderr** | Errors and diagnostic messages | Free-form text |

The SDK provides helper functions for structured output. Import them from `@hmcs/sdk/commands`:

```typescript
import { output } from "@hmcs/sdk/commands";
```

**Success output** — Write a JSON result to stdout and exit with code 0:

```typescript
output.succeed({ speakers: [...], count: 5 });
```

**Error output** — Write a structured error to stderr and exit with a non-zero code:

```typescript
output.fail("NOT_FOUND", "Speaker 99 does not exist");
// exits with code 1 (default)

output.fail("TIMEOUT", "Request timed out", 2);
// exits with code 2
```

The `output.fail` function writes a JSON object with `code` and `message` fields to stderr:
```json
{"code":"NOT_FOUND","message":"Speaker 99 does not exist"}
```

If you need to write output without exiting (e.g., intermediate progress), use the non-exit variants:

```typescript
import { output } from "@hmcs/sdk/commands";

output.write({ progress: 50 });         // writes to stdout, does not exit
output.writeError("WARN", "retrying");  // writes to stderr, does not exit
```

Exit codes follow standard conventions:

- **`0`** — success
- **non-zero** — failure (the caller sees this in the `exit` event)

### Error Handling

When `input.parse` fails, it throws a `StdinParseError` with one of three error codes:

| Code | Cause |
|------|-------|
| `EMPTY_STDIN` | No input received (stdin was empty or whitespace-only) |
| `INVALID_JSON` | Input is not valid JSON |
| `VALIDATION_ERROR` | JSON does not match the Zod schema |

**Pattern 1: Fail on bad input** — Use when input is required.

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(
    z.object({ linkedVrm: z.number() })
  );
  // ... use data ...
} catch (e) {
  console.error(e);
  process.exit(1);
}
```

:::tip
For menu commands that only need the linked VRM, use [`input.parseMenu()`](./menus.md#handling-menu-commands) instead — it handles the schema and returns a `Vrm` instance directly.
:::

**Pattern 2: Fall back to defaults** — Use when input is optional.

```typescript
import { z } from "zod";
import { input, StdinParseError } from "@hmcs/sdk/commands";

const defaults = { host: "http://localhost:50021" };
let parsed = defaults;
try {
  parsed = await input.parse(
    z.object({ host: z.string().default(defaults.host) })
  );
} catch (err) {
  if (!(err instanceof StdinParseError)) throw err;
  // Use defaults if stdin is empty or malformed
}
```

## Execution

### HTTP API

Bin commands are invoked via the HTTP API:

```
POST http://localhost:3100/commands/execute
Content-Type: application/json
```

### Request Parameters

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `command` | `string` | Yes | Command name to execute (as declared in `bin`) |
| `args` | `string[]` | No | Arguments passed to the script. Max 64 items, each max 4096 characters. |
| `stdin` | `string` | No | Data written to the process stdin. Stdin is closed after writing. Max 1 MiB. |
| `timeoutMs` | `number` | No | Timeout in milliseconds. Range: 1–300,000. Default: 30,000 (30 seconds). |

### Response Format

The response is an NDJSON (newline-delimited JSON) stream. Each line is one of three event types:

**`stdout`** — A line of standard output from the script:
```json
{"type":"stdout","data":"Hello, world!"}
```

**`stderr`** — A line of standard error output:
```json
{"type":"stderr","data":"Warning: using default config"}
```

**`exit`** — The process has exited (always the last event):
```json
{"type":"exit","code":0,"timedOut":false}
```

The `exit` event may also include a `signal` field (e.g., `"15"`) if the process was killed by a signal rather than exiting normally.

### SDK Wrappers

The `@hmcs/sdk` provides two convenience functions for calling bin commands from other MOD scripts:

- **`mods.executeCommand(request)`** — Buffers all output and returns a single `CommandResult` with `stdout`, `stderr`, `exitCode`
- **`mods.streamCommand(request)`** — Returns an `AsyncGenerator<CommandEvent>` for real-time streaming

See the [Mods API](./sdk/mods-api) reference for full details.

## Complete Example

Here is a complete bin command that builds a greeting message based on input parameters.

**`package.json`** (relevant fields):

```json
{
  "name": "my-mod",
  "type": "module",
  "bin": {
    "my-mod:greet": "bin/greet.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest",
    "zod": "^3.25.0"
  }
}
```

**`bin/greet.ts`**:

```typescript
#!/usr/bin/env -S node --experimental-strip-types

/// <reference types="node" />

import { z } from "zod";
import { input, StdinParseError } from "@hmcs/sdk/commands";

// Define input schema with defaults
const schema = z.object({
  name: z.string().default("World"),
  language: z.enum(["en", "ja"]).default("en"),
});

// Parse stdin, fall back to defaults if empty
const defaults = { name: "World", language: "en" as const };
let parsed: z.infer<typeof schema> = defaults;
try {
  parsed = await input.parse(schema);
} catch (err) {
  if (!(err instanceof StdinParseError)) throw err;
}

// Build greeting
const greetings = { en: "Hello", ja: "こんにちは" };
const greeting = greetings[parsed.language];
const message = `${greeting}, ${parsed.name}!`;

// Output as JSON
console.log(JSON.stringify({ message }));
```

**Invoke with `curl`:**

```bash
# With input
curl -X POST http://localhost:3100/commands/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "my-mod:greet", "stdin": "{\"name\": \"Alice\", \"language\": \"ja\"}"}'

# Without input (uses defaults)
curl -X POST http://localhost:3100/commands/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "my-mod:greet"}'
```
