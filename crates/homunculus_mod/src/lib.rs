//! # Homunculus Mod
//!
//! This crate provides mod system functionality for the Desktop Homunculus application,
//! enabling users to create custom extensions, scripts, and menu items to extend
//! the mascot's behavior.
//!
//! ## Overview
//!
//! `homunculus_mod` implements a comprehensive mod system that allows users to:
//! - Load custom JavaScript/HTML mods from the filesystem
//! - Add custom menu items to the system menu
//! - Execute startup scripts when mods are loaded
//! - Define mod metadata and configuration through JSON files
//!
//! ## Key Features
//!
//! - **JSON Configuration**: Mods are configured through `mod.json` files
//! - **Startup Scripts**: Automatic execution of scripts when mods load
//! - **System Menu Integration**: Mods can add custom menu items
//! - **Asset Integration**: Full integration with Bevy's asset system
//! - **Hot Reloading**: Mods can be reloaded during development
//!
//! ## Mod Structure
//!
//! Mods are organized in directories under `./assets/mods/` with the following structure:
//!
//! ```text
//! assets/mods/my_mod/
//! ├── mod.json          # Mod configuration and metadata
//! ├── index.html        # Main HTML content (optional)
//! ├── script.js         # JavaScript code (optional)
//! └── assets/           # Additional assets (optional)
//! ```
//!
//! ## Mod Configuration
//!
//! The `mod.json` file defines mod metadata, menu items, and startup behavior:
//!
//! ```json
//! {
//!   "name": "My Custom Mod",
//!   "version": "1.0.0",
//!   "description": "A custom mod for my mascot",
//!   "menu_items": [...],
//!   "startup_scripts": [...]
//! }
//! ```

mod load;
mod startup_scripts;
mod system_menu;

use crate::load::ModLoadPlugin;
use crate::startup_scripts::StartupScriptsPlugin;
use crate::system_menu::ModSystemMenuPlugin;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use homunculus_core::prelude::ModSettings;

/// Plugin that provides mod system functionality for extending Homunculus with user-created content.
///
/// This plugin enables a comprehensive mod system that allows users to create custom
/// extensions using JavaScript, HTML, and JSON configuration files. Mods can add
/// menu items, execute startup scripts, and provide custom functionality.
///
/// # Included Components
///
/// - `JsonAssetPlugin<ModSettings>`: Handles loading and parsing of mod.json files
/// - `ModLoadPlugin`: Manages mod discovery and loading from the filesystem
/// - `StartupScriptsPlugin`: Executes mod startup scripts when mods are loaded
/// - `ModSystemMenuPlugin`: Integrates mod menu items into the system menu
///
/// # Mod Development
///
/// Mods are created by placing directories in `./assets/mods/` with:
/// 1. A `mod.json` configuration file defining metadata and behavior
/// 2. Optional HTML, JavaScript, and asset files
/// 3. Menu item definitions for system integration
/// 4. Startup scripts for initialization logic
///
/// # Asset Integration
///
/// The plugin integrates with Bevy's asset system, enabling:
/// - Hot reloading of mod files during development
/// - Proper asset lifecycle management
/// - Integration with the broader Homunculus asset pipeline
pub struct HomunculusModPlugin;

impl Plugin for HomunculusModPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            JsonAssetPlugin::<ModSettings>::new(&["mod.json"]),
            ModLoadPlugin,
            StartupScriptsPlugin,
            ModSystemMenuPlugin,
        ));
    }
}
