//! MCP extension registry — tracks downstream mod MCP servers and proxies them to the engine's single `/mcp` endpoint.

#[allow(unused_imports)]
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
