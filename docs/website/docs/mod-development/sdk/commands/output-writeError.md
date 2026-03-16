---
sidebar_position: 8
---

# output.writeError

Write a structured error to stderr **without** exiting. Useful for non-fatal warnings.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | `string` | A machine-readable error code |
| `message` | `string` | A human-readable error description |

## Returns

`void`

## Example

```typescript
output.writeError("WARNING", "non-fatal issue");
// stderr: {"code":"WARNING","message":"non-fatal issue"}\n
```
