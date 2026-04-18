//! MCP extension registry — tracks downstream mod MCP servers and proxies them to the engine's single `/mcp` endpoint.

use rmcp::{
    model::{
        CallToolResult, GetPromptResult, Prompt, ReadResourceResult, Resource, ResourceTemplate,
        ServerCapabilities, Tool,
    },
    service::{ClientInitializeError, RoleClient, RunningService, ServiceError},
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
    Initialize(#[from] ClientInitializeError),
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

/// Summary of a registered downstream MCP client returned by [`McpExtensionRegistry::list_registrations`].
#[derive(serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationInfo {
    pub mod_name: String,
    pub mod_slug: String,
    pub mcp_url: String,
    pub tool_count: usize,
    pub prompt_count: usize,
    pub resource_count: usize,
}

/// Returns `true` if `s` matches the pattern `^[a-z][a-z0-9_]*$`.
fn is_valid_slug(s: &str) -> bool {
    let mut chars = s.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

impl McpExtensionRegistry {
    /// Register a new downstream MCP server.
    ///
    /// If a registration already exists for the given slug, it is cancelled and replaced
    /// (upsert semantics). On success, upstream clients are notified of list changes.
    pub async fn add(&mut self, req: RegisterArgs) -> Result<(), RegisterError> {
        if !is_valid_slug(&req.mod_slug) {
            return Err(RegisterError::InvalidSlug);
        }

        // Upsert: cancel existing before replacing.
        if let Some(existing) = self.clients.remove(&req.mod_slug) {
            let _ = existing.service.cancel().await;
        }

        let transport =
            rmcp::transport::StreamableHttpClientTransport::from_uri(req.mcp_url.clone());
        let handler = DownstreamClientHandler::new(
            req.mod_slug.clone(),
            self.upstream_hub.clone(),
            self.invalidator_tx.clone(),
        );
        let service = rmcp::service::serve_client(handler, transport).await?;

        let capabilities = service
            .peer_info()
            .ok_or(RegisterError::MissingPeerInfo)?
            .capabilities
            .clone();

        // Pre-warm caches. Errors leave empty lists rather than failing registration.
        let tools = if capabilities.tools.is_some() {
            service.list_all_tools().await.unwrap_or_default()
        } else {
            Vec::new()
        };
        let prompts = if capabilities.prompts.is_some() {
            service.list_all_prompts().await.unwrap_or_default()
        } else {
            Vec::new()
        };
        let (resources, resource_templates) = if capabilities.resources.is_some() {
            let r = service.list_all_resources().await.unwrap_or_default();
            let t = service.list_all_resource_templates().await.unwrap_or_default();
            (r, t)
        } else {
            (Vec::new(), Vec::new())
        };

        // Reserved scheme check: homunculus:// is for built-in resources only.
        for res in &resources {
            if res.uri.starts_with("homunculus://") {
                return Err(RegisterError::ReservedScheme("homunculus".into()));
            }
        }

        let client = DownstreamClient {
            mod_slug: req.mod_slug.clone(),
            mod_name: req.mod_name,
            mcp_url: req.mcp_url,
            service,
            cached: RwLock::new(CapabilitiesCache {
                tools,
                prompts,
                resources,
                resource_templates,
            }),
            capabilities,
        };
        self.clients.insert(req.mod_slug.clone(), client);

        // Broadcast list_changed to all upstream clients.
        self.upstream_hub.notify_tools_changed().await;
        self.upstream_hub.notify_prompts_changed().await;
        self.upstream_hub.notify_resources_changed().await;

        Ok(())
    }

    /// Remove a downstream registration by slug, cancelling its service if present.
    pub async fn remove(&mut self, mod_slug: &str) {
        if let Some(client) = self.clients.remove(mod_slug) {
            let _ = client.service.cancel().await;
            self.upstream_hub.notify_tools_changed().await;
            self.upstream_hub.notify_prompts_changed().await;
            self.upstream_hub.notify_resources_changed().await;
        }
    }

    /// Returns `true` if a registration with the given slug exists.
    pub fn has_slug(&self, slug: &str) -> bool {
        self.clients.contains_key(slug)
    }

    /// Returns a snapshot of all current registrations with their cached capability counts.
    pub async fn list_registrations(&self) -> Vec<RegistrationInfo> {
        let mut out = Vec::with_capacity(self.clients.len());
        for c in self.clients.values() {
            let cache = c.cached.read().await;
            out.push(RegistrationInfo {
                mod_name: c.mod_name.clone(),
                mod_slug: c.mod_slug.clone(),
                mcp_url: c.mcp_url.clone(),
                tool_count: cache.tools.len(),
                prompt_count: cache.prompts.len(),
                resource_count: cache.resources.len(),
            });
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_accepts_lowercase_alnum_underscore() {
        assert!(is_valid_slug("voicevox"));
        assert!(is_valid_slug("my_mod"));
        assert!(is_valid_slug("mod1_2"));
    }

    #[test]
    fn slug_rejects_invalid() {
        assert!(!is_valid_slug(""));
        assert!(!is_valid_slug("_foo"));
        assert!(!is_valid_slug("1foo"));
        assert!(!is_valid_slug("Foo"));
        assert!(!is_valid_slug("foo-bar"));
        assert!(!is_valid_slug("foo.bar"));
    }
}
