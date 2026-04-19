//! Custom rmcp ClientHandler for downstream mod MCP servers.
//! Intercepts list_changed notifications to invalidate caches and broadcast upstream.

use std::sync::Arc;

use rmcp::{
    model::{
        ClientCapabilities, ClientInfo, Implementation, LoggingMessageNotificationParam,
        ProtocolVersion,
    },
    service::{NotificationContext, RoleClient},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::downstream::CacheInvalidation;
use crate::upstream_hub::UpstreamSessionHub;

pub struct DownstreamClientHandler {
    pub mod_slug: String,
    pub upstream_hub: Arc<UpstreamSessionHub>,
    pub cache_invalidator: UnboundedSender<CacheInvalidation>,
}

impl DownstreamClientHandler {
    pub fn new(
        mod_slug: String,
        upstream_hub: Arc<UpstreamSessionHub>,
        cache_invalidator: UnboundedSender<CacheInvalidation>,
    ) -> Self {
        Self {
            mod_slug,
            upstream_hub,
            cache_invalidator,
        }
    }
}

impl rmcp::ClientHandler for DownstreamClientHandler {
    fn get_info(&self) -> ClientInfo {
        ClientInfo::new(
            ClientCapabilities::default(),
            Implementation::new("homunculus-aggregator", env!("CARGO_PKG_VERSION")),
        )
        .with_protocol_version(ProtocolVersion::V_2025_03_26)
    }

    async fn on_tool_list_changed(&self, _context: NotificationContext<RoleClient>) {
        let _ = self
            .cache_invalidator
            .send(CacheInvalidation::Tools(self.mod_slug.clone()));
        self.upstream_hub.notify_tools_changed().await;
    }

    async fn on_prompt_list_changed(&self, _context: NotificationContext<RoleClient>) {
        let _ = self
            .cache_invalidator
            .send(CacheInvalidation::Prompts(self.mod_slug.clone()));
        self.upstream_hub.notify_prompts_changed().await;
    }

    async fn on_resource_list_changed(&self, _context: NotificationContext<RoleClient>) {
        let _ = self
            .cache_invalidator
            .send(CacheInvalidation::Resources(self.mod_slug.clone()));
        self.upstream_hub.notify_resources_changed().await;
    }

    async fn on_logging_message(
        &self,
        params: LoggingMessageNotificationParam,
        _context: NotificationContext<RoleClient>,
    ) {
        bevy::log::info!("downstream mcp log [{}]: {:?}", self.mod_slug, params);
    }
}
