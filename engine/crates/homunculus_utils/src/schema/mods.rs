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
    /// Available bin command names (paths are not included).
    pub commands: Vec<String>,
    /// Assets registered by this mod.
    pub assets: HashMap<String, AssetDeclaration>,
    /// Menus registered by this mod.
    pub menus: Vec<ModMenu>,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ModMenu {
    pub id: String,
    pub text: String,
    pub command: String,
}
