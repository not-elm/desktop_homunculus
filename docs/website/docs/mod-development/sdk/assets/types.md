---
sidebar_position: 100
---

# Type Definitions

### AssetType

```typescript
type AssetType = "vrm" | "vrma" | "sound" | "image" | "html";
```

| Value | Description |
|-------|-------------|
| `vrm` | VRM 3D character models |
| `vrma` | VRMA animation files for VRM characters |
| `sound` | Audio files (sound effects, BGM) |
| `image` | Image files (PNG, JPG, etc.) |
| `html` | HTML files for WebView content |

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
