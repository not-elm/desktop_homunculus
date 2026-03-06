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
//! - **Texture Alpha Testing**: Ignores hits on transparent parts of textures (alpha cutout)
//! - **Window Transparency Control**: Automatic window hit test enabling/disabling for proper transparency
//! - **Development Mode Support**: Special handling for development UI overlays
//! - **Multi-Window Support**: Independent hit testing for each application window
//! - **Automatic VRM Integration**: Automatically sets up hit testing for newly spawned VRM models
//!
//! ## How It Works
//!
//! 1. **Ray Casting**: The system casts rays from the camera through the cursor position
//! 2. **Mesh Intersection**: Tests for intersection with VRM model geometry (MToon materials)
//! 3. **Alpha Testing**: For materials with alpha cutout, samples texture to skip transparent regions
//! 4. **Window Hit Test Control**: Enables/disables window hit testing based on whether cursor is over 3D content
//! 5. **Transparency Management**: When cursor is not over 3D content, window becomes transparent to mouse events
//!
//! ## Development Features
//!
//! In development mode (with "develop" feature), the system includes special handling
//! for UI overlays and development tools, ensuring they remain interactive even when
//! the main application window is transparent.

use bevy::app::{App, Plugin};
use bevy::camera::visibility::RenderLayers;
use bevy::image::Image;
use bevy::pbr::MeshMaterial3d;
use bevy::picking::mesh_picking::ray_cast::RayMeshHit;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::window::{CursorOptions, Window, WindowPosition};
use bevy_cef::prelude::WebviewExtendStandardMaterial;
use bevy_vrm1::prelude::{Cameras, MToonMaterial};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Reflect, Message)]
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
        app.add_message::<RequestDetermineHitTest>()
            .add_systems(
                PreUpdate,
                request_determine_hit_test.run_if(on_message::<CursorEntered>),
            )
            .add_systems(Update, (observe_hit_test, update_hit_test));
    }
}

fn request_determine_hit_test(mut request: MessageWriter<RequestDetermineHitTest>) {
    request.write(RequestDetermineHitTest);
}

#[allow(clippy::too_many_arguments)]
fn update_hit_test(
    mut mesh_ray_cast: MeshRayCast,
    mut windows: Query<(Entity, &Window, &mut CursorOptions), With<RenderLayers>>,
    cameras: Cameras,
    mtoon_materials: Query<&MeshMaterial3d<MToonMaterial>>,
    webview_materials: Query<&MeshMaterial3d<WebviewExtendStandardMaterial>>,
    webview_material_assets: Res<Assets<WebviewExtendStandardMaterial>>,
    image_assets: Res<Assets<Image>>,
    #[cfg(feature = "develop")] mut ctx: bevy_egui::EguiContexts,
) {
    #[cfg(feature = "develop")]
    let on_egui = ctx.ctx_mut().is_ok_and(|c| c.is_pointer_over_area());

    for (window_entity, window, mut cursor) in windows.iter_mut() {
        let cursor_pos = match window.cursor_position() {
            Some(pos) => pos,
            None => match fallback_cursor_position(window) {
                Some(pos) => pos,
                None => {
                    cursor.hit_test = false;
                    continue;
                }
            },
        };
        let Some((_, camera, _, tf, _)) = cameras.find_camera_from_window(window_entity) else {
            cursor.hit_test = false;
            continue;
        };
        let Ok(ray) = camera.viewport_to_world(tf, cursor_pos) else {
            cursor.hit_test = false;
            continue;
        };

        let hits = mesh_ray_cast.cast_ray(
            ray,
            &MeshRayCastSettings {
                visibility: RayCastVisibility::VisibleInView,
                filter: &|e| mtoon_materials.get(e).is_ok() || webview_materials.get(e).is_ok(),
                early_exit_test: &|_| false,
            },
        );

        let hitting_anyone = hits.iter().any(|(entity, hit)| {
            if mtoon_materials.get(*entity).is_ok() {
                return true;
            }
            is_hit_opaque(
                *entity,
                hit,
                &webview_materials,
                &webview_material_assets,
                &image_assets,
            )
        });

        match (hitting_anyone, cursor.hit_test) {
            (true, false) => cursor.hit_test = true,
            (false, true) => cursor.hit_test = false,
            _ => (),
        }
        #[cfg(feature = "develop")]
        if on_egui {
            cursor.hit_test = true;
        }
    }
}

fn observe_hit_test(mut commands: Commands, vrms: Query<Entity, Added<Mesh3d>>) {
    for vrm in vrms.iter() {
        commands
            .entity(vrm)
            .observe(apply_pointer::<Over>)
            .observe(apply_pointer::<Out>);
    }
}

fn apply_pointer<P: Debug + Clone + Reflect>(
    _: On<Pointer<P>>,
    mut ew: MessageWriter<RequestDetermineHitTest>,
) {
    ew.write(RequestDetermineHitTest);
}

/// Returns the cursor position in window-local logical coordinates by polling the OS.
///
/// On Windows, uses `GetCursorPos` to get the global cursor position and converts
/// it to window-local coordinates. On other platforms, returns `None` since Bevy's
/// event-driven cursor tracking is sufficient.
fn fallback_cursor_position(window: &Window) -> Option<Vec2> {
    #[cfg(target_os = "windows")]
    {
        get_cursor_pos_in_window(window)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = window;
        None
    }
}

#[cfg(target_os = "windows")]
fn get_cursor_pos_in_window(window: &Window) -> Option<Vec2> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point).ok()? };

    let WindowPosition::At(window_pos) = window.position else {
        return None;
    };

    let scale = window.scale_factor();
    let global_logical = Vec2::new(point.x as f32 / scale, point.y as f32 / scale);
    let window_logical = global_logical - window_pos.as_vec2();

    let size = window.resolution.size();
    if window_logical.x >= 0.0
        && window_logical.y >= 0.0
        && window_logical.x <= size.x
        && window_logical.y <= size.y
    {
        Some(window_logical)
    } else {
        None
    }
}

/// Checks if a ray cast hit is on an opaque part of the texture.
///
/// For materials with `AlphaMode::Mask`, this samples the texture at the hit's UV
/// coordinates and compares the alpha value against the cutoff threshold.
/// Returns `true` if the hit should be considered solid (opaque).
fn is_hit_opaque(
    entity: Entity,
    hit: &RayMeshHit,
    webview_materials: &Query<&MeshMaterial3d<WebviewExtendStandardMaterial>>,
    material_assets: &Assets<WebviewExtendStandardMaterial>,
    image_assets: &Assets<Image>,
) -> bool {
    // If no UV data, assume opaque
    let Some(uv) = hit.uv else {
        return true;
    };

    // Get material handle
    let Ok(material_handle) = webview_materials.get(entity) else {
        return true;
    };

    // Get material asset
    let Some(material) = material_assets.get(&material_handle.0) else {
        return true;
    };

    let Some(texture_handle) = &material.extension.surface else {
        return true;
    };

    // Get texture asset
    let Some(image) = image_assets.get(texture_handle) else {
        return true;
    };

    // Sample alpha at UV and compare against cutoff
    let alpha = sample_texture_alpha(image, uv);
    alpha > 0.0
}

/// Samples the alpha value from a texture at the given UV coordinates.
///
/// Handles UV wrapping and various texture formats. Returns 1.0 (fully opaque)
/// if the texture format doesn't have an alpha channel or is unsupported.
fn sample_texture_alpha(image: &Image, uv: Vec2) -> f32 {
    let width = image.width();
    let height = image.height();

    if width == 0 || height == 0 {
        return 1.0;
    }

    // Get pixel data, return opaque if not available
    let Some(data) = &image.data else {
        return 1.0;
    };

    // Wrap UV to [0, 1) using Euclidean remainder for proper negative handling
    let u = uv.x.rem_euclid(1.0);
    let v = uv.y.rem_euclid(1.0);

    // Convert to pixel coordinates
    // Flip V coordinate (1.0 - v) because texture coordinates typically have Y=0 at bottom
    let x = ((u * width as f32) as usize).min(width as usize - 1);
    let y = ((v * height as f32) as usize).min(height as usize - 1);

    // Get alpha based on texture format
    match image.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            // RGBA8: 4 bytes per pixel, alpha at offset 3
            let idx = (y * width as usize + x) * 4 + 3;
            data.get(idx).map(|&a| a as f32 / 255.0).unwrap_or(1.0)
        }
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            // BGRA8: 4 bytes per pixel, alpha is still at offset 3
            let idx = (y * width as usize + x) * 4 + 3;
            data.get(idx).map(|&a| a as f32 / 255.0).unwrap_or(1.0)
        }
        // Unknown or unsupported format - treat as opaque
        _ => 1.0,
    }
}
