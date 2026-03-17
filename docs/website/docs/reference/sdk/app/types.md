---
sidebar_position: 100
---

# Type Definitions

### AppInfo

```typescript
interface AppInfo {
  /** Engine version string (e.g., "0.1.0-alpha.3.2"). */
  version: string;
  /** Platform information. */
  platform: PlatformInfo;
  /** Engine-level features available in this build. */
  features: string[];
  /** All loaded MODs with metadata. */
  mods: InfoMod[];
}
```

### PlatformInfo

```typescript
interface PlatformInfo {
  /** Operating system (e.g., "macos", "windows", "linux"). */
  os: string;
  /** CPU architecture (e.g., "aarch64", "x86_64"). */
  arch: string;
}
```

### InfoMod

```typescript
interface InfoMod {
  /** MOD package name. */
  name: string;
  /** MOD package version. */
  version: string;
  /** Human-readable description. */
  description: string | null;
  /** Author. */
  author: string | null;
  /** License identifier. */
  license: string | null;
  /** Whether the MOD has a running main process. */
  hasMain: boolean;
  /** Available bin command names. */
  binCommands: string[];
  /** Registered asset IDs. */
  assetIds: string[];
}
```
