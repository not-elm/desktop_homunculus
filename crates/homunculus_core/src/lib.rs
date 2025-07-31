//! # Homunculus Core
//!
//! This crate provides the foundational components, events, resources, and utilities
//! for the Desktop Homunculus application. It serves as the core infrastructure
//! upon which other homunculus crates are built.
//!
//! ## Overview
//!
//! `homunculus_core` contains essential functionality shared across the entire
//! Homunculus ecosystem, including core components for VRM models, event systems
//! for handling user interactions, system parameters for common operations, and
//! standardized error handling.
//!
//! ## Key Features
//!
//! - **Core Components**: Essential Bevy components like `Loading`, `ShadowPanel`,
//!   `GlobalViewport`, `PrimaryCamera`, and `VrmState`
//! - **Event System**: Comprehensive event handling for VRM models, pointer
//!   interactions, and state changes
//! - **System Parameters**: Custom Bevy system parameters for common operations
//!   like bone offset calculations, camera management, and coordinate transformations
//! - **Resource Management**: Core resources for managing application state
//! - **Path Utilities**: Helper functions for working with file paths and assets
//! - **Error Handling**: Standardized error types and handling throughout the application
//! - **Render Layers**: Management of VRM-specific render layers
//!
//! ## Plugin Architecture
//!
//! The crate is organized around Bevy's plugin system with the main
//! [`HomunculusCorePlugin`] providing all core functionality:
//!
//! ```rust
//! use bevy::prelude::*;
//! use homunculus_core::HomunculusCorePlugin;
//!
//! App::new()
//!     .add_plugins(HomunculusCorePlugin)
//!     .run();
//! ```
//!
//! ## Core Components
//!
//! ### VRM Management
//! - `VrmState`: Tracks the current state of VRM models (idle, sitting, etc.)
//! - `Loading`: Indicates when VRM models are being loaded
//! - Various bone and animation-related components
//!
//! ### Camera System
//! - `PrimaryCamera`: Marks the primary camera for rendering
//! - `GlobalViewport`: Manages viewport settings across displays
//!
//! ### Visual Effects
//! - `ShadowPanel`: Controls shadow rendering for VRM models
//!
//! ## System Parameters
//!
//! The crate provides several custom system parameters that simplify common operations:
//!
//! - [`BoneOffsets`](prelude::BoneOffsets): Calculate bone positions and offsets
//! - [`Camera2ds`](prelude::Camera2ds): Manage 2D camera operations
//! - [`Coordinate`](prelude::Coordinate): Handle coordinate transformations
//! - [`MascotTracker`](prelude::MascotTracker): Track mascot positions and movements
//! - [`Monitors`](prelude::Monitors): Access monitor and display information

use crate::events::HomunculusEventsPlugin;
use crate::prelude::CoreComponentsPlugin;
use crate::render_layers::VrmRenderLayersPlugin;
use crate::resources::CoreResourcesPlugin;
use bevy::app::{App, Plugin};

mod api;
mod components;
mod consts;
mod error;
mod events;
mod path;
mod render_layers;
mod resources;
mod schema;
mod system_param;
mod system_set;

pub mod prelude {
    pub use crate::{
        HomunculusCorePlugin, components::*, consts::prelude::*, error::*, events::prelude::*,
        path::*, resources::prelude::*, schema::prelude::*, system_param::prelude::*,
        system_set::HomunculusSystemSet,
    };
}

/// Core plugin that provides fundamental components, events, and resources for Homunculus.
///
/// This plugin bundles together all the essential functionality needed by the
/// Homunculus application, including component registration, event handling,
/// resource management, and render layer setup.
///
/// # Usage
///
/// Add this plugin to your Bevy app to enable core Homunculus functionality:
///
/// ```rust
/// use bevy::prelude::*;
/// use homunculus_core::HomunculusCorePlugin;
///
/// App::new()
///     .add_plugins(HomunculusCorePlugin)
///     .run();
/// ```
///
/// # Included Plugins
///
/// - `CoreComponentsPlugin`: Registers core components used throughout the application
/// - `HomunculusEventsPlugin`: Sets up the event system for VRM and interaction events
/// - `CoreResourcesPlugin`: Initializes core resources and state management
/// - `VrmRenderLayersPlugin`: Configures render layers for VRM model rendering
///
/// # Dependencies
///
/// This plugin should be added before other Homunculus plugins as it provides
/// the foundational systems they depend on.
pub struct HomunculusCorePlugin;

impl Plugin for HomunculusCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreComponentsPlugin,
            HomunculusEventsPlugin,
            CoreResourcesPlugin,
            VrmRenderLayersPlugin,
        ));
    }
}
