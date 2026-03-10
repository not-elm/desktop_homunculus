//! MCP request handler implementation.
//!
//! Provides [`HomunculusMcpHandler`], the central handler that implements the
//! [`ServerHandler`] trait from `rmcp`, exposing Desktop Homunculus capabilities
//! (tools and resources) to MCP-compatible AI agents.

use std::sync::{Arc, Mutex};

use bevy::math::Vec2;
use bevy::prelude::Entity;
use homunculus_api::assets::{AssetFilter, AssetsApi};
use homunculus_api::mods::ModsApi;
use homunculus_api::prelude::{ApiReactor, AppApi, VrmApi, WebviewApi};
use homunculus_core::prelude::{WebviewOffset, WebviewOpenOptions, WebviewSource};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    AnnotateAble, Implementation, ListResourcesResult, PaginatedRequestParams, RawResource,
    ReadResourceRequestParams, ReadResourceResult, Resource, ResourceContents,
    ServerCapabilities, ServerInfo,
};
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler, tool, tool_handler, tool_router};
use serde::{Deserialize, Serialize};

const SERVER_NAME: &str = "homunculus";

const FEATURES: &[&str] = &[
    "vrm", "audio", "webviews", "effects", "speech", "signals", "mods",
];

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

/// Static resource definitions returned by `list_resources`.
fn resource_definitions() -> Vec<Resource> {
    vec![
        RawResource::new("homunculus://info", "homunculus-info")
            .with_description("Application info including version, platform, features, and mods")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://characters", "homunculus-characters")
            .with_description("Detailed snapshot of all loaded VRM characters")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://mods", "homunculus-mods")
            .with_description("List of installed mods")
            .with_mime_type("application/json")
            .no_annotation(),
        RawResource::new("homunculus://assets", "homunculus-assets")
            .with_description("List of available assets across all mods")
            .with_mime_type("application/json")
            .no_annotation(),
    ]
}

// ---------------------------------------------------------------------------
// Tool parameter structs
// ---------------------------------------------------------------------------

/// Parameters for the `open_webview` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OpenWebviewParams {
    /// Inline HTML content to display (mutually exclusive with url).
    pub html: Option<String>,
    /// URL or mod asset path to load (mutually exclusive with html).
    pub url: Option<String>,
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
    /// New inline HTML content to display.
    pub html: String,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// MCP handler that bridges AI agent requests to the Homunculus engine.
///
/// Holds domain API handles for dispatching tool calls and resource reads
/// to the Bevy ECS via the [`ApiReactor`] pattern.
#[derive(Clone)]
pub struct HomunculusMcpHandler {
    pub(crate) webview_api: WebviewApi,
    pub(crate) app_api: AppApi,
    pub(crate) vrm_api: VrmApi,
    pub(crate) mods_api: ModsApi,
    pub(crate) assets_api: AssetsApi,
    /// Tracks open webview IDs so they can be cleaned up when the MCP session ends.
    pub(crate) open_webviews: Arc<Mutex<Vec<u64>>>,
    tool_router: ToolRouter<Self>,
}

impl HomunculusMcpHandler {
    /// Creates a new handler, constructing all domain APIs from the given reactor.
    pub fn new(reactor: ApiReactor) -> Self {
        Self {
            webview_api: WebviewApi::from(reactor.clone()),
            app_api: AppApi::from(reactor.clone()),
            vrm_api: VrmApi::from(reactor.clone()),
            mods_api: ModsApi::from(reactor.clone()),
            assets_api: AssetsApi::from(reactor),
            open_webviews: Arc::new(Mutex::new(Vec::new())),
            tool_router: Self::tool_router(),
        }
    }
}

// ---------------------------------------------------------------------------
// ServerHandler impl (resources are handled here; tools via tool_handler macro)
// ---------------------------------------------------------------------------

#[tool_handler(router = self.tool_router)]
impl ServerHandler for HomunculusMcpHandler {
    fn get_info(&self) -> ServerInfo {
        let capabilities = ServerCapabilities::builder()
            .enable_tools()
            .enable_resources()
            .build();

        ServerInfo::new(capabilities)
            .with_server_info(Implementation::new(
                SERVER_NAME,
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Desktop Homunculus MCP server. Controls a desktop mascot application — \
                 manage VRM characters, open webviews, query mods and assets.",
            )
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, rmcp::ErrorData>> + Send + '_
    {
        std::future::ready(Ok(ListResourcesResult {
            meta: None,
            next_cursor: None,
            resources: resource_definitions(),
        }))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, rmcp::ErrorData>> + Send + '_
    {
        async move {
            let uri = &request.uri;
            let json_text = match uri.as_str() {
                "homunculus://info" => {
                    let mod_list = self
                        .mods_api
                        .list()
                        .await
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

                    let info = serde_json::json!({
                        "version": env!("CARGO_PKG_VERSION"),
                        "platform": {
                            "os": std::env::consts::OS,
                            "arch": std::env::consts::ARCH,
                        },
                        "features": FEATURES,
                        "mods": mod_list,
                    });
                    serde_json::to_string_pretty(&info)
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
                }
                "homunculus://characters" => {
                    let snapshots = self
                        .vrm_api
                        .snapshot()
                        .await
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

                    serde_json::to_string_pretty(&snapshots)
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
                }
                "homunculus://mods" => {
                    let mods = self
                        .mods_api
                        .list()
                        .await
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

                    serde_json::to_string_pretty(&mods)
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
                }
                "homunculus://assets" => {
                    let assets = self
                        .assets_api
                        .list(AssetFilter::default())
                        .await
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?;

                    serde_json::to_string_pretty(&assets)
                        .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))?
                }
                _ => {
                    return Err(rmcp::ErrorData::resource_not_found(
                        format!("Unknown resource: {uri}"),
                        None,
                    ));
                }
            };

            Ok(ReadResourceResult::new(vec![
                ResourceContents::text(json_text, uri.clone())
                    .with_mime_type("application/json"),
            ]))
        }
    }
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[tool_router(router = tool_router)]
impl HomunculusMcpHandler {
    /// Open a webview panel displaying HTML content or a URL near the active character.
    #[tool(
        name = "open_webview",
        description = "Open a webview panel displaying HTML content or a URL near the active character. Returns the webview entity ID. Use close_webview to close it."
    )]
    async fn open_webview(&self, params: Parameters<OpenWebviewParams>) -> String {
        let args = params.0;

        let source = match (args.html, args.url) {
            (Some(html), None) => WebviewSource::Html { content: html },
            (None, Some(url)) => WebviewSource::Url { url },
            (Some(_), Some(_)) => {
                return "Error: 'html' and 'url' are mutually exclusive.".to_string();
            }
            (None, None) => {
                return "Error: Either 'html' or 'url' must be provided.".to_string();
            }
        };

        let size_x = args.size_x.unwrap_or(0.7);
        let size_y = args.size_y.unwrap_or(0.5);
        let viewport_width = args.viewport_width.unwrap_or(800);
        let viewport_height = args.viewport_height.unwrap_or(600);
        let offset_x = args.offset_x.unwrap_or(0.0);
        let offset_y = args.offset_y.unwrap_or(0.5);

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
        } else {
            let target_entity_id = if let Some(id) = args.entity {
                id
            } else {
                let last = self
                    .open_webviews
                    .lock()
                    .ok()
                    .and_then(|v| v.last().copied());
                match last {
                    Some(id) => id,
                    None => return "No webviews tracked.".to_string(),
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
    }

    /// Navigate an existing webview to new HTML content.
    #[tool(
        name = "navigate_webview",
        description = "Navigate an existing webview to new HTML content. Use this to update a webview's content without closing and reopening it. If no entity is specified, navigates the most recently opened webview."
    )]
    async fn navigate_webview(&self, params: Parameters<NavigateWebviewParams>) -> String {
        let args = params.0;

        let target_entity_id = if let Some(id) = args.entity {
            id
        } else {
            let last = self
                .open_webviews
                .lock()
                .ok()
                .and_then(|v| v.last().copied());
            match last {
                Some(id) => id,
                None => {
                    return "No webviews tracked. Open a webview first with open_webview."
                        .to_string();
                }
            }
        };

        let entity = Entity::from_bits(target_entity_id);
        let source = WebviewSource::Html {
            content: args.html,
        };

        match self.webview_api.navigate(entity, source).await {
            Ok(()) => format!("Navigated webview (entity {target_entity_id}) to new content."),
            Err(e) => format!("Error navigating webview: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use homunculus_api::prelude::ApiReactor;

    /// Creates a handler backed by a dummy reactor (no Bevy app).
    fn test_handler() -> HomunculusMcpHandler {
        let reactor = ApiReactor::__test_dummy();
        HomunculusMcpHandler::new(reactor)
    }

    #[test]
    fn get_info_returns_correct_server_name_and_version() {
        let handler = test_handler();
        let info = ServerHandler::get_info(&handler);

        assert_eq!(info.server_info.name, "homunculus");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn get_info_enables_tools_and_resources() {
        let handler = test_handler();
        let info = ServerHandler::get_info(&handler);

        let caps = info.capabilities;
        assert!(caps.tools.is_some(), "tools capability should be enabled");
        assert!(
            caps.resources.is_some(),
            "resources capability should be enabled"
        );
    }

    #[test]
    fn get_info_includes_instructions() {
        let handler = test_handler();
        let info = ServerHandler::get_info(&handler);

        let instructions = info.instructions.expect("instructions should be set");
        assert!(
            instructions.contains("Desktop Homunculus"),
            "instructions should mention Desktop Homunculus"
        );
    }

    #[test]
    fn tool_router_lists_three_tools() {
        let router = HomunculusMcpHandler::tool_router();
        let tools = router.list_all();

        assert_eq!(tools.len(), 3, "expected 3 tools, got {}", tools.len());

        let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(names.contains(&"open_webview"), "missing open_webview tool");
        assert!(
            names.contains(&"close_webview"),
            "missing close_webview tool"
        );
        assert!(
            names.contains(&"navigate_webview"),
            "missing navigate_webview tool"
        );
    }

    #[test]
    fn tool_router_tools_have_descriptions() {
        let router = HomunculusMcpHandler::tool_router();
        let tools = router.list_all();

        for tool in &tools {
            assert!(
                tool.description.is_some(),
                "tool '{}' should have a description",
                tool.name
            );
            assert!(
                !tool.description.as_ref().unwrap().is_empty(),
                "tool '{}' description should not be empty",
                tool.name
            );
        }
    }

    #[test]
    fn tool_router_tools_have_input_schemas() {
        let router = HomunculusMcpHandler::tool_router();
        let tools = router.list_all();

        for tool in &tools {
            assert!(
                !tool.input_schema.is_empty(),
                "tool '{}' should have an input schema",
                tool.name
            );
        }
    }

    #[test]
    fn resource_definitions_lists_four_resources() {
        let resources = resource_definitions();
        assert_eq!(
            resources.len(),
            4,
            "expected 4 resources, got {}",
            resources.len()
        );
    }

    #[test]
    fn resource_definitions_have_correct_uris() {
        let resources = resource_definitions();
        let uris: Vec<&str> = resources.iter().map(|r| r.raw.uri.as_str()).collect();

        assert!(uris.contains(&"homunculus://info"), "missing info resource");
        assert!(
            uris.contains(&"homunculus://characters"),
            "missing characters resource"
        );
        assert!(uris.contains(&"homunculus://mods"), "missing mods resource");
        assert!(
            uris.contains(&"homunculus://assets"),
            "missing assets resource"
        );
    }

    #[test]
    fn resource_definitions_have_json_mime_type() {
        let resources = resource_definitions();
        for resource in &resources {
            assert_eq!(
                resource.raw.mime_type.as_deref(),
                Some("application/json"),
                "resource '{}' should have application/json mime type",
                resource.raw.uri
            );
        }
    }

    #[test]
    fn resource_definitions_have_descriptions() {
        let resources = resource_definitions();
        for resource in &resources {
            assert!(
                resource.raw.description.is_some(),
                "resource '{}' should have a description",
                resource.raw.uri
            );
        }
    }

    #[test]
    fn handler_starts_with_empty_webview_tracker() {
        let handler = test_handler();
        let tracked = handler.open_webviews.lock().unwrap();
        assert!(tracked.is_empty(), "new handler should have no tracked webviews");
    }
}
