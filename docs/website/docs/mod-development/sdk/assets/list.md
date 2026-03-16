---
sidebar_position: 2
---

# list

Returns all registered assets, optionally filtered by type and/or MOD name.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `filter` | [`AssetFilter`](./types#assetfilter) (optional) | Filter criteria: `type` and/or `mod` |

## Returns

`Promise<`[`AssetInfo`](./types#assetinfo)`[]>`

## Example

```typescript
// Get all assets
const all = await assets.list();

// Get only VRM models
const vrms = await assets.list({ type: "vrm" });

// Get assets from a specific mod
const elmerAssets = await assets.list({ mod: "elmer" });

// Combine filters
const sounds = await assets.list({ type: "sound", mod: "my-mod" });
```
