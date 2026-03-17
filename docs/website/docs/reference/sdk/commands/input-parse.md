---
sidebar_position: 2
---

# input.parse

Read JSON from stdin and validate it against a Zod schema. Returns the validated and typed object.

Performs three steps internally:
1. Reads all of stdin via `input.read()`
2. Parses the raw string as JSON
3. Validates the parsed object against the provided Zod schema

Throws [`StdinParseError`](./types#stdinparseerror) if any step fails.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `schema` | `ZodType<T>` | A Zod schema to validate the parsed JSON against |

## Returns

`Promise<T>`

## Example

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
