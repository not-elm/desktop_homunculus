---
sidebar_position: 4
---

# exit

Shuts down the Desktop Homunculus application gracefully.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| _(none)_ | — | — |

## Returns

`Promise<void>`

## Example

```typescript
await app.exit();
```

:::warning
`app.exit()` terminates the entire application, including all running MODs. Use with care.
:::
