//! MCP request handler implementation.
//!
//! Provides [`HomunculusMcpHandler`], the central handler that implements the
//! [`ServerHandler`] trait from `rmcp`, exposing Desktop Homunculus capabilities
//! (tools and resources) to MCP-compatible AI agents.

use std::sync::{Arc, Mutex};

use homunculus_api::assets::AssetsApi;
use homunculus_api::mods::ModsApi;
use homunculus_api::prelude::{ApiReactor, AppApi, VrmApi, WebviewApi};
use rmcp::handler::server::ServerHandler;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};

const SERVER_NAME: &str = "homunculus";

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
        }
    }
}

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
}
