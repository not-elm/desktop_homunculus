---
sidebar_position: 2
---

# list

Returns all stored preference key names.

## Parameters

None.

## Returns

`Promise<string[]>`

## Example

```typescript
const keys = await preferences.list();
console.log(`${keys.length} preferences stored`);

for (const key of keys) {
  const value = await preferences.load(key);
  console.log(`${key}:`, value);
}
```
