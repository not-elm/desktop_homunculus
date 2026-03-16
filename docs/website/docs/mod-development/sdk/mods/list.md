---
sidebar_position: 2
---

# list

Returns metadata for every MOD discovered at startup.

## Parameters

None.

## Returns

`Promise<ModInfo[]>`

## Example

```typescript
const allMods = await mods.list();
console.log(`${allMods.length} mods installed`);

// Find mods that expose bin commands
const withCommands = allMods.filter((m) => m.bin_commands.length > 0);
```
