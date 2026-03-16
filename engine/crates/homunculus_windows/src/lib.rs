//! # Homunculus Windows
//!
//! This crate provides multi-window management for the Desktop Homunculus application,
//! creating transparent, always-on-top windows across multiple monitors with
//! proper camera setup and coordinate system management.
//!
//! ## Overview
//!
//! `homunculus_windows` manages the creation and positioning of transparent desktop
//! windows that serve as canvases for VRM mascot rendering. Each monitor gets its
//! own dedicated window with an associated camera, enabling mascots to appear on
//! any display in a multi-monitor setup.
//!
//! ## Key Features
//!
//! - **Multi-Monitor Support**: Automatic window creation for each detected monitor
//! - **Transparent Windows**: Fully transparent backgrounds with alpha compositing
//! - **Always On Top**: Windows stay above all other applications
//! - **Click-Through**: Windows are non-interactive by default (hit testing disabled)
//! - **Orthographic Cameras**: Proper 3D rendering with orthographic projection
//! - **Coordinate Alignment**: Cameras positioned to match desktop coordinate systems
//!
//! ## Window Configuration
//!
//! Each window is configured with:
//! - **Transparency**: Full alpha channel support for see-through backgrounds
//! - **No Decorations**: Borderless windows without title bars or controls
//! - **Always On Top**: Window level set to stay above all other applications
//! - **No Resizing**: Fixed size matching monitor dimensions
//! - **Click-Through**: Cursor hit testing disabled for desktop interaction
//!
//! ## Platform Considerations
//!
//! ### macOS
//! - Uses post-multiplied alpha compositing for proper transparency
//! - VSync enabled for smooth rendering
//! - Full monitor-sized windows
//!
//! ### Windows
//! - Windows are slightly smaller than monitor size (1 pixel smaller) to work around transparency issues
//! - Standard alpha compositing
//!
//! ## Camera System
//!
//! Each window gets a dedicated 3D camera configured with:
//! - **Orthographic Projection**: 3D rendering without perspective distortion
//! - **Fixed Vertical Scaling**: Consistent sizing across different monitor resolutions  
//! - **Desktop Alignment**: Camera positioned to match desktop coordinate system
//! - **Render Layers**: Each monitor uses a separate render layer for proper isolation
//!
//! ## Initialization Process
//!
//! 1. **Monitor Detection**: Discovers all available monitors and their properties
//! 2. **Window Creation**: Reuses Bevy's default primary window for the primary monitor,
//!    spawns new windows for secondary monitors
//! 3. **Camera Setup**: Spawns orthographic cameras for each window
//! 4. **Position Calculation**: Aligns cameras with desktop coordinate systems

use bevy::camera::visibility::RenderLayers;
use bevy::camera::{RenderTarget, ScalingMode};
use bevy::prelude::*;
use bevy::window::*;
use homunculus_core::prelude::{AppWindow, CameraOrders, PrimaryCamera};
use serde::{Deserialize, Serialize};

/// Plugin that provides multi-window management for desktop mascot rendering.
///
/// This plugin creates transparent, always-on-top windows across all monitors
/// with properly positioned cameras for VRM mascot rendering. Each monitor
/// gets its own dedicated window and camera for independent mascot display.
///
/// # Window Properties
///
/// Each created window has:
/// - Full transparency with alpha compositing
/// - Always-on-top window level
/// - No decorations (borderless)
/// - Click-through capability (hit testing disabled)
/// - Fixed size matching monitor dimensions
/// - Proper positioning on target monitor
///
/// # Camera Configuration
///
/// Cameras are configured with:
/// - Orthographic projection for consistent 3D rendering
/// - Fixed vertical scaling based on monitor aspect ratio
/// - Desktop-aligned positioning for proper coordinate mapping
/// - Render layer separation for multi-monitor isolation
///
/// # Systems
///
/// - `setup_windows`: Creates windows and cameras for each monitor (reuses the default primary window)
/// - `initialize_camera_position`: Aligns cameras with desktop coordinates
///
/// # Multi-Monitor Support
///
/// The plugin automatically detects all available monitors and creates
/// appropriate windows and cameras, enabling mascots to appear on any
/// display in the system.
pub struct HomunculusWindowsPlugin;

impl Plugin for HomunculusWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UninitializedCamera>()
            .register_type::<CameraWindowPosition>()
            .add_systems(PreStartup, setup_windows)
            .add_systems(Update, initialize_camera_position);
    }
}

#[derive(Component, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
struct UninitializedCamera;

fn setup_windows(
    mut commands: Commands,
    monitors: Query<(Entity, &Monitor, Option<&PrimaryMonitor>)>,
    default_window: Query<Entity, With<PrimaryWindow>>,
) {
    let default_window_entity = default_window
        .single()
        .expect("failed to obtain the primary window entity");
    for (layer, (monitor_entity, monitor, primary)) in monitors.iter().enumerate() {
        let mut window = create_window(layer, monitor.physical_size().as_vec2());
        let s = monitor.scale_factor as f32;
        let window_position = monitor.physical_position.as_vec2() / s;
        window.position.set(window_position.as_ivec2());
        window.resolution.set_scale_factor(s);

        let window_entity = if primary.is_some() {
            default_window_entity
        } else {
            commands.spawn_empty().id()
        };

        commands.entity(window_entity).insert((
            Name::new(format!("Window({:?})", monitor.physical_position)),
            RenderLayers::layer(layer),
            window,
            AppWindow,
            CursorOptions {
                hit_test: true,
                ..default()
            },
        ));

        commands
            .entity(monitor_entity)
            .try_insert(RenderLayers::layer(layer));

        spawn_camera(
            &mut commands,
            window_entity,
            layer,
            primary.is_some(),
            monitor.physical_size().as_vec2(),
            window_position,
        );
    }
}

/// Stores the target window position for camera initialization.
/// This is used because WINIT_WINDOWS is not reliably accessible from ECS systems in Bevy 0.18.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct CameraWindowPosition(Vec2);

fn spawn_camera(
    commands: &mut Commands,
    window_entity: Entity,
    camera_layer: usize,
    is_primary: bool,
    _window_size: Vec2,
    window_position: Vec2,
) {
    let mut cmd = commands.spawn((
        UninitializedCamera,
        CameraWindowPosition(window_position),
        Name::new(format!("Camera({camera_layer})")),
        RenderLayers::layer(camera_layer),
        Camera3d::default(),
        Camera {
            order: CameraOrders::DEFAULT,
            ..default()
        },
        RenderTarget::Window(WindowRef::Entity(window_entity)),
        // Projection::Perspective(PerspectiveProjection {
        //     fov: 0.1,
        //     ..default()
        // }),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 16. / 9.,
            },
            scale: 2.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 20.0),
    ));
    if is_primary {
        cmd.try_insert(PrimaryCamera);
    }
}

fn initialize_camera_position(
    mut commands: Commands,
    cameras: Query<
        (
            Entity,
            &Camera,
            &CameraWindowPosition,
            &Transform,
            &GlobalTransform,
        ),
        With<UninitializedCamera>,
    >,
) {
    for (entity, camera, window_pos, camera_tf, gtf) in cameras.iter() {
        let Some(viewport_size) = camera.logical_viewport_size() else {
            // Camera viewport not yet ready, will retry next frame
            continue;
        };
        let center = viewport_size / 2.;
        let Ok(ray) = camera.viewport_to_world(gtf, center + window_pos.0) else {
            warn!(
                "Camera {:?}: viewport_to_world failed for {:?}",
                entity,
                center + window_pos.0
            );
            continue;
        };
        let plane = InfinitePlane3d::new(gtf.back());
        let distance = ray.intersect_plane(Vec3::ZERO, plane).unwrap_or(0.);
        let camera_pos = ray.get_point(distance).with_z(camera_tf.translation.z);
        commands
            .entity(entity)
            .try_insert(Transform::from_translation(camera_pos))
            .remove::<UninitializedCamera>()
            .remove::<CameraWindowPosition>();
    }
}

fn create_window(layer: usize, size: Vec2) -> Window {
    let size = size.as_uvec2();
    Window {
        transparent: true,
        has_shadow: false,
        composite_alpha_mode: if cfg!(target_os = "windows") {
            CompositeAlphaMode::PreMultiplied
        } else {
            CompositeAlphaMode::PostMultiplied
        },
        #[cfg(target_os = "macos")]
        present_mode: PresentMode::AutoVsync,
        #[cfg(target_os = "windows")]
        present_mode: PresentMode::AutoVsync,
        resizable: false,
        decorations: false,
        ime_enabled: true,
        window_level: WindowLevel::AlwaysOnTop,
        title: format!("Window({layer:?})"),
        // Weired, on Windows, it doesn't become transparent if make it the same size as the screen.
        #[cfg(target_os = "windows")]
        resolution: WindowResolution::new(size.x - 1, size.y - 1),
        #[cfg(not(target_os = "windows"))]
        resolution: WindowResolution::new(size.x, size.y),
        titlebar_shown: false,
        mode: WindowMode::Windowed,
        #[cfg(target_os = "windows")]
        skip_taskbar: true,
        ..default()
    }
}
