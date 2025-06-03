use bevy::app::{App, Plugin, Update};
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::log::debug;
use bevy::prelude::{Commands, Component, Local, Query, Reflect};
use bevy::winit::{UpdateMode, WinitSettings};
use bevy_vrm1::vrm::loader::VrmHandle;
use bevy_vrm1::vrma::VrmaHandle;
use serde::{Deserialize, Serialize};

/// If this component exists, the application is in active state.
#[derive(Debug, Reflect, Component, Copy, Clone, Serialize, Deserialize)]
pub struct Loading;

pub struct PowerStatePlugin;

impl Plugin for PowerStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Loading>()
            .add_systems(Update, update_active_status);

        register_loading_target::<VrmHandle>(app);
        register_loading_target::<VrmaHandle>(app);
    }
}

fn register_loading_target<C: Component>(app: &mut App) {
    app.world_mut().register_component_hooks::<C>().on_add(
        |mut world: DeferredWorld, context: HookContext| {
            world.commands().entity(context.entity).insert(Loading);
        },
    );
}

fn update_active_status(
    mut commands: Commands,
    #[cfg(not(feature = "develop"))] mut windows: Query<&mut bevy::window::Window>,
    mut is_sleep: Local<bool>,
    loading_contents: Query<&Loading>,
) {
    match (*is_sleep, loading_contents.is_empty()) {
        (true, false) => {
            debug!("Change to PowerState: Active");
            *is_sleep = false;
            commands.insert_resource(WinitSettings {
                unfocused_mode: UpdateMode::Continuous,
                focused_mode: UpdateMode::Continuous,
            });
        }
        (false, true) => {
            debug!("Change to PowerState: Sleep");
            *is_sleep = true;
            commands.insert_resource(WinitSettings::desktop_app());
            #[cfg(not(feature = "develop"))]
            windows.par_iter_mut().for_each(|mut window| {
                //FIXME: If the hit test is true before transitioning to sleep mode, operations on other windows will be blocked,
                // but this response seems incomplete.
                window.cursor_options.hit_test = false;
            });
        }
        _ => {}
    }
}
