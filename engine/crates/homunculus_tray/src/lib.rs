//! # Homunculus Tray
//!
//! System tray integration for Desktop Homunculus. Builds a tray icon with
//! per-mod menu items sourced from `ModRegistry` and dispatches clicks to
//! mod commands via `pnpm exec`.

use std::collections::HashMap;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_tray_icon::plugin::TrayIconPlugin;
use bevy_tray_icon::plugin::menu_event::MenuMessage;
use bevy_tray_icon::resource::{Menu, MenuItem, TrayIcon};
use homunculus_core::prelude::{HomunculusConfig, ModRegistry, TrayMenuItem};
use tracing::{error, info};

/// Maps prefixed menu IDs (`"{mod_name}::{item_id}"`) to the mod name and
/// command string that should be executed when the item is clicked.
#[derive(Resource, Debug, Default)]
pub struct TrayMenuRegistry {
    entries: HashMap<String, (String, String)>,
}

impl TrayMenuRegistry {
    /// Register all leaf items from a `TrayMenuItem` tree.
    ///
    /// Only items with a `command` are registered. Submenu containers are
    /// traversed recursively but not registered themselves.
    pub fn register_item(&mut self, mod_name: &str, item: &TrayMenuItem) {
        if let Some(ref command) = item.command {
            let prefixed_id = format!("{mod_name}::{}", item.id);
            self.entries
                .insert(prefixed_id, (mod_name.to_string(), command.clone()));
        }
        if let Some(ref children) = item.items {
            for child in children {
                self.register_item(mod_name, child);
            }
        }
    }

    /// Look up the mod name and command for a prefixed menu ID.
    #[inline]
    pub fn lookup(&self, prefixed_id: &str) -> Option<&(String, String)> {
        self.entries.get(prefixed_id)
    }
}

/// Plugin that provides system tray integration for Desktop Homunculus.
///
/// Adds the `TrayIconPlugin`, builds a tray menu from mod declarations,
/// and dispatches menu clicks to mod commands.
pub struct HomunculusTrayPlugin;

impl Plugin for HomunculusTrayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrayIconPlugin)
            .add_systems(Startup, setup_tray)
            .add_systems(Update, handle_tray_clicks);
    }
}

/// Startup system that builds the tray icon with menu items from all mods.
fn setup_tray(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mod_registry: Res<ModRegistry>,
) {
    let (menu, registry) = build_menu(&mod_registry);
    commands.insert_resource(registry);
    commands.insert_resource(TrayIcon {
        icon: Some(asset_server.load("icons/icon.png")),
        tooltip: Some("Desktop Homunculus".to_string()),
        menu,
        show_menu_on_left_click: true,
    });
}

/// Update system that handles tray menu click events.
///
/// Looks up the clicked menu ID in the registry and spawns a fire-and-forget
/// task to execute the associated mod command.
fn handle_tray_clicks(
    mut mr: MessageReader<MenuMessage>,
    registry: Option<Res<TrayMenuRegistry>>,
    config: Res<HomunculusConfig>,
) {
    let Some(registry) = registry else {
        return;
    };
    let mods_dir = config.mods_dir.clone();
    for message in mr.read() {
        let id = &message.id.0;
        if let Some((_mod_name, command)) = registry.lookup(id) {
            let mods_dir = mods_dir.clone();
            let command = command.clone();
            info!("Tray menu clicked: id={id}, command={command}");
            IoTaskPool::get()
                .spawn(async move {
                    execute_command(mods_dir, command).await;
                })
                .detach();
            // tokio::runtime::Builder::new_current_thread()
            //     .build()
            //     .unwrap()
            //     .spawn(async move {
            //         execute_command(mods_dir, command).await;
            //     });
        } else {
            tracing::warn!("Unknown tray menu id: {id}");
        }
    }
}

/// Execute a mod command via `pnpm exec` in the mods directory.
///
/// Stdout is discarded; stderr is piped and logged on failure.
async fn execute_command(mods_dir: PathBuf, command: String) {
    use homunculus_utils::process::CommandNoWindow;
    let result = std::process::Command::new(homunculus_utils::mods::pnpm_program())
        .no_window()
        .arg("exec")
        .arg(&command)
        .current_dir(&mods_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();

    match result {
        Ok(output) if !output.status.success() => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!(
                "Tray command `{command}` failed (status {}): {stderr}",
                output.status
            );
        }
        Err(e) => {
            error!("Failed to spawn tray command `{command}`: {e}");
        }
        _ => {}
    }
}

/// Build the tray menu and registry from all mods in the `ModRegistry`.
///
/// Each mod that declares a `tray` field contributes one top-level item
/// (or submenu). Mods are separated by `MenuItem::Separator`.
fn build_menu(mod_registry: &ModRegistry) -> (Menu, TrayMenuRegistry) {
    let mut menu_items: Vec<MenuItem> = Vec::new();
    let mut registry = TrayMenuRegistry::default();
    let mut first = true;
    for mod_info in mod_registry.all() {
        let Some(ref tray_item) = mod_info.tray else {
            continue;
        };

        if !first {
            menu_items.push(MenuItem::separator());
        }
        first = false;

        menu_items.push(to_bevy_menu_item(&mod_info.name, tray_item));
        registry.register_item(&mod_info.name, tray_item);
    }

    (Menu::new(menu_items), registry)
}

/// Convert a `TrayMenuItem` to a `bevy_tray_icon` `MenuItem`.
///
/// Leaf items (those with `command`) become `MenuItem::Common`.
/// Items with `items` become `MenuItem::SubMenu`.
/// IDs are prefixed with `"{mod_name}::"`.
fn to_bevy_menu_item(mod_name: &str, item: &TrayMenuItem) -> MenuItem {
    let prefixed_id = format!("{mod_name}::{}", item.id);
    if let Some(ref children) = item.items {
        let child_items: Vec<MenuItem> = children
            .iter()
            .map(|child| to_bevy_menu_item(mod_name, child))
            .collect();
        MenuItem::submenu(prefixed_id, &item.text, true, Menu::new(child_items))
    } else {
        MenuItem::common(prefixed_id, &item.text, true, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use homunculus_utils::prelude::TrayMenuItem;

    #[test]
    fn registry_registers_leaf_item() {
        let mut registry = TrayMenuRegistry::default();
        let item = TrayMenuItem {
            id: "open-settings".to_string(),
            text: "Settings".to_string(),
            command: Some("open-ui".to_string()),
            items: None,
        };
        registry.register_item("my-mod", &item);

        let result = registry.lookup("my-mod::open-settings");
        assert!(result.is_some());
        let (mod_name, command) = result.expect("should find registered leaf item");
        assert_eq!(mod_name, "my-mod");
        assert_eq!(command, "open-ui");
    }

    #[test]
    fn registry_registers_submenu_items_recursively() {
        let mut registry = TrayMenuRegistry::default();
        let item = TrayMenuItem {
            id: "tools".to_string(),
            text: "Tools".to_string(),
            command: None,
            items: Some(vec![
                TrayMenuItem {
                    id: "tool-a".to_string(),
                    text: "Tool A".to_string(),
                    command: Some("run-a".to_string()),
                    items: None,
                },
                TrayMenuItem {
                    id: "tool-b".to_string(),
                    text: "Tool B".to_string(),
                    command: Some("run-b".to_string()),
                    items: None,
                },
            ]),
        };
        registry.register_item("my-mod", &item);

        // Container should NOT be registered (no command).
        assert!(registry.lookup("my-mod::tools").is_none());

        // Children should be registered.
        let a = registry.lookup("my-mod::tool-a");
        assert!(a.is_some());
        assert_eq!(a.expect("tool-a should be registered").1, "run-a");

        let b = registry.lookup("my-mod::tool-b");
        assert!(b.is_some());
        assert_eq!(b.expect("tool-b should be registered").1, "run-b");
    }

    #[test]
    fn registry_lookup_returns_none_for_unknown_id() {
        let registry = TrayMenuRegistry::default();
        assert!(registry.lookup("nonexistent::id").is_none());
    }

    #[test]
    fn to_bevy_menu_item_leaf() {
        let item = TrayMenuItem {
            id: "open".to_string(),
            text: "Open".to_string(),
            command: Some("do-open".to_string()),
            items: None,
        };
        let bevy_item = to_bevy_menu_item("test-mod", &item);
        match bevy_item {
            MenuItem::Common {
                id, text, enabled, ..
            } => {
                assert_eq!(id.0, "test-mod::open");
                assert_eq!(text, "Open");
                assert!(enabled);
            }
            other => panic!("expected Common, got {other:?}"),
        }
    }

    #[test]
    fn to_bevy_menu_item_submenu() {
        let item = TrayMenuItem {
            id: "parent".to_string(),
            text: "Parent".to_string(),
            command: None,
            items: Some(vec![TrayMenuItem {
                id: "child".to_string(),
                text: "Child".to_string(),
                command: Some("run-child".to_string()),
                items: None,
            }]),
        };
        let bevy_item = to_bevy_menu_item("test-mod", &item);
        match bevy_item {
            MenuItem::SubMenu {
                id,
                text,
                enabled,
                menu,
            } => {
                assert_eq!(id.0, "test-mod::parent");
                assert_eq!(text, "Parent");
                assert!(enabled);
                assert_eq!(menu.len(), 1);
                match &menu[0] {
                    MenuItem::Common { id, text, .. } => {
                        assert_eq!(id.0, "test-mod::child");
                        assert_eq!(text, "Child");
                    }
                    other => panic!("expected Common child, got {other:?}"),
                }
            }
            other => panic!("expected SubMenu, got {other:?}"),
        }
    }
}
