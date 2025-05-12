use bevy::app::{App, Plugin};
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy_mod_scripting::core::asset::ScriptAsset;
use bevy_mod_scripting::core::script::ScriptComponent;
use homunculus_core::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ScriptFolderHandle(Handle<LoadedFolder>);

/// Loads scripts from `assets/scripts/*.lua`.
pub struct LoadScriptsPlugin;

impl Plugin for LoadScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ScriptFolderHandle>()
            .add_systems(Startup, start_load_plugins_folder)
            .add_systems(Update, load_script_files);
    }
}

fn start_load_plugins_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    remove_mystery_file_if_exists(&plugins_dir());
    commands.spawn((
        Loading,
        ScriptFolderHandle(asset_server.load_folder("mods")),
    ));
}

fn load_script_files(
    mut commands: Commands,
    mut scripts: Local<Vec<Handle<ScriptAsset>>>,
    asset_server: Res<AssetServer>,
    folders: Res<Assets<LoadedFolder>>,
    handle: Query<(Entity, &ScriptFolderHandle)>,
) {
    for (folder_entity, folder_handle) in handle.iter() {
        let Some(folder) = folders.get(folder_handle.0.id()) else {
            continue;
        };
        for script_path in folder.handles.iter().flat_map(|handle| handle.path()) {
            if script_path.path().extension().is_none_or(|e| e != "lua") {
                continue;
            }
            let Some(script) = script_path.path().to_str() else {
                continue;
            };
            commands.spawn((
                Name::new(script.to_string()),
                ScriptComponent::new(vec![script.to_string()]),
            ));
            scripts.push(asset_server.load(script_path));
        }
        commands.entity(folder_entity).despawn();
    }
}
