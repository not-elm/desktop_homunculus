use crate::prelude::{ModModuleSource, WebviewOpenOptions};
use bevy::asset::Asset;
use bevy::prelude::TypePath;
use bevy_tray_icon::menu::accelerator::{Code, Modifiers};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ModSettings {
    /// The name of the mod.
    pub name: String,
    /// The version of the mod.
    pub version: String,
    /// The description of the mod.
    pub description: Option<String>,
    /// The author of the mod.
    pub author: Option<String>,
    /// The license of the mod.
    pub license: Option<String>,
    /// The scripts that should be automatically run when the mod is loaded.
    #[serde(rename = "startupScripts")]
    pub startup_scripts: Option<Vec<ModModuleSource>>,
    #[serde(rename = "systemMenus")]
    pub system_menus: Option<Vec<ModSystemMenu>>,
    pub menus: Option<Vec<ModMenu>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModMenu {
    pub thumbnail: Option<ModModuleSource>,
    pub text: String,
    pub script: Option<ModModuleSource>,
    pub webview: Option<WebviewOpenOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ModSystemMenu {
    Common(ModSystemCommonMenu),
    Sub(ModSystemSubMenu),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModSystemSubMenu {
    pub title: String,
    pub menus: Vec<ModSystemMenu>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModSystemCommonMenu {
    pub text: String,
    pub shortcut: Option<SystemMenuShortcut>,
    pub webview: Option<WebviewOpenOptions>,
    pub script: Option<ModModuleSource>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SystemMenuShortcut {
    pub key: Code,
    pub modifiers: Option<Modifiers>,
}
