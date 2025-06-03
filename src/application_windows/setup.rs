use crate::application_windows::{PrimaryCamera, TargetMonitor};
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode, Viewport};
use bevy::render::render_resource::{
    AsBindGroup, ShaderRef,
};
use bevy::render::view::RenderLayers;
use bevy::window::{
    CursorOptions, Monitor, PrimaryMonitor, PrimaryWindow, WindowLevel, WindowMode, WindowRef,
    WindowResolution,
};
use bevy::winit::WinitWindows;
use bevy_vrm1::system_param::cameras::Cameras;
use serde::{Deserialize, Serialize};

pub struct ApplicationWindowsSetupPlugin;

impl Plugin for ApplicationWindowsSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, WallMaterial>,
        >::default())
            .register_type::<UninitializedCamera>()
            .register_type::<DefaultPrimaryWindow>()
            .register_type::<TargetMonitor>()
            .add_systems(
                PreStartup,
                (mark_default_window, setup_windows, despawn_default_window).chain(),
            )
            .add_systems(Startup, (spawn_directional_light, spawn_wall))
            .add_systems(Update, initialize_camera_position);
    }
}

fn spawn_directional_light(mut commands: Commands, cameras: Cameras) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_array([-0.1, 0.1, 0., 1.])),
        cameras.all_layers(),
    ));
}

fn spawn_wall(
    mut commands: Commands,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WallMaterial>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    cameras: Cameras,
) {
    commands.spawn((
        Name::new("Wall"),
        Mesh3d(meshes.add(Cuboid::new(1000., 1000., 1.))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                unlit: false,
                alpha_mode: AlphaMode::Mask(0.3),
                base_color: Color::NONE,
                ..default()
            },
            extension: WallMaterial {},
        })),
        Transform::from_xyz(0., 0., -1.),
        cameras.all_layers(),
    ));
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
    commands
        .entity(default_window.single().unwrap())
        .insert(DefaultPrimaryWindow);
}

fn despawn_default_window(
    mut commands: Commands,
    default_window: Query<Entity, With<DefaultPrimaryWindow>>,
) {
    commands.entity(default_window.single().unwrap()).despawn();
}

fn setup_windows(
    mut commands: Commands,
    monitors: Query<(Entity, &Monitor, Option<&PrimaryMonitor>)>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    let current_monitor_scale_factor = winit_windows
        .get_window(primary_window.single().unwrap())
        .unwrap()
        .current_monitor()
        .map(|monitor| monitor.scale_factor() as f32)
        .unwrap_or(1.);
    for (layer, (monitor_entity, monitor, primary)) in monitors.iter().enumerate() {
        let mut window = create_window(monitor.physical_size().as_vec2());
        window
            .position
            .set((monitor.physical_position.as_vec2() * current_monitor_scale_factor).as_ivec2());
        window
            .resolution
            .set_scale_factor(monitor.scale_factor as f32);
        let window_entity = commands
            .spawn((
                Name::new(format!("Window({:?})", monitor.physical_position)),
                TargetMonitor(monitor_entity),
                RenderLayers::layer(layer),
                window,
            ))
            .id();
        commands
            .entity(monitor_entity)
            .insert(RenderLayers::layer(layer));

        spawn_camera(
            &mut commands,
            window_entity,
            monitor,
            layer,
            primary.is_some(),
        );

        if primary.is_some() {
            commands.entity(window_entity).insert(PrimaryWindow);
        }
    }
}

fn spawn_camera(
    commands: &mut Commands,
    window_entity: Entity,
    monitor: &Monitor,
    camera_layer: usize,
    is_primary: bool,
) {
    let mut cmd = commands.spawn((
        UninitializedCamera,
        Name::new(format!("Camera({camera_layer})")),
        RenderLayers::layer(camera_layer),
        Camera3d::default(),
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(window_entity)),
            viewport: Some(Viewport {
                physical_size: monitor.physical_size(),
                ..default()
            }),
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 3.,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 4.5),
    ));
    if is_primary {
        cmd.insert(PrimaryCamera);
    }
}

fn initialize_camera_position(
    mut commands: Commands,
    cameras: Query<(Entity, &Camera, &GlobalTransform), With<UninitializedCamera>>,
    winit_window: NonSend<WinitWindows>,
) {
    cameras.iter().for_each(|(entity, camera, gtf)| {
        let RenderTarget::Window(WindowRef::Entity(window_entity)) = camera.target else {
            return;
        };
        let Some(window) = winit_window.get_window(window_entity) else {
            return;
        };
        let pos = window.outer_position().unwrap().cast();
        let pos = Vec2::new(pos.x, pos.y);
        let center = camera.logical_viewport_size().unwrap() / 2.;
        let camera_pos = camera
            .viewport_to_world_2d(gtf, center + pos)
            .unwrap_or_default()
            .extend(4.5);

        commands
            .entity(entity)
            .insert(Transform::from_translation(camera_pos))
            .remove::<UninitializedCamera>();
    });
}

fn create_window(size: Vec2) -> Window {
    Window {
        transparent: true,
        has_shadow: false,
        #[cfg(target_os = "macos")]
        composite_alpha_mode: bevy::window::CompositeAlphaMode::PostMultiplied,
        resizable: false,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        // Weired, on Windows, it doesn't become transparent if make it the same size as the screen.
        resolution: WindowResolution::new(size.x - 1., size.y - 1.),
        titlebar_shown: false,
        mode: WindowMode::Windowed,
        cursor_options: CursorOptions {
            #[cfg(not(feature = "develop"))]
            hit_test: false,
            ..default()
        },
        ..default()
    }
}

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
struct WallMaterial {}

impl MaterialExtension for WallMaterial {
    fn fragment_shader() -> ShaderRef {
        "shadow.wgsl".into()
    }

    fn alpha_mode() -> Option<AlphaMode> {
        Some(AlphaMode::Mask(0.3))
    }
}
