---
sidebar_position: 3
---

# get

Retrieves detailed information about a single MOD by name.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `modName` | `string` | The mod package name |

## Returns

`Promise<`[`ModInfo`](./types#modinfo)`>`

## Example

```typescript
const elmer = await mods.get("elmer");
console.log("Elmer assets:", elmer.asset_ids);
```
