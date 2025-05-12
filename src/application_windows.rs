pub mod hit_test;
mod setup;

use crate::application_windows::setup::ApplicationWindowsSetupPlugin;
use bevy::app::App;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_vrma::system_param::cameras::Cameras;

#[derive(Debug, Component, Eq, PartialEq, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct TargetMonitor(pub Entity);

#[derive(Debug, Component, Eq, PartialEq, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct PrimaryCamera;

pub struct ApplicationWindowsPlugin;

impl Plugin for ApplicationWindowsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<PrimaryCamera>()
            .add_plugins((
                ApplicationWindowsSetupPlugin,
                #[cfg(not(feature = "develop"))]
                hit_test::ApplicationWindowsHitTestPlugin,
            ))
            .add_systems(Update, setup_mesh_3d);
    }
}

fn setup_mesh_3d(
    par_commands: ParallelCommands,
    meshes: Query<Entity, Added<Mesh3d>>,
    cameras: Cameras,
) {
    meshes.par_iter().for_each(|entity| {
        let layers = cameras.all_layers();
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert((
                layers,
                //FIXME: When mascot moves to the top of the screen while sitting, the face is not drawn,
                // so it is inserted as a temporary measure
                NoFrustumCulling,
            ));
        });
    });
}
