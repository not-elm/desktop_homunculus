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
//! 1. **Default Window Handling**: Marks and later removes Bevy's default window
//! 2. **Monitor Detection**: Discovers all available monitors and their properties
//! 3. **Window Creation**: Creates transparent windows sized and positioned for each monitor
//! 4. **Camera Setup**: Spawns orthographic cameras for each window
//! 5. **Position Calculation**: Aligns cameras with desktop coordinate systems

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::view::RenderLayers;
use bevy::window::*;
use bevy::winit::WinitWindows;
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
/// - `mark_default_window`: Identifies Bevy's default window for removal
/// - `setup_windows`: Creates windows and cameras for each monitor
/// - `despawn_default_window`: Removes the default window after setup
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
            .register_type::<DefaultPrimaryWindow>()
            .add_systems(
                PreStartup,
                (mark_default_window, setup_windows, despawn_default_window).chain(),
            )
            .add_systems(Update, initialize_camera_position);
    }
}

#[derive(Component, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
struct UninitializedCamera;

/// The default primary window is only used to adjust the position of the window, and
/// it is despawn after all windows are created.
#[derive(Component, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
struct DefaultPrimaryWindow;

fn mark_default_window(mut commands: Commands, default_window: Query<Entity, With<PrimaryWindow>>) {
    if let Ok(window) = default_window.single() {
        commands.entity(window).insert(DefaultPrimaryWindow);
    }
}

fn despawn_default_window(
    mut commands: Commands,
    default_window: Query<Entity, With<DefaultPrimaryWindow>>,
) {
    if let Ok(window) = default_window.single() {
        commands.entity(window).despawn();
    }
}

fn setup_windows(
    mut commands: Commands,
    monitors: Query<(Entity, &Monitor, Option<&PrimaryMonitor>)>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    let current_monitor_scale_factor = primary_window
        .single()
        .ok()
        .and_then(|entity| winit_windows.get_window(entity))
        .and_then(|w| w.current_monitor())
        .map(|monitor| monitor.scale_factor() as f32)
        .unwrap_or(1.);
    for (layer, (monitor_entity, monitor, primary)) in monitors.iter().enumerate() {
        let mut window = create_window(layer, monitor.physical_size().as_vec2());
        window
            .position
            .set((monitor.physical_position.as_vec2() * current_monitor_scale_factor).as_ivec2());
        window
            .resolution
            .set_scale_factor(monitor.scale_factor as f32);
        let window_entity = commands
            .spawn((
                Name::new(format!("Window({:?})", monitor.physical_position)),
                RenderLayers::layer(layer),
                window,
                AppWindow,
            ))
            .id();
        commands
            .entity(monitor_entity)
            .insert(RenderLayers::layer(layer));

        spawn_camera(
            &mut commands,
            window_entity,
            layer,
            primary.is_some(),
            monitor.physical_size().as_vec2(),
        );

        if primary.is_some() {
            commands.entity(window_entity).insert(PrimaryWindow);
        }
    }
}

fn spawn_camera(
    commands: &mut Commands,
    window_entity: Entity,
    camera_layer: usize,
    is_primary: bool,
    window_size: Vec2,
) {
    let mut cmd = commands.spawn((
        UninitializedCamera,
        Name::new(format!("Camera({camera_layer})")),
        RenderLayers::layer(camera_layer),
        Camera3d::default(),
        Camera {
            order: CameraOrders::DEFAULT,
            target: RenderTarget::Window(WindowRef::Entity(window_entity)),
            ..default()
        },
        // Projection::Perspective(PerspectiveProjection {
        //     fov: 0.1,
        //     ..default()
        // }),
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: window_size.x / window_size.y,
            },
            scale: 1.5,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 10.0),
    ));
    if is_primary {
        cmd.insert(PrimaryCamera);
    }
}

fn initialize_camera_position(
    mut commands: Commands,
    cameras: Query<(Entity, &Camera, &Transform, &GlobalTransform), With<UninitializedCamera>>,
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
            .remove::<UninitializedCamera>();
    });
}

fn create_window(layer: usize, size: Vec2) -> Window {
    Window {
        transparent: true,
        has_shadow: false,
        #[cfg(target_os = "macos")]
        composite_alpha_mode: bevy::window::CompositeAlphaMode::PostMultiplied,
        #[cfg(target_os = "macos")]
        present_mode: PresentMode::AutoVsync,
        resizable: false,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        title: format!("Window({layer:?})"),
        // Weired, on Windows, it doesn't become transparent if make it the same size as the screen.
        #[cfg(target_os = "windows")]
        resolution: WindowResolution::new(size.x - 1., size.y - 1.),
        #[cfg(not(target_os = "windows"))]
        resolution: WindowResolution::new(size.x, size.y),
        titlebar_shown: false,
        mode: WindowMode::Windowed,
        cursor_options: CursorOptions {
            hit_test: false,
            ..default()
        },
        ..default()
    }
}
