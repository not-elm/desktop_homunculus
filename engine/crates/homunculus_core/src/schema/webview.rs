use super::asset::AssetId;
use super::transform::TransformArgs;
use super::transform_constraint::WebviewConstraints;
use crate::components::PersonaId;
use bevy::math::{UVec2, Vec2};
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
    /// Transform for positioning. translation sets the offset from parent.
    #[serde(default)]
    pub transform: Option<TransformArgs>,
    /// Transform constraint parameters for parent transform propagation.
    #[serde(default)]
    pub constraints: Option<WebviewConstraints>,
    /// Persona to link to this webview (optional).
    #[serde(default)]
    pub linked_persona: Option<PersonaId>,
    /// Enable drag-to-resize. Omit or set to null to disable.
    #[serde(default)]
    pub resizable: Option<WebviewResizableOptions>,
}

/// Aspect ratio lock behavior during resize drag.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum AspectLockMode {
    /// Shift-drag locks aspect ratio.
    #[default]
    LockOnShift,
    /// Always lock aspect ratio.
    Always,
    /// Never lock aspect ratio.
    Never,
}

/// Options for enabling drag-to-resize on a webview.
/// All fields are optional; omitted fields use bevy_cef defaults.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct WebviewResizableOptions {
    /// Width of the invisible resize border in texture pixels. Default: 16.
    #[serde(default = "default_edge_thickness")]
    pub edge_thickness: u32,
    /// Minimum texture size in pixels. Default: [100, 100].
    #[serde(default = "default_min_size")]
    #[cfg_attr(feature = "openapi", schema(value_type = [u32; 2]))]
    pub min_size: UVec2,
    /// Maximum texture size in pixels. None = no limit.
    #[serde(default)]
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[u32; 2]>))]
    pub max_size: Option<UVec2>,
    /// Aspect ratio lock mode. Default: "lockOnShift".
    #[serde(default)]
    pub aspect_lock: AspectLockMode,
}

fn default_edge_thickness() -> u32 {
    16
}

fn default_min_size() -> UVec2 {
    UVec2::splat(100)
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
    /// Current transform (translation = intended offset from parent).
    pub transform: TransformArgs,
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
    /// Transform update (translation updates the offset from parent).
    #[serde(default)]
    pub transform: Option<TransformArgs>,
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

/// Navigation history state of a webview.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct NavigationState {
    /// Whether the webview can navigate back in history.
    pub can_go_back: bool,
    /// Whether the webview can navigate forward in history.
    pub can_go_forward: bool,
}
