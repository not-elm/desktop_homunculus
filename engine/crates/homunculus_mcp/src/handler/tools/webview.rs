//! Webview tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Entity;
use homunculus_core::prelude::{
    TransformArgs, WebviewConstraints, WebviewOpenOptions, WebviewSource,
};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};

/// Default webview panel width in world units.
const DEFAULT_SIZE_X: f32 = 0.7;
/// Default webview panel height in world units.
const DEFAULT_SIZE_Y: f32 = 0.5;
/// Default internal browser width in pixels.
const DEFAULT_VIEWPORT_WIDTH: u32 = 800;
/// Default internal browser height in pixels.
const DEFAULT_VIEWPORT_HEIGHT: u32 = 600;
/// Default horizontal translation from character center.
const DEFAULT_TRANSLATION_X: f32 = 0.0;
/// Default vertical translation from character center (positive = above).
const DEFAULT_TRANSLATION_Y: f32 = 1.5;
/// Default depth translation (z-offset from character).
const DEFAULT_TRANSLATION_Z: f32 = 10.0;

/// Parameters for the `open_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenWebviewParams {
    /// Inline HTML content to display (mutually exclusive with url and asset_id).
    pub html: Option<String>,
    /// URL to load (mutually exclusive with html and asset_id).
    pub url: Option<String>,
    /// Local mod asset ID to load, e.g. "mod-name:asset-id" (mutually exclusive with html and url).
    pub asset_id: Option<String>,
    /// Panel width in world units.
    pub size_x: Option<f32>,
    /// Panel height in world units.
    pub size_y: Option<f32>,
    /// Internal browser width in pixels.
    pub viewport_width: Option<u32>,
    /// Internal browser height in pixels.
    pub viewport_height: Option<u32>,
    /// Horizontal translation from character center.
    pub translation_x: Option<f32>,
    /// Vertical translation from character center (positive = above).
    pub translation_y: Option<f32>,
    /// Depth translation (z-offset). Default: 10.0.
    pub translation_z: Option<f32>,
    /// Name of the character to link this webview to.
    /// When linked, the webview follows the character's head position.
    /// Use get_character_snapshot to see available character names.
    pub character_name: Option<String>,
    /// How much rotation to inherit from parent (0.0 = billboard, 1.0 = full). Default: 0.0.
    pub rotation_follow: Option<f32>,
    /// Maximum tilt angle from upright in degrees. Default: 0.0.
    pub max_tilt_degrees: Option<f32>,
    /// Lock scale at 1.0 regardless of parent scale. Default: true.
    pub lock_scale: Option<bool>,
}

/// Parameters for the `close_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloseWebviewParams {
    /// Entity ID of the webview to close. If omitted, closes the most recently opened.
    pub entity: Option<u64>,
    /// Close all open webviews (default: false).
    pub all: Option<bool>,
}

/// Parameters for the `navigate_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NavigateWebviewParams {
    /// Entity ID of the webview to navigate. If omitted, navigates the most recently opened.
    pub entity: Option<u64>,
    /// New inline HTML content to display (mutually exclusive with url and asset_id).
    pub html: Option<String>,
    /// URL to navigate to (mutually exclusive with html and asset_id).
    pub url: Option<String>,
    /// Local mod asset ID to navigate to, e.g. "mod-name:asset-id" (mutually exclusive with html and url).
    pub asset_id: Option<String>,
}

/// Resolve exactly one of `html`, `url`, or `asset_id` into a [`WebviewSource`].
fn resolve_source(
    html: Option<String>,
    url: Option<String>,
    asset_id: Option<String>,
) -> Result<WebviewSource, String> {
    match (html, url, asset_id) {
        (Some(html), None, None) => Ok(WebviewSource::Html { content: html }),
        (None, Some(url), None) => {
            let lower = url.to_lowercase();
            if !lower.starts_with("http://") && !lower.starts_with("https://") {
                return Err("Only http:// and https:// URLs are allowed.".to_string());
            }
            Ok(WebviewSource::Url { url })
        }
        (None, None, Some(id)) => Ok(WebviewSource::Local { id: id.into() }),
        (None, None, None) => Err("One of 'html', 'url', or 'asset_id' must be provided.".into()),
        _ => Err("Only one of 'html', 'url', or 'asset_id' may be provided.".into()),
    }
}

#[rmcp::tool_router(router = webview_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Open a webview panel displaying HTML content, a URL, or a local mod asset near the active character.
    #[tool(
        name = "open_webview",
        description = "Open a webview panel. Provide exactly one of: 'html' (inline HTML), 'url' (a URL to load), or 'asset_id' (a local mod asset, e.g. 'mod-name:asset-id'). Position with 'translationX', 'translationY', 'translationZ'. Optionally provide 'characterName' to link the webview to a specific character (it will follow the character's head position). Returns the webview entity ID. Use close_webview to close it.",
        annotations(destructive_hint = false)
    )]
    async fn open_webview(&self, params: Parameters<OpenWebviewParams>) -> String {
        let args = params.0;

        let source = match resolve_source(args.html, args.url, args.asset_id) {
            Ok(s) => s,
            Err(e) => return format!("Error: {e}"),
        };

        let linked_persona = if let Some(name) = &args.character_name {
            match self.resolve_persona_by_name(name).await {
                Ok(persona) => Some(persona.id),
                Err(e) => return format!("Error: {e}"),
            }
        } else {
            None
        };

        let size_x = args.size_x.unwrap_or(DEFAULT_SIZE_X);
        let size_y = args.size_y.unwrap_or(DEFAULT_SIZE_Y);
        let viewport_width = args.viewport_width.unwrap_or(DEFAULT_VIEWPORT_WIDTH);
        let viewport_height = args.viewport_height.unwrap_or(DEFAULT_VIEWPORT_HEIGHT);
        let translation_x = args.translation_x.unwrap_or(DEFAULT_TRANSLATION_X);
        let translation_y = args.translation_y.unwrap_or(DEFAULT_TRANSLATION_Y);
        let translation_z = args.translation_z.unwrap_or(DEFAULT_TRANSLATION_Z);

        let options = WebviewOpenOptions {
            source,
            size: Some(Vec2::new(size_x, size_y)),
            viewport_size: Some(Vec2::new(viewport_width as f32, viewport_height as f32)),
            transform: Some(TransformArgs {
                translation: Some(Vec3::new(translation_x, translation_y, translation_z)),
                rotation: None,
                scale: None,
            }),
            constraints: if args.rotation_follow.is_some()
                || args.max_tilt_degrees.is_some()
                || args.lock_scale.is_some()
            {
                Some(WebviewConstraints {
                    rotation_follow: args.rotation_follow,
                    max_tilt_degrees: args.max_tilt_degrees,
                    lock_scale: args.lock_scale,
                })
            } else {
                None
            },
            linked_persona,
            resizable: None,
        };

        match self.webview_api.open(options).await {
            Ok(entity) => {
                let entity_id = entity.to_bits();
                self.open_webviews
                    .lock()
                    .unwrap_or_else(|e| {
                        bevy::log::warn!("Mutex poisoned: {e}");
                        e.into_inner()
                    })
                    .push(entity_id);
                format!("Opened webview (entity {entity_id})")
            }
            Err(e) => format!("Error opening webview: {e}"),
        }
    }

    /// Close a webview panel.
    #[tool(
        name = "close_webview",
        description = "Close a webview panel. If no entity ID is given, closes the most recently opened webview. Use all=true to close all webviews.",
        annotations(idempotent_hint = true, open_world_hint = false)
    )]
    async fn close_webview(&self, params: Parameters<CloseWebviewParams>) -> String {
        let args = params.0;
        let close_all = args.all.unwrap_or(false);

        if close_all {
            self.close_all().await
        } else {
            self.close_single(args.entity).await
        }
    }

    /// Navigate an existing webview to new content.
    #[tool(
        name = "navigate_webview",
        description = "Navigate an existing webview to new content without closing and reopening it. Provide exactly one of: 'html' (inline HTML), 'url' (a URL to load), or 'asset_id' (a local mod asset, e.g. 'mod-name:asset-id'). If no entity is specified, navigates the most recently opened webview.",
        annotations(destructive_hint = false, idempotent_hint = true)
    )]
    async fn navigate_webview(&self, params: Parameters<NavigateWebviewParams>) -> String {
        let args = params.0;

        let source = match resolve_source(args.html, args.url, args.asset_id) {
            Ok(s) => s,
            Err(e) => return format!("Error: {e}"),
        };

        let target_entity_id = match self.resolve_webview_entity(args.entity) {
            Some(id) => id,
            None => {
                return "No webviews tracked. Open a webview first with open_webview.".to_string();
            }
        };

        let entity = Entity::from_bits(target_entity_id);

        match self.webview_api.navigate(entity, source).await {
            Ok(()) => format!("Navigated webview (entity {target_entity_id}) to new content."),
            Err(e) => format!("Error navigating webview: {e}"),
        }
    }

    async fn close_all(&self) -> String {
        let webviews = match self.webview_api.list().await {
            Ok(list) => list,
            Err(e) => return format!("Error listing webviews: {e}"),
        };

        if webviews.is_empty() {
            return "No webviews are open.".to_string();
        }

        let total = webviews.len();
        let mut failures = 0;
        for info in &webviews {
            if self.webview_api.close(info.entity).await.is_err() {
                failures += 1;
            }
        }

        self.open_webviews
            .lock()
            .unwrap_or_else(|e| {
                bevy::log::warn!("Mutex poisoned: {e}");
                e.into_inner()
            })
            .clear();

        if failures > 0 {
            format!("Closed {} webview(s), {failures} failed.", total - failures)
        } else {
            format!("Closed {total} webview(s).")
        }
    }

    async fn close_single(&self, entity: Option<u64>) -> String {
        let target_entity_id = match self.resolve_webview_entity(entity) {
            Some(id) => id,
            None => {
                return "No webviews tracked".to_string();
            }
        };

        let entity = Entity::from_bits(target_entity_id);
        match self.webview_api.close(entity).await {
            Ok(()) => {
                self.open_webviews
                    .lock()
                    .unwrap_or_else(|e| {
                        bevy::log::warn!("Mutex poisoned: {e}");
                        e.into_inner()
                    })
                    .retain(|&id| id != target_entity_id);
                format!("Closed webview (entity {target_entity_id}).")
            }
            Err(e) => format!("Error closing webview: {e}"),
        }
    }

    fn resolve_webview_entity(&self, entity: Option<u64>) -> Option<u64> {
        if let Some(id) = entity {
            Some(id)
        } else {
            self.open_webviews
                .lock()
                .unwrap_or_else(|e| {
                    bevy::log::warn!("Mutex poisoned: {e}");
                    e.into_inner()
                })
                .last()
                .copied()
        }
    }
}
