//! MCP extension registry — tracks downstream mod MCP servers and proxies them to the engine's single `/mcp` endpoint.

use rmcp::{
    model::{
        CallToolResult, GetPromptResult, Prompt, ReadResourceResult, Resource, ResourceTemplate,
        ServerCapabilities, Tool,
    },
    service::{RoleClient, RunningService, ServiceError},
};
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};

pub mod handler;
pub use handler::DownstreamClientHandler;

/// Axum state wrapper for the registry.
#[derive(Clone)]
pub struct SharedMcpExtensionRegistry(pub Arc<RwLock<McpExtensionRegistry>>);

pub struct McpExtensionRegistry {
    clients: HashMap<String, DownstreamClient>,
    upstream_hub: Arc<crate::upstream_hub::UpstreamSessionHub>,
    invalidator_tx: mpsc::UnboundedSender<CacheInvalidation>,
    invalidator_task: tokio::task::JoinHandle<()>,
}

pub struct DownstreamClient {
    pub mod_slug: String,
    pub mod_name: String,
    pub mcp_url: String,
    pub service: RunningService<RoleClient, DownstreamClientHandler>,
    pub cached: RwLock<CapabilitiesCache>,
    pub capabilities: ServerCapabilities,
}

#[derive(Default)]
pub struct CapabilitiesCache {
    pub tools: Vec<Tool>,
    pub prompts: Vec<Prompt>,
    pub resources: Vec<Resource>,
    pub resource_templates: Vec<ResourceTemplate>,
}

#[derive(Debug, Clone)]
pub enum CacheInvalidation {
    Tools(String),
    Prompts(String),
    Resources(String),
}

impl CacheInvalidation {
    pub fn mod_slug(&self) -> &str {
        match self {
            CacheInvalidation::Tools(s)
            | CacheInvalidation::Prompts(s)
            | CacheInvalidation::Resources(s) => s,
        }
    }
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("mcpUrl unreachable or initialize failed: {0}")]
    Initialize(#[from] ServiceError),
    #[error("peer info missing after initialize")]
    MissingPeerInfo,
    #[error("reserved URI scheme '{0}' not allowed for mod resources")]
    ReservedScheme(String),
    #[error("invalid mod slug: must match ^[a-z][a-z0-9_]*$")]
    InvalidSlug,
}

#[derive(Error, Debug)]
pub enum DownstreamError {
    #[error("unknown mod slug: {0}")]
    UnknownSlug(String),
    #[error("downstream service error: {0}")]
    ServiceError(#[from] ServiceError),
}

/// Args passed to `McpExtensionRegistry::add()`.
pub struct RegisterArgs {
    pub mod_slug: String,
    pub mod_name: String,
    pub mcp_url: String,
}

impl McpExtensionRegistry {
    pub fn new(
        upstream_hub: Arc<crate::upstream_hub::UpstreamSessionHub>,
    ) -> SharedMcpExtensionRegistry {
        let (tx, mut rx) = mpsc::unbounded_channel::<CacheInvalidation>();

        // Create the registry with a placeholder task.
        let shared = SharedMcpExtensionRegistry(Arc::new(RwLock::new(Self {
            clients: HashMap::new(),
            upstream_hub,
            invalidator_tx: tx,
            invalidator_task: tokio::spawn(async {}), // placeholder, replaced below
        })));

        // Spawn real receiver task that holds a handle back to the registry.
        let shared_for_task = shared.clone();
        let task = tokio::spawn(async move {
            while let Some(inv) = rx.recv().await {
                let reg = shared_for_task.0.read().await;
                if let Some(client) = reg.clients.get(inv.mod_slug()) {
                    Self::refresh_cache_for(client, &inv).await;
                }
            }
        });

        // Replace placeholder with real task. Safe: registry just created, no other holders.
        {
            let mut guard = shared.0.blocking_write();
            let placeholder = std::mem::replace(&mut guard.invalidator_task, task);
            placeholder.abort();
        }

        shared
    }

    async fn refresh_cache_for(client: &DownstreamClient, inv: &CacheInvalidation) {
        match inv {
            CacheInvalidation::Tools(_) => {
                if client.capabilities.tools.is_some() {
                    let new = client.service.list_all_tools().await.unwrap_or_default();
                    client.cached.write().await.tools = new;
                }
            }
            CacheInvalidation::Prompts(_) => {
                if client.capabilities.prompts.is_some() {
                    let new = client.service.list_all_prompts().await.unwrap_or_default();
                    client.cached.write().await.prompts = new;
                }
            }
            CacheInvalidation::Resources(_) => {
                if client.capabilities.resources.is_some() {
                    let new_res = client.service.list_all_resources().await.unwrap_or_default();
                    let new_tpl = client
                        .service
                        .list_all_resource_templates()
                        .await
                        .unwrap_or_default();
                    let mut cache = client.cached.write().await;
                    cache.resources = new_res;
                    cache.resource_templates = new_tpl;
                }
            }
        }
    }
}

impl Drop for McpExtensionRegistry {
    fn drop(&mut self) {
        self.invalidator_task.abort();
    }
}
