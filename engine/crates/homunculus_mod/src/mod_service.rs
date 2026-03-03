use crate::node_process::{NodeAvailable, NodeProcessHandle};
use bevy::prelude::*;
use std::path::PathBuf;
use std::process::Command;

/// A MOD service identified by its absolute filesystem path.
///
/// Services are long-running Node.js child processes that run for the
/// entire app session, declared via the `homunculus.service` field in a MOD's `package.json`.
#[derive(Component)]
pub(crate) struct ModService(pub PathBuf);

pub(crate) struct ModServicePlugin;

impl Plugin for ModServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            run_mod_services.run_if(resource_exists::<NodeAvailable>),
        );
    }
}

fn run_mod_services(mut commands: Commands, services: Query<(Entity, &ModService)>) {
    for (entity, service) in services.iter() {
        info!("Starting mod service: {}", service.0.display());
        match Command::new("node")
            .arg("--experimental-strip-types")
            .arg(&service.0)
            .current_dir(service.0.parent().unwrap_or(&service.0))
            .spawn()
        {
            Ok(child) => {
                commands.spawn(NodeProcessHandle(child));
            }
            Err(e) => {
                error!("Failed to start mod service {}: {}", service.0.display(), e);
            }
        }
        commands.entity(entity).despawn();
    }
}
