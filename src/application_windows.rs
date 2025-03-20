pub mod hit_test;
mod setup;

use crate::application_windows::hit_test::ApplicationWindowsHitTestPlugin;
use crate::application_windows::setup::ApplicationWindowsSetupPlugin;
use crate::error::OutputLog;
use bevy::app::App;
use bevy::ecs::system::RunSystemOnce;
use bevy::ecs::world::DeferredWorld;
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
        app.register_type::<PrimaryCamera>().add_plugins((
            ApplicationWindowsSetupPlugin,
            #[cfg(not(feature = "develop"))]
            ApplicationWindowsHitTestPlugin,
        ));

        app.world_mut().register_component_hooks::<Mesh3d>().on_add(
            |mut world: DeferredWorld, entity: Entity, _| {
                world.commands().queue(move |world: &mut World| {
                    world
                        .run_system_once_with(entity, setup_render_layers)
                        .output_log_if_error("[Mesh3d]");
                });

                world.commands().entity(entity).insert((
                    //FIXME: When mascot moves to the top of the screen while sitting, the face is not drawn,
                    // so it is inserted as a temporary measure
                    NoFrustumCulling,
                ));
            },
        );
    }
}

fn setup_render_layers(
    In(mesh_entity): In<Entity>,
    mut commands: Commands,
    cameras: Cameras,
) {
    commands.entity(mesh_entity).insert(cameras.all_layers());
}
