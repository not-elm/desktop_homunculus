---
sidebar_position: 4
---

# save

Stores any JSON-serializable value under the given key. Overwrites the previous value if the key already exists.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `string` | The unique identifier for storing the data |
| `value` | `V` | The data to save (must be JSON-serializable) |

## Returns

`Promise<void>`

## Example

```typescript
await preferences.save("my-mod:theme", "dark");

await preferences.save("my-mod:settings", {
  volume: 0.8,
  notifications: true,
});
```

:::note Key naming
Use a `"mod-name:key"` prefix to avoid collisions with other MODs. For example: `"my-mod:theme"`, `"my-mod:settings"`.
:::
