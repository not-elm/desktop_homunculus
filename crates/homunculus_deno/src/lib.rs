//! # Homunculus Deno
//!
//! This crate provides Deno (JavaScript/TypeScript) runtime integration for the
//! Desktop Homunculus application, enabling dynamic script execution and hot
//! reloading capabilities.
//!
//! ## Overview
//!
//! `homunculus_deno` allows the Homunculus application to execute JavaScript and
//! TypeScript code through the Deno runtime. This enables dynamic behavior,
//! scripting capabilities, and extensibility through user-defined scripts.
//!
//! ## Key Features
//!
//! - **Deno Runtime Integration**: Embedded Deno runtime for JavaScript/TypeScript execution
//! - **Asset System**: Seamless integration with Bevy's asset system for script loading
//! - **Hot Reloading**: Automatic reloading and re-execution of scripts when files change
//! - **Event Loop Management**: Proper integration with Bevy's game loop and Deno's event loop
//! - **Script Lifecycle**: Automatic cleanup and management of script resources
//!
//! ### Hot Reloading
//!
//! Scripts are automatically reloaded when their source files change during
//! development, enabling rapid iteration and testing of script behavior.
//!
//! ## Architecture
//!
//! The crate consists of two main components:
//!
//! - **Asset System**: Handles loading and management of JavaScript/TypeScript files
//! - **Runtime System**: Manages the Deno runtime and script execution
//!
//! Scripts are executed asynchronously and can interact with the Homunculus
//! application through exposed APIs and interfaces.

mod asset;
mod runtime;

use crate::asset::{DenoAssetPlugin, DenoScript, DenoScriptHandle};
use crate::runtime::{DenoRuntimePlugin, RequestDeno, RequestSender};
use bevy::prelude::*;
use homunculus_core::prelude::OutputLog;
use std::collections::HashSet;

pub mod prelude {
    pub use crate::asset::*;
}

/// Main plugin that provides Deno JavaScript/TypeScript runtime integration.
///
/// This plugin bundles together all components needed for Deno script execution,
/// including asset loading, runtime management, hot reloading, and event loop
/// integration with Bevy's game loop.
///
/// # Included Components
///
/// - `DenoAssetPlugin`: Handles loading of JavaScript/TypeScript files as assets
/// - `DenoRuntimePlugin`: Manages the Deno runtime and script execution
/// - Event loop integration: Properly ticks Deno's event loop within Bevy's game loop
/// - Hot reload system: Automatically reloads and re-executes scripts when files change
/// - Script lifecycle management: Tracks and manages script handles and resources
///
/// # Systems
///
/// The plugin adds several systems that run automatically:
///
/// - `tick_event_loop`: Ensures Deno's event loop is properly ticked each frame
/// - `request_call_javascript`: Executes newly loaded scripts
/// - `hot_reload`: Handles automatic reloading of modified scripts
///
/// # Script Execution
///
/// Scripts are executed by spawning entities with `DenoScriptHandle` components.
/// The plugin automatically detects these entities and executes the associated
/// scripts in the Deno runtime.
pub struct HomunculusDenoPlugin;

impl Plugin for HomunculusDenoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DenoAssetPlugin, DenoRuntimePlugin))
            .init_resource::<DenoScriptHandleStore>()
            .add_systems(FixedPreUpdate, tick_event_loop)
            .add_systems(Update, (request_call_javascript, hot_reload));
    }
}

fn request_call_javascript(
    mut commands: Commands,
    mut store: ResMut<DenoScriptHandleStore>,
    sender: Res<RequestSender>,
    scripts: Res<Assets<DenoScript>>,
    handles: Query<(Entity, &DenoScriptHandle)>,
) {
    handles.iter().for_each(|(entity, handle)| {
        let Some(script) = scripts.get(handle.0.id()).cloned() else {
            return;
        };
        sender
            .send_blocking(RequestDeno::CallScript {
                name: None,
                content: script.0,
            })
            .output_log_if_error("DenoScript::CallScript");
        store.0.insert(handle.0.clone());
        commands.entity(entity).despawn();
    });
}

fn hot_reload(
    mut er: EventReader<AssetEvent<DenoScript>>,
    sender: Res<RequestSender>,
    scripts: Res<Assets<DenoScript>>,
) {
    for event in er.read() {
        if let AssetEvent::Modified { id } = event
            && let Some(script) = scripts.get(*id).cloned()
        {
            sender
                .send_blocking(RequestDeno::CallScript {
                    name: None,
                    content: script.0,
                })
                .output_log_if_error("DenoScript::CallScript");
        }
    }
}

fn tick_event_loop(sender: Res<RequestSender>) {
    let _ = sender.send_blocking(RequestDeno::TickEventLoop);
}

// pub static HOMUNCULUS_SNAPSHOT: Option<&[u8]> = Some(include_bytes!(concat!(
//     env!("OUT_DIR"),
//     "/HOMUNCULUS_SNAPSHOT.bin"
// )));

#[derive(Resource, Debug, Default)]
struct DenoScriptHandleStore(HashSet<Handle<DenoScript>>);
