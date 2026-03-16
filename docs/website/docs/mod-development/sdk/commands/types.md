---
sidebar_position: 100
---

# Type Definitions

## StdinParseError

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
