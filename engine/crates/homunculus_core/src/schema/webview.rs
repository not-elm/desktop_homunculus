use super::asset::AssetId;
use super::transform_constraint::WebviewConstraints;
use crate::components::PersonaId;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Component, Entity, Reflect};
use serde::{Deserialize, Serialize};

/// Webview source specification (request).
/// Either a URL/module path or inline HTML content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WebviewSource {
    /// URL or module path (mods://, asset://, https://, etc.)
    Url { url: String },
    /// Raw HTML content
    Html { content: String },
    /// Local HTML asset by registry ID
    Local { id: AssetId },
}

/// Webview source information (response).
/// In list responses (GET /webviews), Html content is omitted (None).
/// In detail responses (GET /webviews/{id}), Html content is included.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WebviewSourceInfo {
    Url {
        url: String,
    },
    Html {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
    },
    Local {
        id: AssetId,
    },
}

/// Request body for POST /webviews
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct WebviewOpenOptions {
    /// The source of the webview (URL, local path, or inline HTML).
    pub source: WebviewSource,
    /// Mesh size in world units. Default: [0.7, 0.7].
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 2]>))]
    pub size: Option<Vec2>,
    /// Viewport resolution in pixels. Default: [800, 800].
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 2]>))]
    pub viewport_size: Option<Vec2>,
    /// Deprecated: use `transform` instead. Maps to transform.translation with z=10.0.
    #[serde(default)]
    pub offset: Option<WebviewOffset>,
    /// Transform constraint parameters for parent transform propagation.
    #[serde(default)]
    pub constraints: Option<WebviewConstraints>,
    /// Persona to link to this webview (optional).
    #[serde(default)]
    pub linked_persona: Option<PersonaId>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Component, Copy)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "openapi", schema(value_type = [f32; 3]))]
pub struct WebviewOffset(pub Vec3);

impl Default for WebviewOffset {
    fn default() -> Self {
        Self(Vec3::new(0.0, 0.0, 10.0))
    }
}

impl<'de> Deserialize<'de> for WebviewOffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<f32> = Vec::deserialize(deserializer)?;
        match arr.len() {
            2 => Ok(WebviewOffset(Vec3::new(arr[0], arr[1], 10.0))),
            3 => Ok(WebviewOffset(Vec3::new(arr[0], arr[1], arr[2]))),
            _ => Err(serde::de::Error::custom(
                "offset must be [x, y] or [x, y, z]",
            )),
        }
    }
}

/// Tracks the mesh size of a webview in world units.
/// Inserted when a webview is created, updated when size changes.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Component, Copy, Reflect)]
pub struct WebviewMeshSize(pub Vec2);

impl Default for WebviewMeshSize {
    fn default() -> Self {
        Self(Vec2::splat(0.7))
    }
}

/// Response for GET /webviews and GET /webviews/{entity}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct WebviewInfo {
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub entity: Entity,
    pub source: WebviewSourceInfo,
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 2]))]
    pub size: Vec2,
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 2]))]
    pub viewport_size: Vec2,
    /// Deprecated: included for backward compatibility.
    pub offset: WebviewOffset,
    /// Active constraint parameters.
    pub constraints: WebviewConstraints,
    #[serde(default)]
    pub linked_persona: Option<PersonaId>,
}

/// Request for PATCH /webviews/{entity}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct WebviewPatchRequest {
    /// Deprecated: use transform via /entities/{entity}/transform instead.
    #[serde(default)]
    pub offset: Option<WebviewOffset>,
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 2]>))]
    pub size: Option<Vec2>,
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 2]>))]
    pub viewport_size: Option<Vec2>,
    /// Constraint parameters update (partial merge).
    #[serde(default)]
    pub constraints: Option<WebviewConstraints>,
}

/// Request for POST /webviews/{entity}/navigate
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct WebviewNavigateRequest {
    pub source: WebviewSource,
}
