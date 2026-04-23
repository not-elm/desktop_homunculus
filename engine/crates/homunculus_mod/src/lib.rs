//! # Homunculus Mod
//!
//! This crate provides mod system functionality for the Desktop Homunculus application,
//! enabling users to install and manage mods as NPM packages.
//!
//! ## Overview
//!
//! `homunculus_mod` implements a mod system based on NPM package conventions:
//! - Mods are installed via `npm add` into `$MODS_ROOT/node_modules/`
//! - Discovery reads `$MODS_ROOT/package.json` dependencies
//! - Services (`main`) run as long-running Node.js child processes
//! - MOD commands (`bin`) are executed via HTTP API
//!
//! ## Mod Structure
//!
//! ```text
//! $MODS_ROOT/
//! ├── package.json          # Root manifest with dependencies
//! ├── package-lock.json
//! └── node_modules/
//!     └── my-mod/
//!         ├── package.json  # Must include "homunculus" field
//!         ├── index.js      # Entry point (main)
//!         └── assets/       # Additional assets
//! ```

mod load;
pub mod managed_process;
mod mod_asset_reader;
pub mod mod_service;
pub mod node_process;

use crate::load::ModLoadPlugin;
use crate::mod_asset_reader::ModAssetReader;
use crate::mod_service::ModServicePlugin;
use crate::node_process::NodeProcessPlugin;
use bevy::asset::io::{AssetSourceBuilder, AssetSourceId};
use bevy::prelude::*;
use homunculus_core::prelude::{AssetRegistry, HomunculusConfig, ModRegistry};
use homunculus_utils::path::homunculus_dir;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct HomunculusModPlugin;

impl Plugin for HomunculusModPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world().resource::<HomunculusConfig>();
        let node_modules = config.mods_dir.join("node_modules");
        let dir_map = Arc::new(RwLock::new(HashMap::new()));
        register_local_assets_dir(&dir_map);
        let dir_map_for_reader = dir_map.clone();

        app.register_asset_source(
            AssetSourceId::from("asset"),
            AssetSourceBuilder::new(move || {
                Box::new(ModAssetReader::new(
                    node_modules.clone(),
                    dir_map_for_reader.clone(),
                ))
            }),
        );

        app.init_resource::<AssetRegistry>();
        app.init_resource::<ModRegistry>();
        app.add_plugins((NodeProcessPlugin, ModLoadPlugin, ModServicePlugin));
    }
}

/// Registers `~/.homunculus/assets/` in the dir_map under key `"local"`,
/// so `ModAssetReader` can resolve imported asset paths like `asset://local/filename.vrm`.
fn register_local_assets_dir(dir_map: &Arc<RwLock<HashMap<String, PathBuf>>>) {
    let assets_dir = homunculus_dir().join("assets");
    if let Ok(mut map) = dir_map.write() {
        map.insert("local".to_string(), assets_dir);
    }
}
