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
use homunculus_utils::runtime::RuntimeResolver;
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
    runtime: Res<RuntimeResolver>,
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
            let runtime = runtime.clone();
            info!("Tray menu clicked: id={id}, command={command}");
            IoTaskPool::get()
                .spawn(async move {
                    execute_command(mods_dir, command, &runtime).await;
                })
                .detach();
        } else {
            tracing::warn!("Unknown tray menu id: {id}");
        }
    }
}

/// Execute a mod command via `pnpm exec` in the mods directory.
///
/// Stdout is discarded; stderr is piped and logged on failure.
async fn execute_command(mods_dir: PathBuf, command: String, runtime: &RuntimeResolver) {
    use homunculus_utils::process::CommandNoWindow;
    let (program, pnpm_args) = runtime.pnpm_program_and_args();
    let mut cmd = std::process::Command::new(program);
    cmd.no_window()
        .args(&pnpm_args)
        .arg("exec")
        .arg(&command)
        .current_dir(&mods_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    #[cfg(windows)]
    if !runtime.is_bundled()
        && let Some(path) = homunculus_utils::process::path_with_node_prepended()
    {
        cmd.env("PATH", path);
    }
    let result = cmd.output();

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

/// Resolve the tray position string to a sort-order group index.
///
/// Returns 0 for `"top"`, 1 for `"middle"` (default), 2 for `"bottom"`.
/// Unknown values are treated as `"middle"` with a warning log.
fn resolve_position(position: Option<&str>, mod_name: &str) -> u8 {
    match position {
        Some("top") => 0,
        Some("bottom") => 2,
        Some("middle") | None => 1,
        Some(unknown) => {
            tracing::warn!(
                "MOD {mod_name}: unknown tray position \"{unknown}\", falling back to \"middle\""
            );
            1
        }
    }
}

/// Build the tray menu and registry from all mods in the `ModRegistry`.
///
/// Items are grouped by position (`top` → `middle` → `bottom`) and sorted
/// alphabetically by mod name within each group. Groups are separated by
/// `MenuItem::Separator`.
fn build_menu(mod_registry: &ModRegistry) -> (Menu, TrayMenuRegistry) {
    let mut registry = TrayMenuRegistry::default();

    let mut entries = collect_tray_entries(mod_registry);
    entries.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let menu_items = build_grouped_menu_items(&entries, &mut registry);
    (Menu::new(menu_items), registry)
}

/// Collect (position_key, mod_name, tray_item) tuples from all mods.
fn collect_tray_entries(mod_registry: &ModRegistry) -> Vec<(u8, String, TrayMenuItem)> {
    mod_registry
        .all()
        .iter()
        .filter_map(|mod_info| {
            let tray_item = mod_info.tray.as_ref()?;
            let pos = resolve_position(tray_item.position.as_deref(), &mod_info.name);
            Some((pos, mod_info.name.clone(), tray_item.clone()))
        })
        .collect()
}

/// Build the flat `Vec<MenuItem>` with separators between position groups.
fn build_grouped_menu_items(
    entries: &[(u8, String, TrayMenuItem)],
    registry: &mut TrayMenuRegistry,
) -> Vec<MenuItem> {
    let mut menu_items: Vec<MenuItem> = Vec::new();
    let mut last_group: Option<u8> = None;

    for (group, mod_name, tray_item) in entries {
        if let Some(prev) = last_group
            && *group != prev
        {
            menu_items.push(MenuItem::separator());
        }
        last_group = Some(*group);

        menu_items.push(to_bevy_menu_item(mod_name, tray_item));
        registry.register_item(mod_name, tray_item);
    }

    menu_items
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
            position: None,
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
                    position: None,
                },
                TrayMenuItem {
                    id: "tool-b".to_string(),
                    text: "Tool B".to_string(),
                    command: Some("run-b".to_string()),
                    items: None,
                    position: None,
                },
            ]),
            position: None,
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
            position: None,
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
                position: None,
            }]),
            position: None,
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

    #[test]
    fn build_menu_orders_by_position_then_mod_name() {
        use homunculus_core::prelude::ModInfo;
        use std::collections::HashMap;
        use std::path::PathBuf;

        let mut mod_registry = ModRegistry::default();

        let mods = vec![
            ("@hmcs/voicevox", Some("middle")),
            ("@hmcs/app-exit", Some("bottom")),
            ("@hmcs/settings", Some("top")),
        ];
        for (name, position) in mods {
            mod_registry.register(ModInfo {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                description: None,
                author: None,
                license: None,
                service_script_path: None,
                commands: vec![],
                assets: HashMap::new(),
                menus: vec![],
                tray: Some(TrayMenuItem {
                    id: format!("{name}-tray"),
                    text: name.to_string(),
                    command: Some(format!("{name}-cmd")),
                    items: None,
                    position: position.map(|s| s.to_string()),
                }),
                mod_dir: PathBuf::from("/tmp"),
            });
        }

        let (menu, _registry) = build_menu(&mod_registry);

        let texts: Vec<&str> = menu
            .iter()
            .filter_map(|item| match item {
                MenuItem::Common { text, .. } => Some(text.as_str()),
                MenuItem::SubMenu { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();

        assert_eq!(
            texts,
            vec![
                "@hmcs/settings",
                "@hmcs/voicevox",
                "@hmcs/app-exit"
            ]
        );

        let separator_count = menu
            .iter()
            .filter(|item| matches!(item, MenuItem::Separator))
            .count();
        assert_eq!(separator_count, 2);
    }

    #[test]
    fn build_menu_skips_separator_for_empty_groups() {
        use homunculus_core::prelude::ModInfo;
        use std::collections::HashMap;
        use std::path::PathBuf;

        let mut mod_registry = ModRegistry::default();

        for (name, position) in [("@hmcs/settings", "top"), ("@hmcs/app-exit", "bottom")] {
            mod_registry.register(ModInfo {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                description: None,
                author: None,
                license: None,
                service_script_path: None,
                commands: vec![],
                assets: HashMap::new(),
                menus: vec![],
                tray: Some(TrayMenuItem {
                    id: format!("{name}-tray"),
                    text: name.to_string(),
                    command: Some(format!("{name}-cmd")),
                    items: None,
                    position: Some(position.to_string()),
                }),
                mod_dir: PathBuf::from("/tmp"),
            });
        }

        let (menu, _) = build_menu(&mod_registry);

        let separator_count = menu
            .iter()
            .filter(|item| matches!(item, MenuItem::Separator))
            .count();
        assert_eq!(separator_count, 1);

        let texts: Vec<&str> = menu
            .iter()
            .filter_map(|item| match item {
                MenuItem::Common { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(texts, vec!["@hmcs/settings", "@hmcs/app-exit"]);
    }
}
