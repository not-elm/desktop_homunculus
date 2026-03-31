use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Represents the state of the VRM model.
#[repr(transparent)]
#[derive(Debug, Component, Eq, PartialEq, Clone, Reflect, Serialize, Deserialize, Deref)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[reflect(Component, Serialize, Deserialize)]
pub struct VrmState(pub String);

impl VrmState {
    pub const SITTING: &'static str = "sitting";
}

impl From<&str> for VrmState {
    fn from(state: &str) -> Self {
        Self(state.to_string())
    }
}

impl Default for VrmState {
    fn default() -> Self {
        Self("idle".to_string())
    }
}

/// Links a webview to a VRM entity.
/// This is pure metadata - does not affect positioning or parenting.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub struct LinkedVrm(pub Entity);

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

/// Gender identity for a VRM character.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    Male,
    Female,
    Other,
    #[default]
    Unknown,
}

/// Persona data for a VRM character.
#[derive(Component, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<u32>,
    #[serde(default)]
    pub gender: Gender,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_person_pronoun: Option<String>,
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
            .register_type::<VrmState>()
            .register_type::<LinkedVrm>()
            .register_type::<AppWindow>();
    }
}
