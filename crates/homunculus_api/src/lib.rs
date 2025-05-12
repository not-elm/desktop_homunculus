//! # Homunculus API
//!
//! This crate provides a comprehensive API layer for the Desktop Homunculus application,
//! enabling communication between different components and external processes.
//!
//! ## Overview
//!
//! `homunculus_api` serves as the primary interface for controlling the desktop mascot
//! application. It provides structured APIs for managing VRM models, animations, user
//! interactions, preferences, and various system integrations.
//!
//! ## Key Features
//!
//! - **VRM Model Management**: Spawn, control, and manage 3D VRM mascot models
//! - **Animation System**: Play and control VRMA animations on VRM models
//! - **GPT Integration**: Interface with AI chat systems for interactive conversations
//! - **Speech Synthesis**: Text-to-speech functionality using VoiceVox
//! - **Camera Control**: Manage 2D cameras and viewport settings
//! - **Effects System**: Apply visual effects, stamps, and sounds
//! - **WebView Integration**: Embed and control web content within the application
//! - **Preferences Management**: Store and retrieve user preferences
//! - **Entity Management**: Query and manipulate entities within the Bevy world
//!
//! ## Plugin Architecture
//!
//! The crate is organized around Bevy's plugin system, with the main [`HomunculusApiPlugin`]
//! providing all API functionality:
//!
//! ## Core Components
//!
//! - [`ApiReactor`](prelude::ApiReactor): Central communication hub for all API operations
//! - [`CommandsApi`](prelude::CommandsApi): Execute commands and stream results
//! - VRM APIs: Model spawning, state management, and animation control
//! - GPT APIs: AI chat integration and model management
//! - WebView APIs: Web content embedding and control
mod app;
mod cameras;
mod commands;
mod display;
mod effects;
mod entities;
mod error;
mod gpt;
mod mods;
pub mod preferences;
mod reactor;
mod scripts;
mod settings;
mod shadow_panel;
mod speech;
pub mod vrm;
mod vrma;
mod webview;

use crate::commands::CommandsApiPlugin;
use crate::gpt::GptApiPlugin;
use crate::prelude::{ShadowPanelApiPlugin, WebviewApiPlugin};
use crate::reactor::ApiReactorPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod prelude {
    pub use crate::{
        app::*, cameras::*, commands::CommandsApi, display::*, effects::*, entities::*, error::*,
        gpt::*, mods::*, preferences::*, reactor::*, scripts::*, settings::*, shadow_panel::*,
        speech::*, vrm::*, vrma::*, webview::*,
    };
}

/// Main plugin group that provides all API functionality for the Homunculus application.
///
/// This plugin group bundles together all the individual API plugins, providing a
/// comprehensive interface for controlling the desktop mascot. It includes APIs for
/// VRM model management, GPT integration, WebView control, command execution, and
/// shadow panel management.
///
/// # Included Plugins
///
/// - `ApiReactorPlugin`: Core communication system
/// - `WebviewApiPlugin`: WebView management and control
/// - `GptApiPlugin`: AI chat integration
/// - `CommandsApiPlugin`: Command execution and streaming
/// - `ShadowPanelApiPlugin`: Shadow rendering control
pub struct HomunculusApiPlugin;

impl PluginGroup for HomunculusApiPlugin {
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>();
        builder
            .add(ApiReactorPlugin)
            .add(WebviewApiPlugin)
            .add(GptApiPlugin)
            .add(CommandsApiPlugin)
            .add(ShadowPanelApiPlugin)
            .build()
    }
}

/// Macro for creating API resource types that wrap [`ApiReactor`](prelude::ApiReactor).
///
/// This macro simplifies the creation of API resource types by automatically generating
/// the necessary boilerplate code. The generated types implement `Clone`, `Resource`,
/// and `Deref` to provide convenient access to the underlying `ApiReactor`.
///
/// # Usage
///
/// ```rust
/// use homunculus_api::api;
///
/// api! {
///     /// Custom API for specific functionality.
///     MyCustomApi
/// }
/// ```
#[macro_export]
macro_rules! api {
    (
        $(#[$meta:meta])*
        $name: ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, bevy::prelude::Resource, bevy::prelude::Deref)]
        pub struct $name($crate::prelude::ApiReactor);

        impl From<$crate::prelude::ApiReactor> for $name {
            fn from(reactor: $crate::prelude::ApiReactor) -> Self {
                Self(reactor)
            }
        }
    };
}
