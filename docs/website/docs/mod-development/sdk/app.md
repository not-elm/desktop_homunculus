---
title: "app"
sidebar_position: 15
---

# app

Application lifecycle, health checks, and platform information. Use `app` to verify the engine is running, query its version and features, or shut down the application.

## Import

```typescript
import { app } from "@hmcs/sdk";
```

## Health Check

Returns `true` if the Desktop Homunculus engine is reachable and healthy, `false` otherwise. Useful for services that need to wait for the engine before proceeding.

```typescript
const alive = await app.health();
if (!alive) {
  console.error("Homunculus engine is not running");
}
```

**Signature:**

```typescript
app.health(): Promise<boolean>
```

## App Info

Returns metadata about the running engine instance in a single request -- version string, platform details, compiled features, and all loaded MODs.

```typescript
const info = await app.info();
console.log(`Engine v${info.version} on ${info.platform.os}/${info.platform.arch}`);
console.log(`Features: ${info.features.join(", ")}`);
console.log(`${info.mods.length} MODs loaded`);

for (const mod of info.mods) {
  console.log(`  ${mod.name}@${mod.version} — ${mod.binCommands.length} commands`);
}
```

**Signature:**

```typescript
app.info(): Promise<AppInfo>
```

## Exit

Shuts down the Desktop Homunculus application gracefully.

```typescript
await app.exit();
```

:::warning
`app.exit()` terminates the entire application, including all running MODs. Use with care.
:::

## Types

### AppInfo

```typescript
interface AppInfo {
  /** Engine version string (e.g., "0.1.0-alpha.4"). */
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

## Next Steps

- **[SDK Overview](./)** -- Full module map and quick example.
