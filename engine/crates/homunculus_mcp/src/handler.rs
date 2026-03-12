//! MCP request handler implementation.
//!
//! Provides [`HomunculusMcpHandler`], the central handler that implements the
//! [`ServerHandler`] trait from `rmcp`, exposing Desktop Homunculus capabilities
//! (tools and resources) to MCP-compatible AI agents.

#[allow(dead_code)]
mod presets;
mod resources;
mod tools;

use std::sync::{Arc, Mutex};

use bevy::prelude::Entity;
use homunculus_api::assets::AssetsApi;
use homunculus_api::mods::ModsApi;
use homunculus_api::prelude::{
    ApiReactor, AudioBgmApi, AudioSeApi, EntitiesApi, VrmAnimationApi, VrmApi, WebviewApi,
};
use homunculus_utils::config::HomunculusConfig;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{
    Implementation, ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams,
    ReadResourceResult, ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler, tool_handler};

const SERVER_NAME: &str = "homunculus";

const FEATURES: &[&str] = &[
    "vrm", "audio", "webviews", "effects", "speech", "signals", "mods",
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Converts any `Display` error into an `rmcp::ErrorData`.
pub(crate) fn api_err(e: impl std::fmt::Display) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(e.to_string(), None)
}

/// Serializes a value to a pretty-printed JSON string.
pub(crate) fn to_json_string(value: &impl serde::Serialize) -> Result<String, rmcp::ErrorData> {
    serde_json::to_string_pretty(value).map_err(api_err)
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
    pub(crate) vrm_api: VrmApi,
    pub(crate) mods_api: ModsApi,
    pub(crate) assets_api: AssetsApi,
    #[allow(dead_code)]
    pub(crate) audio_se_api: AudioSeApi,
    #[allow(dead_code)]
    pub(crate) audio_bgm_api: AudioBgmApi,
    #[allow(dead_code)]
    pub(crate) entities_api: EntitiesApi,
    #[allow(dead_code)]
    pub(crate) vrma_api: VrmAnimationApi,
    #[allow(dead_code)]
    pub(crate) active_character: Arc<Mutex<Option<u64>>>,
    #[allow(dead_code)]
    pub(crate) config: HomunculusConfig,
    /// Tracks open webview IDs so they can be cleaned up when the MCP session ends.
    pub(crate) open_webviews: Arc<Mutex<Vec<u64>>>,
    tool_router: ToolRouter<Self>,
}

impl HomunculusMcpHandler {
    /// Creates a new handler, constructing all domain APIs from the given reactor.
    pub fn new(reactor: ApiReactor, config: HomunculusConfig) -> Self {
        Self {
            webview_api: WebviewApi::from(reactor.clone()),
            vrm_api: VrmApi::from(reactor.clone()),
            mods_api: ModsApi::from(reactor.clone()),
            audio_se_api: AudioSeApi::from(reactor.clone()),
            audio_bgm_api: AudioBgmApi::from(reactor.clone()),
            entities_api: EntitiesApi::from(reactor.clone()),
            vrma_api: VrmAnimationApi::from(reactor.clone()),
            assets_api: AssetsApi::from(reactor),
            active_character: Arc::new(Mutex::new(None)),
            config,
            open_webviews: Arc::new(Mutex::new(Vec::new())),
            tool_router: tools::tool_router(),
        }
    }

    /// Resolves the active character entity, falling back to the first character in snapshot.
    #[allow(dead_code)]
    pub(crate) async fn resolve_character(&self) -> Result<Entity, String> {
        if let Some(bits) = self.active_character.lock().ok().and_then(|g| *g) {
            return Ok(Entity::from_bits(bits));
        }
        let snapshots = self.vrm_api.snapshot().await.map_err(|e| e.to_string())?;
        let first = snapshots
            .first()
            .ok_or_else(|| "No characters loaded. Use spawn_character first.".to_string())?;
        let bits = first.entity.to_bits();
        if let Ok(mut guard) = self.active_character.lock() {
            *guard = Some(bits);
        }
        Ok(first.entity)
    }

    /// Sets or clears the active character.
    #[allow(dead_code)]
    pub(crate) fn set_active_character(&self, entity: Option<u64>) {
        if let Ok(mut guard) = self.active_character.lock() {
            *guard = entity;
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
            .with_server_info(Implementation::new(SERVER_NAME, env!("CARGO_PKG_VERSION")))
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
            resources: resources::resource_definitions(),
        }))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, rmcp::ErrorData>> + Send + '_
    {
        resources::read_resource(self, request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use homunculus_api::prelude::ApiReactor;

    /// Creates a handler backed by a dummy reactor (no Bevy app).
    fn test_handler() -> HomunculusMcpHandler {
        let reactor = ApiReactor::__test_dummy();
        let config = HomunculusConfig {
            mods_dir: std::path::PathBuf::from("/tmp/mods"),
            port: 3100,
        };
        HomunculusMcpHandler::new(reactor, config)
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
        let router = tools::tool_router();
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
        let router = tools::tool_router();
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
        let router = tools::tool_router();
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
        let resources = resources::resource_definitions();
        assert_eq!(
            resources.len(),
            4,
            "expected 4 resources, got {}",
            resources.len()
        );
    }

    #[test]
    fn resource_definitions_have_correct_uris() {
        let resources = resources::resource_definitions();
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
        let resources = resources::resource_definitions();
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
        let resources = resources::resource_definitions();
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
        assert!(
            tracked.is_empty(),
            "new handler should have no tracked webviews"
        );
    }
}
