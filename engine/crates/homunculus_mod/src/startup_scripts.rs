use crate::node_process::{NodeAvailable, NodeProcessHandle};
use bevy::prelude::*;
use std::path::PathBuf;
use std::process::Command;

/// A single startup script identified by its absolute filesystem path.
#[derive(Component)]
pub(crate) struct StartupScript(pub PathBuf);

pub(crate) struct StartupScriptsPlugin;

impl Plugin for StartupScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            run_startup_scripts.run_if(resource_exists::<NodeAvailable>),
        );
    }
}

fn run_startup_scripts(mut commands: Commands, scripts: Query<(Entity, &StartupScript)>) {
    for (entity, script) in scripts.iter() {
        info!("Starting mod script: {}", script.0.display());
        match Command::new("node")
            .arg("--experimental-strip-types")
            .arg(&script.0)
            .current_dir(script.0.parent().unwrap_or(&script.0))
            .spawn()
        {
            Ok(child) => {
                commands.spawn(NodeProcessHandle(child));
            }
            Err(e) => {
                error!("Failed to start mod script {}: {}", script.0.display(), e);
            }
        }
        commands.entity(entity).despawn();
    }
}
