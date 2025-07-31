use crate::startup_scripts::StartupScripts;
use crate::system_menu::RequestUpdateSystemMenus;
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use homunculus_core::prelude::{
    Loading, ModMenuMetadata, ModMenuMetadataList, ModModuleSource, ModModuleSpecifier,
    ModSettings, mod_dir, remove_mystery_file_if_exists,
};
use std::path::PathBuf;

pub(crate) struct ModLoadPlugin;

impl Plugin for ModLoadPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ModFolderHandle>()
            .register_type::<ModDirPath>()
            .register_type::<ModSettingsHandle>()
            .add_systems(Startup, start_load_mod_folder)
            .add_systems(Update, (start_mod_settings_json, read_mod_settings));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ModSettingsHandle(Handle<ModSettings>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
struct ModDirPath(PathBuf);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ModFolderHandle(Handle<LoadedFolder>);

fn start_load_mod_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    remove_mystery_file_if_exists(&mod_dir());
    remove_mystery_file_if_exists(&PathBuf::from("assets").join("mods"));
    commands.spawn((Loading, ModFolderHandle(asset_server.load_folder("mods"))));
}

fn start_mod_settings_json(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    folders: Res<Assets<LoadedFolder>>,
    handle: Query<(Entity, &ModFolderHandle), With<Loading>>,
) {
    for (folder_entity, folder_handle) in handle.iter() {
        let Some(folder) = folders.get(folder_handle.0.id()) else {
            continue;
        };
        for handle in folder.handles.iter() {
            if let Some(asset_path) = handle.path()
                && let Some(path) = asset_path.path().to_str()
                && path.ends_with("mod.json")
            {
                commands.spawn((
                    Loading,
                    Name::new(path.to_string()),
                    ModSettingsHandle(asset_server.load(asset_path)),
                ));
            }
        }
        commands.entity(folder_entity).remove::<Loading>();
    }
}

fn read_mod_settings(
    mut commands: Commands,
    mut ew_menu: EventWriter<RequestUpdateSystemMenus>,
    mut menus: ResMut<ModMenuMetadataList>,
    settings: Res<Assets<ModSettings>>,
    handles: Query<(Entity, &ModSettingsHandle)>,
) {
    for (entity, handle) in handles.iter() {
        let Some(settings) = settings.get(handle.0.id()) else {
            continue;
        };
        if let Some(scripts) = settings.startup_scripts.as_ref() {
            commands.spawn(StartupScripts(scripts.clone()));
        }
        if let Some(system_menus) = settings.system_menus.as_ref() {
            ew_menu.write(RequestUpdateSystemMenus {
                mod_name: settings.name.clone(),
                menus: system_menus.clone(),
            });
        }
        if let Some(settings_menus) = settings.menus.as_ref() {
            menus.extend(settings_menus.iter().map(|m| ModMenuMetadata {
                thumbnail: thumbnail_source(&m.thumbnail),
                text: m.text.clone(),
                script: m.script.as_ref().map(|s| PathBuf::from(&s.0)),
                webview: m.webview.clone(),
            }));
        }
        info!("Loaded mod: [{}]", settings.name);
        commands.entity(entity).despawn();
    }
}

fn thumbnail_source(thumbnail: &Option<ModModuleSource>) -> Option<String> {
    thumbnail
        .as_ref()
        .map(|source| match source.to_specifier() {
            ModModuleSpecifier::Remote(url) => url.to_string(),
            ModModuleSpecifier::Local(path) => {
                // Convert to a relative path from `mods/menu/index.html`.
                PathBuf::from("/").join(path).to_string_lossy().to_string()
            }
        })
}
