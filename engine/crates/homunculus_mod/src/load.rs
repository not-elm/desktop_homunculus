use std::path::PathBuf;

use crate::startup_scripts::StartupScript;
use bevy::prelude::*;
use homunculus_core::prelude::{
    AssetEntry, AssetId, AssetRegistry, HomunculusConfig, ModInfo, ModMenuMetadata,
    ModMenuMetadataList, ModRegistry, create_dir_all_if_need,
};

pub(crate) struct ModLoadPlugin;

impl Plugin for ModLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, discover_mods);
    }
}

fn discover_mods(
    mut commands: Commands,
    mut menus: ResMut<ModMenuMetadataList>,
    mut registry: ResMut<AssetRegistry>,
    mut mod_registry: ResMut<ModRegistry>,
    config: Res<HomunculusConfig>,
) {
    let mods_root = config.mods_dir.clone();
    info!("Mods root: {}", mods_root.display());
    create_dir_all_if_need(&mods_root);
    let mods = match homunculus_utils::mods::list::list_installation_mods() {
        Ok(mods) => mods,
        Err(e) => {
            error!("{e}");
            return;
        }
    };
    for m in mods {
        schedule_entry_point(&m, &mut commands);
        load_assets(&m, &mut registry);
        load_menus(&m, &mut menus);
        info!("Loaded mod: [{}]", m.name);
        mod_registry.register(m);
    }
}

fn schedule_entry_point(info: &ModInfo, commands: &mut Commands) {
    if let Some(entry_point_path) = &info.entry_point_path {
        if entry_point_path.exists() {
            commands.spawn(StartupScript(entry_point_path.clone()));
        } else {
            warn!("Startup script not found: {}", entry_point_path.display());
        }
    }
}

fn load_assets(info: &ModInfo, registry: &mut ResMut<AssetRegistry>) {
    for (asset_id, decl) in &info.assets {
        registry.register(AssetEntry {
            id: AssetId::new(asset_id.clone()),
            path: PathBuf::from(&decl.path),
            absolute_path: info.mod_dir.join(&decl.path),
            asset_type: decl.asset_type.clone(),
            description: decl.description.clone(),
            mod_name: info.name.clone(),
        });
    }
}

fn load_menus(info: &ModInfo, menus: &mut ResMut<ModMenuMetadataList>) {
    menus.extend(info.menus.iter().map(|m| ModMenuMetadata {
        id: m.id.clone(),
        mod_name: info.name.clone(),
        text: m.text.clone(),
        command: m.command.clone(),
    }));
}
