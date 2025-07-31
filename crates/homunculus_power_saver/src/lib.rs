//! # Homunculus Power Saver
//!
//! This crate provides power management and performance optimization for the
//! Desktop Homunculus application, including frame rate limiting and loading
//! state management to reduce CPU and GPU usage when appropriate.
//!
//! ## Overview
//!
//! `homunculus_power_saver` implements intelligent power management by:
//! - Controlling frame rate limits based on user preferences
//! - Managing loading states for VRM models and animations
//! - Providing dynamic frame rate adjustment capabilities
//! - Integrating with the preferences system for persistent settings
//!
//! ## Key Features
//!
//! - **Frame Rate Control**: Configurable FPS limiting with user preferences
//! - **Loading State Management**: Automatic loading state tracking for VRM assets
//! - **Dynamic Adjustment**: Runtime frame rate changes through events
//! - **Preference Integration**: Persistent frame rate settings through preferences database
//! - **Asset Hook System**: Automatic loading state application for VRM assets
//!
//! ## Frame Rate Management
//!
//! The system loads frame rate preferences at startup and applies them using
//! the bevy_framepace plugin. The default frame rate is 60 FPS if no preference
//! is set.
//!
//! ## Loading State System
//!
//! The plugin automatically applies loading states to VRM assets (VrmHandle, VrmaHandle)
//! when they are spawned, and removes them when the assets are fully initialized.
//! This helps with performance management during asset loading phases.

use bevy::app::{App, Plugin, Update};
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_vrm1::prelude::*;
use homunculus_core::prelude::Loading;
use homunculus_prefs::PrefsDatabase;
use std::time::Duration;

#[derive(Event, Debug, Copy, Clone)]
pub struct RequestUpdateFrameRate(pub f64);

/// Plugin that provides power management and performance optimization for Homunculus.
///
/// This plugin manages frame rate limiting, loading states, and performance optimization
/// to reduce power consumption and improve battery life on mobile devices while
/// maintaining smooth operation.
///
/// # Features
///
/// - **Frame Rate Limiting**: Uses bevy_framepace to limit frame rate based on user preferences
/// - **Loading State Management**: Automatically tracks loading states for VRM assets
/// - **Dynamic Frame Rate**: Supports runtime frame rate changes through events
/// - **Preference Integration**: Loads and persists frame rate settings
///
/// # Systems
///
/// - `load_max_fps`: Loads frame rate preference at startup (default: 60 FPS)
/// - `remove_loading`: Removes loading components when assets are initialized
/// - `request_update_frame_rate`: Observer for dynamic frame rate changes
///
/// # Asset Integration
///
/// The plugin automatically registers component hooks for VRM assets:
/// - `VrmHandle`: VRM model handles get loading components when added
/// - `VrmaHandle`: VRM animation handles get loading components when added
///
/// These loading components are automatically removed when the assets are
/// fully initialized, helping track loading progress and optimize performance.
pub struct HomunculusPowerSaverPlugin;

impl Plugin for HomunculusPowerSaverPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin)
            .add_systems(Startup, load_max_fps)
            .add_systems(Update, remove_loading)
            .add_observer(request_update_frame_rate);
        register_loading_target::<VrmHandle>(app);
        register_loading_target::<VrmaHandle>(app);
    }
}

pub const MAX_FPS_KEY: &str = "max_fps";

fn load_max_fps(mut commands: Commands, db: NonSend<PrefsDatabase>) {
    let max_fps = db
        .load(MAX_FPS_KEY)
        .and_then(|v| v.as_f64())
        .unwrap_or(60.0);
    info!("[FPS] Loaded max FPS: {max_fps}");
    commands.insert_resource(FramepaceSettings {
        limiter: Limiter::Manual(Duration::from_secs_f64(1.0 / max_fps)),
    });
}

fn register_loading_target<C: Component>(app: &mut App) {
    app.world_mut().register_component_hooks::<C>().on_add(
        |mut world: DeferredWorld, context: HookContext| {
            world.commands().entity(context.entity).insert(Loading);
        },
    );
}

// fn update_active_status(
//     mut commands: Commands,
//     mut windows: Query<&mut Window, With<RenderLayers>>,
//     mut is_sleep: Local<bool>,
//     loading_contents: Query<&Loading>,
// ) {
//     match (*is_sleep, loading_contents.is_empty()) {
//         (true, false) => {
//             info!("[Power]: Active");
//             *is_sleep = false;
//             commands.insert_resource(WinitSettings::game());
//         }
//         (false, true) => {
//             info!("[Power]: Sleep");
//             *is_sleep = true;
//             commands.insert_resource(WinitSettings {
//                 focused_mode: UpdateMode::reactive_low_power(Duration::from_secs_f32(1.0)),
//                 unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(1)),
//             });
//             windows.par_iter_mut().for_each(|mut window| {
//                 //FIXME: If the hit test is true before transitioning to sleep mode, operations on other windows will be blocked,
//                 // but this response seems incomplete.
//                 window.cursor_options.hit_test = false;
//             });
//         }
//         _ => {}
//     }
// }

fn remove_loading(
    mut commands: Commands,
    entities: Query<Entity, (With<Loading>, Added<Initialized>)>,
) {
    for entity in entities.iter() {
        commands.entity(entity).remove::<Loading>();
    }
}

fn request_update_frame_rate(trigger: Trigger<RequestUpdateFrameRate>, mut commands: Commands) {
    commands.insert_resource(FramepaceSettings {
        limiter: Limiter::Manual(Duration::from_secs_f64(1.0 / trigger.0)),
    });
}
