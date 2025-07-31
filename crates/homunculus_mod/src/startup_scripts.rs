use bevy::prelude::*;
use homunculus_core::prelude::ModModuleSource;
use homunculus_deno::prelude::DenoScriptHandle;

#[derive(Component)]
pub(crate) struct StartupScripts(pub Vec<ModModuleSource>);

pub(crate) struct StartupScriptsPlugin;

impl Plugin for StartupScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_loaded_scripts);
    }
}

fn run_loaded_scripts(
    mut commands: Commands,
    mut scripts: Query<(Entity, &mut StartupScripts)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut scripts) in scripts.iter_mut() {
        for source in std::mem::take(&mut scripts.0) {
            let path = source.to_string();
            commands.spawn(DenoScriptHandle(asset_server.load(path)));
        }
        commands.entity(entity).despawn();
    }
}
