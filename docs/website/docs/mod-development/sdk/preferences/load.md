---
sidebar_position: 3
---

# load

Retrieves the value for a key. Returns `undefined` if the key does not exist.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `string` | The unique identifier for the stored data |

## Returns

`Promise<V | undefined>`

## Example

```typescript
const theme = await preferences.load<string>("my-mod:theme");
if (theme !== undefined) {
  console.log(`Theme: ${theme}`);
}

interface Settings {
  volume: number;
  notifications: boolean;
}
const settings = await preferences.load<Settings>("my-mod:settings");
```
