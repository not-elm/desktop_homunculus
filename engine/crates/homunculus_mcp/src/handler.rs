//! MCP request handler implementation.
//!
//! Provides [`HomunculusMcpHandler`], the central handler that implements the
//! [`ServerHandler`] trait from `rmcp`, exposing Desktop Homunculus capabilities
//! (tools and resources) to MCP-compatible AI agents.

mod prompts;
mod resources;
mod tools;

use bevy::prelude::Entity;
use homunculus_api::assets::AssetsApi;
use homunculus_api::mods::ModsApi;
use homunculus_api::prelude::{
    ApiReactor, AudioBgmApi, AudioSeApi, EntitiesApi, PersonaApi, VrmAnimationApi, VrmApi,
    WebviewApi,
};
use homunculus_core::prelude::{Persona, PersonaId};
use homunculus_utils::config::HomunculusConfig;
use homunculus_utils::runtime::RuntimeResolver;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, GetPromptRequestParams, GetPromptResult,
    Implementation, InitializeRequestParams, InitializeResult, ListPromptsResult,
    ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, PaginatedRequestParams,
    ReadResourceRequestParams, ReadResourceResult, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler};
use std::sync::{Arc, Mutex};

const SERVER_NAME: &str = "homunculus";

const FEATURES: &[&str] = &[
    "vrm", "audio", "webviews", "effects", "speech", "signals", "mods",
];

/// Converts any `Display` error into an `rmcp::ErrorData`.
pub(crate) fn api_err(e: impl std::fmt::Display) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(e.to_string(), None)
}

/// Serializes a value to a pretty-printed JSON string.
pub(crate) fn to_json_string(value: &impl serde::Serialize) -> Result<String, rmcp::ErrorData> {
    serde_json::to_string_pretty(value).map_err(api_err)
}

/// Logs a warning if the aggregated list exceeds the soft limit (SA2 from spec).
fn warn_total_limit(n: usize, kind: &'static str) {
    const LIMIT: usize = 1000;
    if n > LIMIT {
        bevy::log::warn!(
            count = n,
            limit = LIMIT,
            kind,
            "aggregated MCP list exceeds soft limit",
        );
    }
}

/// Convert [`crate::downstream::DownstreamError`] into [`rmcp::ErrorData`].
///
/// Maps the 6 [`rmcp::service::ServiceError`] variants to appropriate MCP error codes.
fn downstream_error_to_mcp(e: crate::downstream::DownstreamError) -> rmcp::ErrorData {
    use crate::downstream::DownstreamError;
    use rmcp::service::ServiceError;
    match e {
        DownstreamError::UnknownSlug(s) => {
            rmcp::ErrorData::invalid_params(format!("unknown mod slug: {s}"), None)
        }
        DownstreamError::ServiceError(inner) => match inner {
            ServiceError::TransportSend(_) | ServiceError::TransportClosed => {
                rmcp::ErrorData::internal_error("downstream unavailable", None)
            }
            ServiceError::Timeout { .. } => {
                rmcp::ErrorData::internal_error("downstream timeout", None)
            }
            ServiceError::McpError(mcp_err) => mcp_err,
            other => rmcp::ErrorData::internal_error(other.to_string(), None),
        },
    }
}

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
    pub(crate) audio_se_api: AudioSeApi,
    pub(crate) audio_bgm_api: AudioBgmApi,
    pub(crate) entities_api: EntitiesApi,
    pub(crate) vrma_api: VrmAnimationApi,
    pub(crate) persona_api: PersonaApi,
    /// Stores the active persona's [`PersonaId`] for character resolution.
    pub(crate) active_character: Arc<Mutex<Option<PersonaId>>>,
    pub(crate) config: HomunculusConfig,
    pub(crate) runtime: RuntimeResolver,
    /// Tracks open webview IDs so they can be cleaned up when the MCP session ends.
    pub(crate) open_webviews: Arc<Mutex<Vec<u64>>>,
    /// Registry of downstream mod MCP servers whose tools/prompts/resources are proxied here.
    pub(crate) registry: crate::downstream::SharedMcpExtensionRegistry,
    /// Hub for broadcasting list_changed notifications to all connected upstream MCP clients.
    pub(crate) upstream_hub: Arc<crate::upstream_hub::UpstreamSessionHub>,
    tool_router: ToolRouter<Self>,
}

impl HomunculusMcpHandler {
    /// Creates a new handler, constructing all domain APIs from the given reactor.
    pub fn new(
        reactor: ApiReactor,
        config: HomunculusConfig,
        runtime: RuntimeResolver,
        registry: crate::downstream::SharedMcpExtensionRegistry,
        upstream_hub: Arc<crate::upstream_hub::UpstreamSessionHub>,
    ) -> Self {
        Self {
            webview_api: WebviewApi::from(reactor.clone()),
            vrm_api: VrmApi::from(reactor.clone()),
            mods_api: ModsApi::from(reactor.clone()),
            audio_se_api: AudioSeApi::from(reactor.clone()),
            audio_bgm_api: AudioBgmApi::from(reactor.clone()),
            entities_api: EntitiesApi::from(reactor.clone()),
            vrma_api: VrmAnimationApi::from(reactor.clone()),
            persona_api: PersonaApi::from(reactor.clone()),
            assets_api: AssetsApi::from(reactor),
            active_character: Arc::new(Mutex::new(None)),
            config,
            runtime,
            open_webviews: Arc::new(Mutex::new(Vec::new())),
            registry,
            upstream_hub,
            tool_router: tools::tool_router(),
        }
    }

    /// Resolves the active character entity, falling back to the first persona.
    pub(crate) async fn resolve_character(&self) -> Result<Entity, String> {
        let current = self.active_persona_id();

        if let Some(persona_id) = current {
            return self
                .persona_api
                .resolve(persona_id)
                .await
                .map_err(|e| e.to_string());
        }

        let personas = self.persona_api.list().await.map_err(|e| e.to_string())?;
        let first = personas
            .first()
            .ok_or_else(|| "No characters loaded. Use spawn_character first.".to_string())?;
        self.set_active_character(Some(first.persona.id.clone()));

        self.persona_api
            .resolve(first.persona.id.clone())
            .await
            .map_err(|e| e.to_string())
    }

    /// Resolves a persona by display name.
    ///
    /// Searches all personas for a matching `name` field first,
    /// then falls back to matching against the persona ID string.
    pub(crate) async fn resolve_persona_by_name(&self, name: &str) -> Result<Persona, String> {
        let snapshots = self.persona_api.list().await.map_err(|e| e.to_string())?;

        if let Some(s) = snapshots
            .iter()
            .find(|s| s.persona.name.as_deref() == Some(name))
        {
            return Ok(s.persona.clone());
        }

        if let Some(s) = snapshots.iter().find(|s| s.persona.id.0 == name) {
            return Ok(s.persona.clone());
        }

        Err(format!(
            "No persona found with name or id '{name}'. Use get_character_snapshot to see available characters."
        ))
    }

    /// Returns the currently active [`PersonaId`], if any.
    pub(crate) fn active_persona_id(&self) -> Option<PersonaId> {
        self.active_character
            .lock()
            .unwrap_or_else(|e| {
                bevy::log::warn!("Mutex poisoned: {e}");
                e.into_inner()
            })
            .clone()
    }

    /// Sets or clears the active character by [`PersonaId`].
    pub(crate) fn set_active_character(&self, persona_id: Option<PersonaId>) {
        *self.active_character.lock().unwrap_or_else(|e| {
            bevy::log::warn!("Mutex poisoned: {e}");
            e.into_inner()
        }) = persona_id;
    }

}

impl ServerHandler for HomunculusMcpHandler {
    fn get_info(&self) -> ServerInfo {
        let capabilities = ServerCapabilities::builder()
            .enable_tools()
            .enable_tool_list_changed()
            .enable_resources()
            .enable_resources_list_changed()
            .enable_prompts()
            .enable_prompts_list_changed()
            .build();

        ServerInfo::new(capabilities)
            .with_server_info(Implementation::new(SERVER_NAME, env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "Desktop Homunculus MCP server. Controls a desktop mascot application — \
                 manage VRM characters, open webviews, query mods and assets.",
            )
    }

    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, rmcp::ErrorData> {
        // Register the connecting upstream client so list_changed notifications can be sent.
        self.upstream_hub.register(context.peer.clone()).await;

        // Retain default behavior: store peer info if not already set.
        if context.peer.peer_info().is_none() {
            context.peer.set_peer_info(request);
        }
        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        let mut tools = self.tool_router.list_all();
        let registry = self.registry.0.read().await;
        tools.extend(registry.list_all_tools_prefixed().await);
        warn_total_limit(tools.len(), "tools");
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        // Downstream dispatch only when prefix matches a registered slug.
        if let Some((slug, original)) = request.name.split_once("__") {
            let registry = self.registry.0.read().await;
            if registry.has_slug(slug) {
                let args = request.arguments.clone().unwrap_or_default();
                return registry
                    .call_tool_by_parts(slug, original, args)
                    .await
                    .map_err(downstream_error_to_mcp);
            }
        }
        // Fall through to built-in static tool_router.
        let tcc = ToolCallContext::new(self, request, context);
        self.tool_router.call(tcc).await
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.tool_router.get(name).cloned()
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        let mut resources = resources::resource_definitions();
        let registry = self.registry.0.read().await;
        resources.extend(registry.list_all_resources().await);
        warn_total_limit(resources.len(), "resources");
        Ok(ListResourcesResult {
            meta: None,
            next_cursor: None,
            resources,
        })
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, rmcp::ErrorData> {
        let registry = self.registry.0.read().await;
        let resource_templates = registry.list_all_resource_templates().await;
        Ok(ListResourceTemplatesResult {
            meta: None,
            next_cursor: None,
            resource_templates,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, rmcp::ErrorData> {
        if request.uri.starts_with("homunculus://") {
            return resources::read_resource(self, request).await;
        }
        let registry = self.registry.0.read().await;
        registry
            .read_resource(&request.uri)
            .await
            .map_err(downstream_error_to_mcp)
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, rmcp::ErrorData> {
        let mut prompts = prompts::prompt_definitions();
        let registry = self.registry.0.read().await;
        prompts.extend(registry.list_all_prompts_prefixed().await);
        warn_total_limit(prompts.len(), "prompts");
        Ok(ListPromptsResult {
            meta: None,
            next_cursor: None,
            prompts,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        if let Some((slug, original)) = request.name.split_once("__") {
            let registry = self.registry.0.read().await;
            if registry.has_slug(slug) {
                return registry
                    .get_prompt_by_parts(slug, original, request.arguments.clone())
                    .await
                    .map_err(downstream_error_to_mcp);
            }
        }
        let args = request.arguments.unwrap_or_default();
        prompts::get_prompt(&request.name, &args)
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
            ..Default::default()
        };
        let runtime = RuntimeResolver::detect();
        let upstream_hub = crate::upstream_hub::UpstreamSessionHub::new();
        let (registry, _deregister_sender) =
            crate::downstream::McpExtensionRegistry::new(upstream_hub.clone());
        HomunculusMcpHandler::new(reactor, config, runtime, registry, upstream_hub)
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

    #[test]
    fn handler_starts_with_no_active_character() {
        let handler = test_handler();
        let active = handler.active_character.lock().unwrap();
        assert!(
            active.is_none(),
            "new handler should have no active character"
        );
    }

    #[test]
    fn get_info_enables_prompts() {
        let handler = test_handler();
        let info = ServerHandler::get_info(&handler);

        assert!(
            info.capabilities.prompts.is_some(),
            "prompts capability should be enabled"
        );
    }

    #[test]
    fn prompt_definitions_lists_three_prompts() {
        let prompts = prompts::prompt_definitions();
        assert_eq!(
            prompts.len(),
            3,
            "expected 3 prompts, got {}",
            prompts.len()
        );

        let names: Vec<&str> = prompts.iter().map(|p| p.name.as_ref()).collect();
        assert!(
            names.contains(&"developer-assistant"),
            "missing developer-assistant prompt"
        );
        assert!(
            names.contains(&"character-interaction"),
            "missing character-interaction prompt"
        );
        assert!(
            names.contains(&"mod-command-helper"),
            "missing mod-command-helper prompt"
        );
    }

    #[test]
    fn get_tool_finds_static_tools() {
        let handler = test_handler();
        let tools = handler.tool_router.list_all();
        if let Some(first_static) = tools.first() {
            let found = handler.get_tool(&first_static.name);
            assert!(found.is_some(), "should find static tool by name");
        }
    }

    #[test]
    fn get_tool_returns_none_for_unknown() {
        let handler = test_handler();
        assert!(handler.get_tool("totally_unknown").is_none());
    }

    #[test]
    fn get_character_snapshot_has_read_only_annotation() {
        let handler = test_handler();
        let tool = handler
            .get_tool("get_character_snapshot")
            .expect("get_character_snapshot should exist");
        let ann = tool.annotations.expect("should have annotations");
        assert_eq!(ann.read_only_hint, Some(true));
        assert_eq!(ann.open_world_hint, Some(false));
    }

    #[test]
    fn execute_command_has_no_annotations() {
        let handler = test_handler();
        let tool = handler
            .get_tool("execute_command")
            .expect("execute_command should exist");
        assert!(
            tool.annotations.is_none(),
            "execute_command should have no annotations (all defaults)"
        );
    }

    #[test]
    fn remove_character_is_destructive_and_idempotent() {
        let handler = test_handler();
        let tool = handler
            .get_tool("remove_character")
            .expect("remove_character should exist");
        let ann = tool.annotations.expect("should have annotations");
        assert_eq!(
            ann.destructive_hint, None,
            "destructive_hint should use default (true)"
        );
        assert_eq!(ann.idempotent_hint, Some(true));
        assert_eq!(ann.open_world_hint, Some(false));
    }
}
