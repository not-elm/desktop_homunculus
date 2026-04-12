//! MCP service factory for creating server instances.
//!
//! Provides [`create_mcp_service`], which builds a [`StreamableHttpService`]
//! suitable for mounting on the engine's Axum router via `nest_service`.

use std::sync::{Arc, RwLock};

use homunculus_api::prelude::ApiReactor;
use homunculus_core::rpc_registry::RpcRegistry;
use homunculus_utils::config::HomunculusConfig;
use homunculus_utils::runtime::RuntimeResolver;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};

use crate::handler::HomunculusMcpHandler;

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
    rpc_registry: Arc<RwLock<RpcRegistry>>,
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
                rpc_registry.clone(),
            ))
        },
        session_manager,
        server_config,
    )
}
