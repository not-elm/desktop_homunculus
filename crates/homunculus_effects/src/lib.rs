//! # Homunculus Effects
//!
//! This crate provides visual and audio effects for the Desktop Homunculus application,
//! including sound effects and visual stamps that can be triggered by user interactions
//! or system events.
//!
//! ## Overview
//!
//! `homunculus_effects` enables rich interactive feedback through various effect types.
//! The system includes sound effects for audio feedback and visual stamps for
//! graphical feedback, both integrated with the VRM mascot system and desktop windows.
//!
//! ## Key Features
//!
//! - **Sound Effects**: Play audio files for user interactions and system events
//! - **Visual Stamps**: Display temporary visual effects on the desktop
//! - **Camera Management**: Automatic setup of 2D cameras for effect rendering
//! - **Multi-Window Support**: Effects can be displayed on any desktop window
//! - **Render Layer Integration**: Proper layering of effects with other UI elements
//!
//! ## Camera System
//!
//! The plugin automatically sets up 2D cameras for effect rendering on each window.
//! These cameras are properly positioned and configured to render effects in the
//! correct coordinate space relative to desktop windows.
//!
//! ## Render Integration
//!
//! Effects are rendered using Bevy's standard rendering pipeline and integrate
//! with the homunculus render layer system to ensure proper drawing order and
//! visual consistency with VRM models and other UI elements.

// mod effect;
mod sound;
mod stamp;

use crate::sound::SoundEffectsPlugin;
use crate::stamp::StampEffectsPlugin;
pub use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use bevy::window::WindowRef;
use bevy::winit::WinitWindows;
use homunculus_core::prelude::CameraOrders;
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use crate::{sound::RequestSoundEffect, stamp::*};
}

/// Main plugin that provides visual and audio effects for the Homunculus application.
///
/// This plugin coordinates all effect systems including sound effects, visual stamps,
/// and the camera infrastructure needed to render effects properly across multiple
/// desktop windows.
///
/// # Included Components
///
/// - `SoundEffectsPlugin`: Handles audio playback for sound effects
/// - `StampEffectsPlugin`: Manages visual stamp effects on the desktop
/// - Camera setup system: Automatically creates 2D cameras for each window
/// - Camera positioning system: Properly positions cameras relative to desktop windows
///
/// # Camera Management
///
/// The plugin automatically creates a 2D camera for each window in the application.
/// These cameras are:
/// - Configured for UI rendering order
/// - Positioned relative to their target window's desktop location
/// - Integrated with the render layer system
/// - Automatically initialized when windows are created
///
/// # Effect Rendering
///
/// Effects are rendered through the standard Bevy rendering pipeline and properly
/// integrate with:
/// - Multi-window desktop applications
/// - VRM model rendering
/// - UI layer management
/// - Proper depth sorting and transparency
///
/// # Systems
///
/// The plugin adds the following systems:
/// - `spawn_camera2d`: Creates effect cameras for each window at startup
/// - `initialize_camera_position`: Positions cameras correctly relative to desktop windows
pub struct HomunculusEffectsPlugin;

impl Plugin for HomunculusEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SoundEffectsPlugin, StampEffectsPlugin))
            .add_systems(Startup, spawn_camera2d)
            .add_systems(Update, initialize_camera_position);
    }
}

fn spawn_camera2d(mut commands: Commands, windows: Query<(Entity, &RenderLayers), With<Window>>) {
    for (entity, layer) in windows.iter() {
        commands.spawn((
            Camera2d,
            Camera {
                order: CameraOrders::UI,
                target: RenderTarget::Window(WindowRef::Entity(entity)),
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            layer.clone(),
            UninitializedEffectCamera,
        ));
    }
}

#[derive(Component, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
struct UninitializedEffectCamera;

fn initialize_camera_position(
    mut commands: Commands,
    cameras: Query<
        (Entity, &Camera, &Transform, &GlobalTransform),
        With<UninitializedEffectCamera>,
    >,
    winit_window: NonSend<WinitWindows>,
) {
    cameras.iter().for_each(|(entity, camera, camera_tf, gtf)| {
        let RenderTarget::Window(WindowRef::Entity(window_entity)) = camera.target else {
            return;
        };
        let Some(window) = winit_window.get_window(window_entity) else {
            return;
        };
        let pos = window.outer_position().unwrap().cast();
        let pos = Vec2::new(pos.x, pos.y);
        let center = camera.logical_viewport_size().unwrap() / 2.;
        let Ok(ray) = camera.viewport_to_world(gtf, center + pos) else {
            return;
        };
        let plane = InfinitePlane3d::new(gtf.back());
        let distance = ray.intersect_plane(Vec3::ZERO, plane).unwrap_or(0.);
        let camera_pos = ray.get_point(distance).with_z(camera_tf.translation.z);

        commands
            .entity(entity)
            .insert(Transform::from_translation(camera_pos))
            .remove::<UninitializedEffectCamera>();
    });
}
