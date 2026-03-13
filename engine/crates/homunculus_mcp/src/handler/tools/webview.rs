//! Webview tool implementations for the MCP handler.

use bevy::math::Vec2;
use bevy::prelude::Entity;
use homunculus_core::prelude::{WebviewOffset, WebviewOpenOptions, WebviewSource};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};

use super::super::HomunculusMcpHandler;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Default webview panel width in world units.
const DEFAULT_SIZE_X: f32 = 0.7;
/// Default webview panel height in world units.
const DEFAULT_SIZE_Y: f32 = 0.5;
/// Default internal browser width in pixels.
const DEFAULT_VIEWPORT_WIDTH: u32 = 800;
/// Default internal browser height in pixels.
const DEFAULT_VIEWPORT_HEIGHT: u32 = 600;
/// Default horizontal offset from character center.
const DEFAULT_OFFSET_X: f32 = 0.0;
/// Default vertical offset from character center (positive = above).
const DEFAULT_OFFSET_Y: f32 = 0.5;

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

/// Parameters for the `open_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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
    /// Horizontal offset from character center.
    pub offset_x: Option<f32>,
    /// Vertical offset from character center (positive = above).
    pub offset_y: Option<f32>,
}

/// Parameters for the `close_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CloseWebviewParams {
    /// Entity ID of the webview to close. If omitted, closes the most recently opened.
    pub entity: Option<u64>,
    /// Close all open webviews (default: false).
    pub all: Option<bool>,
}

/// Parameters for the `navigate_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve exactly one of `html`, `url`, or `asset_id` into a [`WebviewSource`].
fn resolve_source(
    html: Option<String>,
    url: Option<String>,
    asset_id: Option<String>,
) -> Result<WebviewSource, String> {
    match (html, url, asset_id) {
        (Some(html), None, None) => Ok(WebviewSource::Html { content: html }),
        (None, Some(url), None) => Ok(WebviewSource::Url { url }),
        (None, None, Some(id)) => Ok(WebviewSource::Local { id: id.into() }),
        (None, None, None) => Err("One of 'html', 'url', or 'asset_id' must be provided.".into()),
        _ => Err("Only one of 'html', 'url', or 'asset_id' may be provided.".into()),
    }
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = webview_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Open a webview panel displaying HTML content, a URL, or a local mod asset near the active character.
    #[tool(
        name = "open_webview",
        description = "Open a webview panel near the active character. Provide exactly one of: 'html' (inline HTML), 'url' (a URL to load), or 'asset_id' (a local mod asset, e.g. 'mod-name:asset-id'). Returns the webview entity ID. Use close_webview to close it."
    )]
    async fn open_webview(&self, params: Parameters<OpenWebviewParams>) -> String {
        let args = params.0;

        let source = match resolve_source(args.html, args.url, args.asset_id) {
            Ok(s) => s,
            Err(e) => return format!("Error: {e}"),
        };

        let size_x = args.size_x.unwrap_or(DEFAULT_SIZE_X);
        let size_y = args.size_y.unwrap_or(DEFAULT_SIZE_Y);
        let viewport_width = args.viewport_width.unwrap_or(DEFAULT_VIEWPORT_WIDTH);
        let viewport_height = args.viewport_height.unwrap_or(DEFAULT_VIEWPORT_HEIGHT);
        let offset_x = args.offset_x.unwrap_or(DEFAULT_OFFSET_X);
        let offset_y = args.offset_y.unwrap_or(DEFAULT_OFFSET_Y);

        let options = WebviewOpenOptions {
            source,
            size: Some(Vec2::new(size_x, size_y)),
            viewport_size: Some(Vec2::new(viewport_width as f32, viewport_height as f32)),
            offset: Some(WebviewOffset(Vec2::new(offset_x, offset_y))),
            linked_vrm: None,
        };

        match self.webview_api.open(options).await {
            Ok(entity) => {
                let entity_id = entity.to_bits();
                if let Ok(mut webviews) = self.open_webviews.lock() {
                    webviews.push(entity_id);
                }
                format!("Opened webview (entity {entity_id})")
            }
            Err(e) => format!("Error opening webview: {e}"),
        }
    }

    /// Close a webview panel.
    #[tool(
        name = "close_webview",
        description = "Close a webview panel. If no entity ID is given, closes the most recently opened webview. Use all=true to close all webviews."
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
        description = "Navigate an existing webview to new content without closing and reopening it. Provide exactly one of: 'html' (inline HTML), 'url' (a URL to load), or 'asset_id' (a local mod asset, e.g. 'mod-name:asset-id'). If no entity is specified, navigates the most recently opened webview."
    )]
    async fn navigate_webview(&self, params: Parameters<NavigateWebviewParams>) -> String {
        let args = params.0;

        let source = match resolve_source(args.html, args.url, args.asset_id) {
            Ok(s) => s,
            Err(e) => return format!("Error: {e}"),
        };

        let target_entity_id = match self.extract_webview(args.entity) {
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

        if let Ok(mut tracked) = self.open_webviews.lock() {
            tracked.clear();
        }

        if failures > 0 {
            format!("Closed {} webview(s), {failures} failed.", total - failures)
        } else {
            format!("Closed {total} webview(s).")
        }
    }

    async fn close_single(&self, entity: Option<u64>) -> String {
        let target_entity_id = match self.extract_webview(entity) {
            Some(id) => id,
            None => {
                return "No webviews tracked".to_string();
            }
        };

        let entity = Entity::from_bits(target_entity_id);
        match self.webview_api.close(entity).await {
            Ok(()) => {
                if let Ok(mut tracked) = self.open_webviews.lock() {
                    tracked.retain(|&id| id != target_entity_id);
                }
                format!("Closed webview (entity {target_entity_id}).")
            }
            Err(e) => format!("Error closing webview: {e}"),
        }
    }

    fn extract_webview(&self, entity: Option<u64>) -> Option<u64> {
        if let Some(id) = entity {
            Some(id)
        } else {
            self.open_webviews
                .lock()
                .ok()
                .and_then(|v| v.last().copied())
        }
    }
}
