//! Tracks active upstream MCP client sessions and broadcasts list_changed notifications
//! when the McpExtensionRegistry mutates.

use futures::future::join_all;
use rmcp::service::{Peer, RoleServer, ServiceError};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Bevy resource wrapper for a shared [`UpstreamSessionHub`].
///
/// Inserted into the Bevy world during HTTP server setup so that any system or
/// handler needing to reach the upstream session hub can access it via
/// `Res<SharedUpstreamHub>`.
#[derive(Clone, bevy::prelude::Resource)]
pub struct SharedUpstreamHub(pub Arc<UpstreamSessionHub>);

#[derive(Clone, Default)]
pub struct UpstreamSessionHub {
    peers: Arc<RwLock<Vec<Peer<RoleServer>>>>,
}

impl UpstreamSessionHub {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn register(&self, peer: Peer<RoleServer>) {
        self.peers.write().await.push(peer);
    }

    pub async fn notify_tools_changed(&self) {
        self.broadcast_tools().await;
    }

    pub async fn notify_prompts_changed(&self) {
        self.broadcast_prompts().await;
    }

    pub async fn notify_resources_changed(&self) {
        self.broadcast_resources().await;
    }

    async fn broadcast_tools(&self) {
        let peers: Vec<Peer<RoleServer>> = self.peers.read().await.clone();
        let results: Vec<Result<(), ServiceError>> =
            join_all(peers.iter().map(|p| p.notify_tool_list_changed())).await;
        self.prune_after(&results).await;
    }

    async fn broadcast_prompts(&self) {
        let peers: Vec<Peer<RoleServer>> = self.peers.read().await.clone();
        let results: Vec<Result<(), ServiceError>> =
            join_all(peers.iter().map(|p| p.notify_prompt_list_changed())).await;
        self.prune_after(&results).await;
    }

    async fn broadcast_resources(&self) {
        let peers: Vec<Peer<RoleServer>> = self.peers.read().await.clone();
        let results: Vec<Result<(), ServiceError>> =
            join_all(peers.iter().map(|p| p.notify_resource_list_changed())).await;
        self.prune_after(&results).await;
    }

    /// Remove peers whose last notify failed. Conservative: only prunes when at least one failed.
    /// Phase 1 approximation — rmcp 1.1.1 does not expose stable peer identity, so we use a
    /// length-based snapshot approach: if #failures == #peers, we drop all; otherwise we
    /// tolerate stale peers until next broadcast (they'll error again and eventually die on
    /// transport close).
    async fn prune_after(&self, results: &[Result<(), ServiceError>]) {
        if results.is_empty() || results.iter().all(Result::is_ok) {
            return;
        }
        // If ALL failed, clear (mass death = likely server shutting down).
        if results.iter().all(Result::is_err) {
            self.peers.write().await.clear();
        }
        // Otherwise: leave stale peers; TODO(phase2): prune individually once rmcp exposes peer identity.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_hub_is_empty() {
        let hub = UpstreamSessionHub::new();
        hub.notify_tools_changed().await;
        hub.notify_prompts_changed().await;
        hub.notify_resources_changed().await;
        // no panic on empty broadcast
    }
}
