---
sidebar_position: 3
---

# info

Returns metadata about the running engine instance in a single request -- version string, platform details, compiled features, and all loaded MODs.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| _(none)_ | — | — |

## Returns

`Promise<`[`AppInfo`](./types#appinfo)`>`

## Example

```typescript
const info = await app.info();
console.log(`Engine v${info.version} on ${info.platform.os}/${info.platform.arch}`);
console.log(`Features: ${info.features.join(", ")}`);
console.log(`${info.mods.length} MODs loaded`);

for (const mod of info.mods) {
  console.log(`  ${mod.name}@${mod.version} — ${mod.binCommands.length} commands`);
}
```
