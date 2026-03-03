---
title: "Preferences"
sidebar_position: 11
---

# Preferences

Persistent key-value storage with JSON serialization, backed by SQLite at `~/.homunculus/prefs.db`. Use preferences to save MOD settings, character state, or any data that should survive restarts.

## Import

```typescript
import { preferences } from "@hmcs/sdk";
```

## Save

`preferences.save(key, value)` stores any JSON-serializable value under the given key. Overwrites the previous value if the key already exists.

```typescript
await preferences.save("my-mod:theme", "dark");

await preferences.save("my-mod:settings", {
  volume: 0.8,
  notifications: true,
});
```

## Load

`preferences.load<V>(key)` retrieves the value for a key. Returns `undefined` if the key does not exist.

```typescript
const theme = await preferences.load<string>("my-mod:theme");
if (theme !== undefined) {
  console.log(`Theme: ${theme}`);
}

interface Settings {
  volume: number;
  notifications: boolean;
}
const settings = await preferences.load<Settings>("my-mod:settings");
```

## List

`preferences.list()` returns all stored key names.

```typescript
const keys = await preferences.list();
console.log(`${keys.length} preferences stored`);

for (const key of keys) {
  const value = await preferences.load(key);
  console.log(`${key}:`, value);
}
```

## Delete

The SDK does not currently support deleting individual preference keys. To delete a key, use the CLI:

```shell
hmcs prefs delete <key>
```

## Example: Saving Character Position

A common pattern is saving a character's transform so it restores on next launch.

```typescript
import { Vrm, preferences } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// Save position on state change
vrm.events().on("state-change", async () => {
  const transform = await vrm.transform();
  await preferences.save("my-mod:vrm-transform", transform);
});

// Restore on startup
const saved = await preferences.load("my-mod:vrm-transform");
if (saved) {
  const character = await Vrm.spawn("my-mod:avatar", { transform: saved });
}
```

:::note Key naming
Use a `"mod-name:key"` prefix to avoid collisions with other MODs. For example: `"my-mod:theme"`, `"my-mod:settings"`.
:::

## Next Steps

- **[SDK Overview](./)** -- Full module map and installation
