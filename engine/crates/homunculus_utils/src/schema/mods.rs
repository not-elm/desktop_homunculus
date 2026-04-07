use crate::prelude::AssetDeclaration;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Summary of a loaded mod, persisted after discovery.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Service script path (long-running process, auto-launched at startup).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<String>))]
    pub service_script_path: Option<PathBuf>,
    /// Available MOD command names (paths are not included).
    pub commands: Vec<String>,
    /// Assets registered by this mod.
    pub assets: HashMap<String, AssetDeclaration>,
    /// Menus registered by this mod.
    pub menus: Vec<ModMenu>,
    /// Tray menu item contributed by this mod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tray: Option<TrayMenuItem>,
    /// Absolute path to the mod's root directory on disk.
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub mod_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub(crate) struct ModPackageJson {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    /// Executable scripts keyed by command name, e.g. `{ "do-something": "./scripts/do-something.js" }`.
    #[serde(default)]
    pub bin: Option<HashMap<String, String>>,
    /// App-specific configuration under the "homunculus" key.
    pub homunculus: ModManifest,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ModManifest {
    /// Service script path (e.g., "index.ts"), relative to the mod root.
    #[serde(default)]
    pub service: Option<String>,
    #[serde(default)]
    pub menus: Option<Vec<ModMenu>>,
    /// Asset declarations keyed by asset ID.
    #[serde(default)]
    pub assets: Option<HashMap<String, AssetDeclaration>>,
    /// Tray menu item contributed by this mod.
    #[serde(default)]
    pub tray: Option<TrayMenuItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ModMenu {
    pub id: String,
    pub text: String,
    pub command: String,
}

/// A single tray menu entry contributed by a mod.
///
/// Has either `command` (leaf — clickable) or `items` (submenu), never both.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "openapi", schema(no_recursion))]
pub struct TrayMenuItem {
    pub id: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TrayMenuItem>>,
    /// Display position in the tray menu: `"top"`, `"middle"`, or `"bottom"`.
    /// Defaults to `"middle"` when omitted or invalid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tray_leaf_item() {
        let json = r#"{"id":"open-settings","text":"Settings","command":"open-ui"}"#;
        let item: TrayMenuItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "open-settings");
        assert_eq!(item.text, "Settings");
        assert_eq!(item.command, Some("open-ui".to_string()));
        assert!(item.items.is_none());
    }

    #[test]
    fn deserialize_tray_submenu() {
        let json =
            r#"{"id":"tools","text":"Tools","items":[{"id":"a","text":"A","command":"run-a"}]}"#;
        let item: TrayMenuItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "tools");
        assert!(item.command.is_none());
        let items = item.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].command, Some("run-a".to_string()));
    }

    #[test]
    fn deserialize_manifest_with_tray() {
        let json = r#"{"tray":{"id":"x","text":"X","command":"do-x"}}"#;
        let manifest: ModManifest = serde_json::from_str(json).unwrap();
        assert!(manifest.tray.is_some());
        assert_eq!(manifest.tray.unwrap().id, "x");
    }

    #[test]
    fn deserialize_manifest_without_tray() {
        let json = r#"{}"#;
        let manifest: ModManifest = serde_json::from_str(json).unwrap();
        assert!(manifest.tray.is_none());
    }

    #[test]
    fn deserialize_tray_item_with_position() {
        let json =
            r#"{"id":"open-settings","text":"Settings","command":"open-ui","position":"top"}"#;
        let item: TrayMenuItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.position, Some("top".to_string()));
    }

    #[test]
    fn deserialize_tray_item_without_position() {
        let json = r#"{"id":"x","text":"X","command":"do-x"}"#;
        let item: TrayMenuItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.position, None);
    }

    #[test]
    fn deserialize_tray_item_with_unknown_position() {
        let json = r#"{"id":"x","text":"X","command":"do-x","position":"invalid"}"#;
        let item: TrayMenuItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.position, Some("invalid".to_string()));
    }
}
