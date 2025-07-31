//! # Homunculus Shadow Panel
//!
//! This crate provides shadow rendering functionality for VRM mascot models in the
//! Desktop Homunculus application. The shadow panel creates realistic shadows
//! beneath characters using custom shaders and directional lighting.
//!
//! ## Overview
//!
//! The shadow panel system places a large transparent plane behind VRM avatars
//! and applies a specialized shader that projects only the shadows of the models.
//! This creates the illusion that the mascots are casting shadows on the desktop
//! surface, enhancing visual realism and grounding.
//!
//! ## Key Features
//!
//! - **Custom Shadow Shader**: Specialized WGSL shader for shadow-only rendering
//! - **Directional Lighting**: Configurable directional light for shadow casting
//! - **Alpha Blending**: Smooth shadow transparency with configurable alpha values
//! - **Large Shadow Plane**: Invisible plane that catches shadows from VRM models
//! - **Render Layer Integration**: Proper integration with the VRM rendering pipeline
//!
//! ## How It Works
//!
//! 1. **Directional Light**: A directional light is spawned to cast shadows from VRM models
//! 2. **Shadow Plane**: A large invisible plane is positioned behind the models
//! 3. **Shadow Material**: A custom material with shadow-only shader is applied
//! 4. **Shadow Projection**: The shader renders only the shadow portions of the scene
//!
//! ## Shadow Material
//!
//! The [`ShadowPanelMaterial`] includes:
//! - `alpha_factor`: Controls the opacity/transparency of shadows
//! - Custom fragment shader for shadow-only rendering
//! - Alpha masking for smooth shadow edges
//!
//! ## Rendering Pipeline
//!
//! The shadow system integrates with Bevy's standard rendering pipeline and uses:
//! - Custom WGSL shader for shadow projection
//! - Material plugin for proper render integration
//! - Directional light shadows for realistic shadow casting
//! - Alpha blending for smooth transparency effects

use bevy::app::{App, Plugin};
use bevy::asset::{Asset, load_internal_asset, weak_handle};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_vrm1::prelude::Cameras;
use homunculus_core::prelude::ShadowPanel;

const SHADOW_PANEL_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("7c1ae0c0-108e-4178-966d-2cd1229ffa0a");

#[derive(SystemSet, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ShadowPanelSetup;

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
pub struct ShadowPanelMaterial {
    #[uniform(100)]
    pub alpha_factor: f32,
}

impl Material for ShadowPanelMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADOW_PANEL_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Mask(0.3)
    }
}

/// Plugin that provides shadow rendering functionality for VRM mascot models.
///
/// This plugin sets up a shadow system that creates realistic shadows beneath
/// VRM characters using custom shaders and directional lighting. The shadows
/// are rendered on a large invisible plane positioned behind the models.
///
/// # Components
///
/// The plugin automatically sets up:
/// - **Custom Shadow Shader**: Loads the internal shadow_panel.wgsl shader
/// - **Shadow Material**: Registers the ShadowPanelMaterial for rendering
/// - **Directional Light**: Creates a directional light for shadow casting
/// - **Shadow Plane**: Spawns a large plane to catch and display shadows
///
/// # Systems
///
/// - `spawn_directional_light`: Creates the directional light at startup
/// - `spawn_shadow_panel`: Creates the shadow plane with shadow material
///
/// Both systems run in the `ShadowPanelSetup` system set during startup.
///
/// # Shader Integration
///
/// The plugin loads an internal WGSL shader that handles shadow-only rendering,
/// ensuring that only the shadow portions of the scene are visible on the
/// shadow panel plane while maintaining proper alpha blending.
pub struct HomunculusShadowPanelPlugin;

impl Plugin for HomunculusShadowPanelPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SHADOW_PANEL_SHADER_HANDLE,
            "shadow_panel.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(MaterialPlugin::<ShadowPanelMaterial>::default())
            .add_systems(
                Startup,
                (spawn_directional_light, spawn_shadow_panel).in_set(ShadowPanelSetup),
            );
    }
}

fn spawn_directional_light(mut commands: Commands, cameras: Cameras) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // Transform::from_rotation(Quat::from_array([-0.1, 0.1, 0., 1.])),
        Transform::from_rotation(Quat::from_array([-0.01, 0.01, 0., 1.])),
        cameras.all_layers(),
    ));
}

fn spawn_shadow_panel(
    mut commands: Commands,
    mut materials: ResMut<Assets<ShadowPanelMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    cameras: Cameras,
) {
    commands.spawn((
        Pickable::IGNORE,
        Name::new("ShadowPanel"),
        ShadowPanel,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::ONE * 1000.))),
        MeshMaterial3d(materials.add(ShadowPanelMaterial { alpha_factor: 0.5 })),
        Transform::from_xyz(0., 0., -5.),
        cameras.all_layers(),
    ));
}
