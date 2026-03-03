# System Tray Feature Design

**Date:** 2026-03-03
**Status:** Approved

## Overview

Add system tray support to Desktop Homunculus using `bevy_tray_icon`. Each MOD can optionally contribute a single tray menu item via a new `homunculus.tray` field in its `package.json`. Clicking a tray item executes the corresponding bin command.

## Package.json Schema

New optional `tray` field under `homunculus` — a single object (not an array). If a mod needs multiple actions, it uses `items` to create a submenu.

**Leaf item (clickable):**

```json
{
  "homunculus": {
    "tray": {
      "id": "open-settings",
      "text": "Settings",
      "command": "open-ui"
    }
  }
}
```

**Submenu:**

```json
{
  "homunculus": {
    "tray": {
      "id": "tools",
      "text": "Tools",
      "items": [
        { "id": "tool-a", "text": "Tool A", "command": "run-tool-a" },
        { "id": "tool-b", "text": "Tool B", "command": "run-tool-b" }
      ]
    }
  }
}
```

**Rules:**

- `tray` is a single object or omitted entirely.
- Has either `command` (leaf) or `items` (submenu), never both.
- `items` entries can themselves nest `items` for deeper submenus.
- `id` is prefixed internally by mod name (`{mod_name}::{id}`) to avoid collisions.

## Rust Types

### Schema type (in `homunculus_utils::schema::mods`)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TrayMenuItem {
    pub id: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TrayMenuItem>>,
}
```

### ModManifest addition

```rust
pub struct ModManifest {
    pub service: Option<String>,
    pub menus: Option<Vec<ModMenu>>,
    pub assets: Option<HashMap<String, AssetDeclaration>>,
    pub tray: Option<TrayMenuItem>,  // NEW
}
```

## Plugin Architecture: `homunculus_tray`

### New crate: `engine/crates/homunculus_tray/`

**Dependencies:**

- `bevy_tray_icon` — `TrayIcon`, `MenuItem`, `MenuMessage`
- `homunculus_core` — `ModRegistry`, `HomunculusConfig`
- `homunculus_utils` — `TrayMenuItem` schema type

### Startup flow

1. **`load_tray_menus()`**: Read `tray` field from each mod in `ModRegistry`. Store in `TrayMenuRegistry` resource (maps prefixed IDs to mod name + command).
2. **`build_tray_icon()`**: Convert `TrayMenuRegistry` into a `bevy_tray_icon::TrayIcon` resource:
   - Each mod's item becomes a top-level `MenuItem`
   - Submenus (`items`) map to `MenuItem::SubMenu`
   - Leaf items (`command`) map to `MenuItem::Common`
   - `MenuItem::Separator` between mod entries
   - IDs prefixed: `"{mod_name}::{id}"`
3. Inserting `TrayIcon` triggers `bevy_tray_icon` to create the OS tray icon.

### Click handling (Update)

**`handle_tray_clicks()`**: Listen for `MenuMessage` events.

1. Parse mod name from the prefixed ID.
2. Look up the command from `TrayMenuRegistry`.
3. Fire-and-forget: `tokio::spawn(pnpm exec <command>)` in `mods_dir`.

No streaming needed — tray clicks just trigger mod commands that do their own work (open webviews, play sounds, etc.).

### Tray icon configuration

```rust
TrayIcon {
    icon: Some(asset_server.load("icon.png")),
    tooltip: Some("Desktop Homunculus".to_string()),
    menu: build_menu_from_registry(&tray_registry),
    show_menu_on_left_click: true,
}
```

### Menu assembly (example)

```
┌─────────────────────┐
│ Settings            │  ← @hmcs/settings (leaf)
│─────────────────────│
│ Character Settings  │  ← @hmcs/character-settings (leaf)
│─────────────────────│
│ Tools             ▶ │  ← hypothetical mod with submenu
│  ├─ Tool A          │
│  └─ Tool B          │
└─────────────────────┘
```

Ordering follows mod discovery order (from `pnpm ls`).

Static only — menu items are declared at startup and do not change at runtime.

### Registration in main.rs

Added after `HomunculusModPlugin` (requires `ModRegistry` to be populated):

```rust
app.add_plugins(HomunculusTrayPlugin);
```

## Mod Changes

### Rename `@hmcs/settings` → `@hmcs/character-settings`

- Rename directory: `mods/settings/` → `mods/character-settings/`
- Update `package.json` name to `@hmcs/character-settings`
- Update all references: pnpm workspace, imports, asset IDs (`settings:ui` → `character-settings:ui`)
- Existing `homunculus.menus` entry stays (powers the right-click context menu)

### New `@hmcs/settings` MOD (tray-driven)

New mod at `mods/settings/` providing the "Settings" tray item:

```json
{
  "name": "@hmcs/settings",
  "version": "1.0.0",
  "homunculus": {
    "tray": {
      "id": "open-settings",
      "text": "Settings",
      "command": "open-ui"
    },
    "assets": {
      "settings:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "Application settings UI"
      }
    }
  },
  "bin": {
    "open-ui": "commands/open-ui.ts"
  }
}
```

Settings webview includes:

- **Frame rate** control (target FPS)
- **Shadow-panel alpha** control (transparency slider)

Persisted via the existing prefs system (`~/.homunculus/prefs.db`).
