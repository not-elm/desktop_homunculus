use std::path::{Path, PathBuf};

use crate::mod_service::ModService;
use bevy::prelude::*;
use homunculus_core::prelude::{
    AssetEntry, AssetId, AssetRegistry, HomunculusConfig, ModInfo, ModMenuMetadata,
    ModMenuMetadataList, ModRegistry, create_dir_all_if_need,
};

pub(crate) struct ModLoadPlugin;

impl Plugin for ModLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, discover_mods);
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
    if let Err(e) = homunculus_utils::mods::ensure_tsx() {
        warn!("Failed to install tsx in mods directory: {e}");
    }
    let mods = match homunculus_utils::mods::list::list_installation_mods() {
        Ok(mods) => mods,
        Err(e) => {
            error!("{e}");
            return;
        }
    };
    for m in mods {
        schedule_service(&m, &mut commands, &mods_root);
        load_assets(&m, &mut registry);
        load_menus(&m, &mut menus);
        info!("Loaded mod: [{}]", m.name);
        mod_registry.register(m);
    }
}

fn schedule_service(info: &ModInfo, commands: &mut Commands, mods_dir: &Path) {
    if let Some(service_script_path) = &info.service_script_path {
        if service_script_path.exists() {
            commands.spawn(ModService {
                script_path: service_script_path.clone(),
                mods_dir: mods_dir.to_path_buf(),
            });
        } else {
            warn!(
                "Service script not found: {}",
                service_script_path.display()
            );
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
