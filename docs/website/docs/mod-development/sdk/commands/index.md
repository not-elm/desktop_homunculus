---
sidebar_position: 1
---

# commands

Stdin/stdout utilities for MOD command scripts. `@hmcs/sdk/commands` is a **separate entry point** that provides structured input parsing and output helpers for MOD commands declared in your MOD's `package.json` under `"bin"`.

:::warning
Do **not** import `@hmcs/sdk/commands` from a MOD's main script or from browser-side code. It uses `process.stdin` and other Node.js APIs that are only available in MOD command script contexts.
:::

## Import

```typescript
import { input, output } from "@hmcs/sdk/commands";
```

## Functions

### Input

| Function | Description |
|----------|-------------|
| [input.parse](./input-parse) | Read JSON from stdin and validate against a Zod schema |
| [input.parseMenu](./input-parseMenu) | Parse menu command stdin and return a `Vrm` instance |
| [input.read](./input-read) | Read all of stdin as a raw UTF-8 string |

### Output

| Function | Description |
|----------|-------------|
| [output.succeed](./output-succeed) | Write a JSON result to stdout and exit with code 0 |
| [output.fail](./output-fail) | Write a structured error to stderr and exit the process |
| [output.write](./output-write) | Write a JSON result to stdout without exiting |
| [output.writeError](./output-writeError) | Write a structured error to stderr without exiting |
