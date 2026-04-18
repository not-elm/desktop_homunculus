//! MCP service factory for creating server instances.
//!
//! Provides [`create_mcp_service`], which builds a [`StreamableHttpService`]
//! suitable for mounting on the engine's Axum router via `nest_service`.

use std::sync::Arc;

use homunculus_api::prelude::ApiReactor;
use homunculus_utils::config::HomunculusConfig;
use homunculus_utils::runtime::RuntimeResolver;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};

use crate::downstream::SharedMcpExtensionRegistry;
use crate::handler::HomunculusMcpHandler;
use crate::upstream_hub::UpstreamSessionHub;

/// Creates a [`StreamableHttpService`] backed by the given [`ApiReactor`].
///
/// The returned service implements `tower::Service<Request<B>>` and can be
/// mounted on an Axum router with `nest_service`.
///
/// Each new MCP session spawns a fresh [`HomunculusMcpHandler`] via the
/// factory closure, so sessions are fully isolated.
pub fn create_mcp_service(
    reactor: ApiReactor,
    config: HomunculusConfig,
    runtime: RuntimeResolver,
    registry: SharedMcpExtensionRegistry,
    upstream_hub: Arc<UpstreamSessionHub>,
) -> StreamableHttpService<HomunculusMcpHandler, LocalSessionManager> {
    let server_config = StreamableHttpServerConfig::default();
    let session_manager = Arc::new(LocalSessionManager {
        sessions: Default::default(),
        session_config: Default::default(),
    });
    StreamableHttpService::new(
        move || {
            Ok(HomunculusMcpHandler::new(
                reactor.clone(),
                config.clone(),
                runtime.clone(),
                registry.clone(),
                upstream_hub.clone(),
            ))
        },
        session_manager,
        server_config,
    )
}
