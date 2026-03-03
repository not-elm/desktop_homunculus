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
//! - On-demand scripts (`bin`) are executed via HTTP API
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
mod mod_asset_reader;
mod node_process;
mod mod_service;

use crate::load::ModLoadPlugin;
use crate::mod_asset_reader::ModAssetReader;
use crate::node_process::NodeProcessPlugin;
use crate::mod_service::ModServicePlugin;
use bevy::asset::io::{AssetSourceBuilder, AssetSourceId};
use bevy::prelude::*;
use homunculus_core::prelude::{AssetRegistry, HomunculusConfig, ModRegistry};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct HomunculusModPlugin;

impl Plugin for HomunculusModPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world().resource::<HomunculusConfig>();
        let node_modules = config.mods_dir.join("node_modules");
        let dir_map = Arc::new(RwLock::new(HashMap::new()));
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
