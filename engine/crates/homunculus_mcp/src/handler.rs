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
use homunculus_core::rpc_registry::RpcRegistry;
use homunculus_utils::config::HomunculusConfig;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{
    GetPromptRequestParams, GetPromptResult, Implementation, ListPromptsResult,
    ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResult,
    ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler, tool_handler};
use std::sync::{Arc, Mutex, RwLock};

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
    /// Tracks open webview IDs so they can be cleaned up when the MCP session ends.
    pub(crate) open_webviews: Arc<Mutex<Vec<u64>>>,
    pub(crate) rpc_registry: Arc<RwLock<RpcRegistry>>,
    tool_router: ToolRouter<Self>,
}

impl HomunculusMcpHandler {
    /// Creates a new handler, constructing all domain APIs from the given reactor.
    pub fn new(
        reactor: ApiReactor,
        config: HomunculusConfig,
        rpc_registry: Arc<RwLock<RpcRegistry>>,
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
            open_webviews: Arc::new(Mutex::new(Vec::new())),
            rpc_registry,
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

        let personas = self
            .persona_api
            .list()
            .await
            .map_err(|e| e.to_string())?;
        let first = personas
            .first()
            .ok_or_else(|| "No characters loaded. Use spawn_character first.".to_string())?;
        self.set_active_character(Some(first.id.clone()));

        self.persona_api
            .resolve(first.id.clone())
            .await
            .map_err(|e| e.to_string())
    }

    /// Resolves a persona by display name.
    ///
    /// Searches all personas for a matching `name` field first,
    /// then falls back to matching against the persona ID string.
    pub(crate) async fn resolve_persona_by_name(
        &self,
        name: &str,
    ) -> Result<Persona, String> {
        let personas = self
            .persona_api
            .list()
            .await
            .map_err(|e| e.to_string())?;

        if let Some(p) = personas
            .iter()
            .find(|p| p.name.as_deref() == Some(name))
        {
            return Ok(p.clone());
        }

        if let Some(p) = personas.iter().find(|p| p.id.0 == name) {
            return Ok(p.clone());
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

#[tool_handler(router = self.tool_router)]
impl ServerHandler for HomunculusMcpHandler {
    fn get_info(&self) -> ServerInfo {
        let capabilities = ServerCapabilities::builder()
            .enable_tools()
            .enable_resources()
            .enable_prompts()
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

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListPromptsResult, rmcp::ErrorData>> + Send + '_
    {
        std::future::ready(Ok(ListPromptsResult {
            meta: None,
            next_cursor: None,
            prompts: prompts::prompt_definitions(),
        }))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<GetPromptResult, rmcp::ErrorData>> + Send + '_
    {
        let args = request.arguments.unwrap_or_default();
        std::future::ready(prompts::get_prompt(&request.name, &args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use homunculus_api::prelude::ApiReactor;

    /// Creates a handler backed by a dummy reactor (no Bevy app).
    fn test_handler() -> HomunculusMcpHandler {
        use homunculus_core::rpc_registry::RpcRegistry;
        use std::sync::{Arc, RwLock};
        let reactor = ApiReactor::__test_dummy();
        let config = HomunculusConfig {
            mods_dir: std::path::PathBuf::from("/tmp/mods"),
            port: 3100,
            ..Default::default()
        };
        let rpc_registry = Arc::new(RwLock::new(RpcRegistry::default()));
        HomunculusMcpHandler::new(reactor, config, rpc_registry)
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
    fn resource_definitions_lists_five_resources() {
        let resources = resources::resource_definitions();
        assert_eq!(
            resources.len(),
            5,
            "expected 5 resources, got {}",
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
        assert!(uris.contains(&"homunculus://rpc"), "missing rpc resource");
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
}
