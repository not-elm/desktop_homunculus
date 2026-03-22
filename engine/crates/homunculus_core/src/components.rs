use crate::character::{CharacterId, CharacterName, CharacterState};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[deprecated(note = "Use CharacterState instead")]
pub type VrmState = CharacterState;

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

/// Links a webview to a validated character ID.
///
/// The tracking system looks up the character entity via
/// [`CharacterRegistry`](crate::character_registry::CharacterRegistry) each frame.
/// Storing a pre-validated [`CharacterId`] avoids per-frame string validation.
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub struct LinkedCharacter(pub CharacterId);

#[deprecated(note = "Use LinkedCharacter instead")]
pub type LinkedVrm = LinkedCharacter;

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
}

pub struct CoreComponentsPlugin;

impl Plugin for CoreComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Loading>()
            .register_type::<ShadowPanel>()
            .register_type::<GlobalViewport>()
            .register_type::<PrimaryCamera>()
            .register_type::<CharacterState>()
            .register_type::<CharacterName>()
            .register_type::<LinkedCharacter>()
            .register_type::<AppWindow>();
    }
}
