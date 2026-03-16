---
sidebar_position: 6
---

# menus

Returns all context menu entries registered across installed MODs. Each entry is declared in a MOD's `package.json` under the `homunculus.menus` field.

## Parameters

None.

## Returns

`Promise<ModMenuMetadata[]>`

## Example

```typescript
const menuItems = await mods.menus();
for (const item of menuItems) {
  console.log(`${item.modName}: ${item.text} -> ${item.command}`);
}
```
