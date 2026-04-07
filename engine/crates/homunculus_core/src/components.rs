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

/// URL-safe unique identifier for a persona.
/// Validation: non-empty, `[a-zA-Z0-9_-]`, max 64 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PersonaId(pub String);

impl PersonaId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn validate(id: &str) -> Result<Self, String> {
        if id.is_empty() {
            return Err("PersonaId cannot be empty".to_string());
        }
        if id.len() > 64 {
            return Err(format!("PersonaId exceeds 64 characters: {}", id.len()));
        }
        if !id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(format!("PersonaId contains invalid characters: {id}"));
        }
        Ok(Self(id.to_string()))
    }
}

impl std::fmt::Display for PersonaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for PersonaId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for PersonaId {
    fn default() -> Self {
        Self(String::new())
    }
}

/// Represents the state of a persona (character).
#[repr(transparent)]
#[derive(Debug, Component, Eq, PartialEq, Clone, Reflect, Serialize, Deserialize, Deref)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[reflect(Component, Serialize, Deserialize)]
pub struct PersonaState(pub String);

impl PersonaState {
    pub const SITTING: &'static str = "sitting";
}

impl Default for PersonaState {
    fn default() -> Self {
        Self("idle".to_string())
    }
}

impl<S: Into<String>> From<S> for PersonaState {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

/// Links a webview to a persona by PersonaId (stable across entity recreation).
#[derive(Component, Debug, Clone)]
pub struct LinkedPersona(pub PersonaId);

/// O(1) PersonaId -> Entity lookup index.
#[derive(Resource, Default, Debug)]
pub struct PersonaIndex(pub HashMap<PersonaId, Entity>);

impl PersonaIndex {
    pub fn get(&self, id: &PersonaId) -> Option<Entity> {
        self.0.get(id).copied()
    }

    pub fn insert(&mut self, id: PersonaId, entity: Entity) {
        self.0.insert(id, entity);
    }

    pub fn remove(&mut self, id: &PersonaId) {
        self.0.remove(id);
    }
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
    pub id: PersonaId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vrm_asset_id: Option<String>,
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
            .register_type::<PersonaState>()
            .init_resource::<PersonaIndex>()
            .register_type::<AppWindow>();
    }
}
