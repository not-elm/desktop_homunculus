use crate::avatar::{AvatarName, AvatarState};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[deprecated(note = "Use AvatarState instead")]
pub type VrmState = AvatarState;

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Loading;

/// The marker component that attaches to the entity of the window used in the app.
/// Windows are generated for each monitor.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AppWindow;

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ShadowPanel;

/// Represents the global screen coordinates.
/// If there are multiple screens, the coordinates of the leftmost screen are used as the origin.
#[derive(Debug, Copy, Clone, PartialEq, Deref, Reflect, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[reflect(Serialize, Deserialize)]
#[repr(transparent)]
#[cfg_attr(feature = "openapi", schema(value_type = [f32; 2]))]
pub struct GlobalViewport(pub Vec2);

/// Cameras are spawned for each window.
/// This component is attached to the camera corresponding to the window that is the [`PrimaryWindow`](bevy::prelude::PrimaryWindow).
#[derive(Debug, Component, Eq, PartialEq, Copy, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PrimaryCamera;

/// Links a webview to an avatar by its string identifier.
///
/// The tracking system resolves the avatar ID to an ECS entity via
/// [`AvatarRegistry`](crate::avatar_registry::AvatarRegistry) each frame.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub struct LinkedAvatar(pub String);

#[deprecated(note = "Use LinkedAvatar instead")]
pub type LinkedVrm = LinkedAvatar;

/// Big Five personality traits (OCEAN model).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Ocean {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openness: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conscientiousness: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extraversion: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreeableness: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neuroticism: Option<f64>,
}

/// Persona data for a VRM character.
#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    #[serde(default)]
    pub profile: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personality: Option<String>,
    #[serde(default)]
    pub ocean: Ocean,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    #[cfg_attr(feature = "openapi", schema(value_type = std::collections::HashMap<String, Object>))]
    pub metadata: HashMap<String, serde_json::Value>,
}

pub struct CoreComponentsPlugin;

impl Plugin for CoreComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Loading>()
            .register_type::<ShadowPanel>()
            .register_type::<GlobalViewport>()
            .register_type::<PrimaryCamera>()
            .register_type::<AvatarState>()
            .register_type::<AvatarName>()
            .register_type::<LinkedAvatar>()
            .register_type::<AppWindow>();
    }
}
