//! # Homunculus Hit Test
//!
//! This crate provides mouse hit testing functionality for VRM models in the
//! Desktop Homunculus application, enabling proper cursor interaction and
//! window transparency management.
//!
//! ## Overview
//!
//! `homunculus_hit_test` implements a sophisticated hit testing system that
//! determines when the mouse cursor is over VRM models. This is crucial for
//! desktop mascot applications where the background should be transparent
//! and only the 3D model should be interactive.
//!
//! ## Key Features
//!
//! - **3D Mesh Ray Casting**: Accurate hit detection using 3D ray casting against VRM geometry
//! - **Window Transparency Control**: Automatic window hit test enabling/disabling for proper transparency
//! - **Development Mode Support**: Special handling for development UI overlays
//! - **Multi-Window Support**: Independent hit testing for each application window
//! - **Automatic VRM Integration**: Automatically sets up hit testing for newly spawned VRM models
//!
//! ## How It Works
//!
//! 1. **Ray Casting**: The system casts rays from the camera through the cursor position
//! 2. **Mesh Intersection**: Tests for intersection with VRM model geometry (MToon materials)
//! 3. **Window Hit Test Control**: Enables/disables window hit testing based on whether cursor is over 3D content
//! 4. **Transparency Management**: When cursor is not over 3D content, window becomes transparent to mouse events
//!
//! ## Development Features
//!
//! In development mode (with "develop" feature), the system includes special handling
//! for UI overlays and development tools, ensuring they remain interactive even when
//! the main application window is transparent.

use bevy::app::{App, Plugin};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::utils::default;
use bevy::window::Window;
use bevy_vrm1::prelude::{Cameras, MToonMaterial};
use bevy_vrm1::vrm::Vrm;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Reflect, Event)]
struct RequestDetermineHitTest;

/// Plugin that provides 3D hit testing for VRM models and window transparency management.
///
/// This plugin enables proper mouse interaction with VRM models while maintaining
/// desktop transparency when the cursor is not over 3D content. It uses ray casting
/// to determine precise intersection with VRM geometry.
///
/// # Behavior
///
/// - **Automatic Setup**: Newly spawned VRM models automatically get hit testing observers
/// - **Ray Casting**: Uses 3D ray casting from camera through cursor position
/// - **Material Filtering**: Only tests against MToon materials (VRM standard)
/// - **Window Control**: Automatically enables/disables window hit testing for transparency
/// - **Development Support**: Special handling for development UI in "develop" feature mode
///
/// # Performance
///
/// - In release mode: Hit testing only runs when cursor enters/exits windows or VRM models
/// - In development mode: Hit testing runs every frame for responsive UI interaction
/// - Ray casting is optimized to only test against VRM materials
pub struct HomunculusHitTestPlugin;

impl Plugin for HomunculusHitTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestDetermineHitTest>()
            .add_systems(
                PreUpdate,
                request_determine_hit_test.run_if(on_event::<CursorEntered>),
            )
            .add_systems(
                Update,
                (
                    observe_hit_test,
                    #[cfg(feature = "develop")]
                    update_hit_test,
                    #[cfg(not(feature = "develop"))]
                    update_hit_test.run_if(on_event::<RequestDetermineHitTest>),
                ),
            );
    }
}

fn request_determine_hit_test(mut request: EventWriter<RequestDetermineHitTest>) {
    request.write(RequestDetermineHitTest);
}

fn update_hit_test(
    mut mesh_ray_cast: MeshRayCast,
    mut windows: Query<(Entity, &mut Window), With<RenderLayers>>,
    cameras: Cameras,
    mtoon_materials: Query<&MeshMaterial3d<MToonMaterial>>,
    #[cfg(feature = "develop")] mut ctx: bevy_egui::EguiContexts,
) {
    #[cfg(feature = "develop")]
    let on_egui = ctx.ctx_mut().is_ok_and(|c| c.is_pointer_over_area());

    for (window_entity, mut window) in windows.iter_mut() {
        let Some(cursor_pos) = window.cursor_position() else {
            window.cursor_options.hit_test = false;
            continue;
        };
        let Some((_, camera, tf, _)) = cameras.find_camera_from_window(window_entity) else {
            window.cursor_options.hit_test = false;
            continue;
        };
        let Ok(ray) = camera.viewport_to_world(tf, cursor_pos) else {
            window.cursor_options.hit_test = false;
            continue;
        };
        let hitting_anyone = !mesh_ray_cast
            .cast_ray(
                ray,
                &MeshRayCastSettings {
                    filter: &|e| mtoon_materials.get(e).is_ok(),
                    ..default()
                },
            )
            .is_empty();

        match (hitting_anyone, window.cursor_options.hit_test) {
            (true, false) => window.cursor_options.hit_test = true,
            (false, true) => window.cursor_options.hit_test = false,
            _ => (),
        }
        #[cfg(feature = "develop")]
        if on_egui {
            window.cursor_options.hit_test = true;
        }
    }
}

fn observe_hit_test(mut commands: Commands, vrms: Query<Entity, Added<Vrm>>) {
    for vrm in vrms.iter() {
        commands
            .entity(vrm)
            .observe(apply_pointer::<Over>)
            .observe(apply_pointer::<Out>);
    }
}

fn apply_pointer<P: Debug + Clone + Reflect>(
    _: Trigger<Pointer<P>>,
    mut ew: EventWriter<RequestDetermineHitTest>,
) {
    ew.write(RequestDetermineHitTest);
}
