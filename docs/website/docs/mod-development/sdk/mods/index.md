---
sidebar_position: 1
---

# mods

Discover installed MODs, execute bin commands with buffered or streaming output, and query registered context menu entries.

## Import

```typescript
import { mods } from "@hmcs/sdk";
```

:::warning Field Name Convention
`mods.get()` returns `ModInfo` with **snake_case** field names (`has_main`, `bin_commands`, `asset_ids`). This differs from the application info endpoint which returns camelCase field names. The field names are planned to be unified in a future release.
:::

## Functions

| Function | Description |
|----------|-------------|
| [list](./list) | Return metadata for every MOD discovered at startup |
| [get](./get) | Retrieve a single MOD by name |
| [executeCommand](./executeCommand) | Run a bin command and collect the buffered result |
| [streamCommand](./streamCommand) | Run a bin command and stream real-time output events |
| [menus](./menus) | Return all context menu entries registered across installed MODs |
