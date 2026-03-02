---
title: "Assets API"
sidebar_position: 13
---

# Assets API

Query the asset registry -- list and filter assets by type and MOD. Assets are declared in each MOD's `package.json` and referenced by globally unique IDs using the format `"mod-name:asset-name"`.

## Import

```typescript
import { assets } from "@hmcs/sdk";
```

## List Assets

`assets.list(filter?)` returns all registered assets, optionally filtered by type and/or MOD name.

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

## Asset Types

| Type | Description |
|------|-------------|
| `vrm` | VRM 3D character models |
| `vrma` | VRMA animation files for VRM characters |
| `sound` | Audio files (sound effects, BGM) |
| `image` | Image files (PNG, JPG, etc.) |
| `html` | HTML files for WebView content |

## Types

### AssetType

```typescript
type AssetType = "vrm" | "vrma" | "sound" | "image" | "html";
```

### AssetInfo

```typescript
interface AssetInfo {
  /** Globally unique asset ID (e.g., "elmer:character"). */
  id: string;
  /** The asset type. */
  type: AssetType;
  /** The MOD that provides this asset. */
  mod: string;
  /** Optional description of the asset. */
  description?: string;
}
```

### AssetFilter

```typescript
interface AssetFilter {
  /** Filter by asset type. */
  type?: AssetType;
  /** Filter by MOD name. */
  mod?: string;
}
```

## Examples

### Find all available characters

```typescript
const characters = await assets.list({ type: "vrm" });
for (const char of characters) {
  console.log(`${char.id} from ${char.mod}`);
}
```

### Check if an animation exists

```typescript
const animations = await assets.list({ type: "vrma" });
const hasIdle = animations.some((a) => a.id === "vrma:idle-maid");
```

## Next Steps

- **[Mods API](./mods-api)** -- List installed MODs and execute bin commands
- **[SDK Overview](./)** -- Full module map and quick example
