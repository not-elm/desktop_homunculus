---
sidebar_position: 7
---

# output.write

Write a JSON result to stdout **without** exiting. Useful for streaming partial results.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `data` | `unknown` | The value to serialize as JSON and write to stdout |

## Returns

`void`

## Example

```typescript
output.write({ partial: "data" });
// stdout: {"partial":"data"}\n
```
